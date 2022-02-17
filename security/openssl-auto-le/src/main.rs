use std::{fs, time::Duration};

use acme_micro::{create_p384_key, Certificate, Directory, DirectoryUrl};
use actix_files::Files;
use actix_web::{rt, web, App, HttpRequest, HttpServer, Responder};
use anyhow::anyhow;
use openssl::{
    pkey::PKey,
    ssl::{SslAcceptor, SslMethod},
    x509::X509,
};

pub async fn gen_tls_cert(
    user_email: &str,
    user_domain: &str,
) -> anyhow::Result<Certificate> {
    // Create acme-challenge dir.
    fs::create_dir("./acme-challenge").unwrap();

    let domain = user_domain.to_string();

    // Create temporary Actix Web server for ACME challenge.
    let srv = HttpServer::new(|| {
        App::new().service(
            Files::new(
                // HTTP route
                "/.well-known/acme-challenge",
                // Server's dir
                "acme-challenge",
            )
            .show_files_listing(),
        )
    })
    .bind((domain, 80))?
    .shutdown_timeout(0)
    .run();

    let srv_handle = srv.handle();
    let srv_task = rt::spawn(srv);

    // Use DirectoryUrl::LetsEncryptStaging for dev/testing.
    let url = DirectoryUrl::LetsEncrypt;

    // Create a directory entrypoint.
    let dir = Directory::from_url(url)?;

    // Our contact addresses; note the `mailto:`
    let user_email_mailto: String = "mailto:{email}".replace("{email}", user_email);
    let contact = vec![user_email_mailto];

    // Generate a private key and register an account with our ACME provider.
    // We should write it to disk any use `load_account` afterwards.
    let acc = dir.register_account(contact.clone())?;

    // Load an account from string
    let privkey = acc.acme_private_key_pem()?;
    let acc = dir.load_account(&privkey, contact)?;

    // Order a new TLS certificate for the domain.
    let mut ord_new = acc.new_order(user_domain, &[])?;

    // If the ownership of the domain have already been
    // authorized in a previous order, we might be able to
    // skip validation. The ACME API provider decides.
    let ord_csr = loop {
        // Are we done?
        if let Some(ord_csr) = ord_new.confirm_validations() {
            break ord_csr;
        }

        // Get the possible authorizations (for a single domain
        // this will only be one element).
        let auths = ord_new.authorizations()?;

        // For HTTP, the challenge is a text file that needs to
        // be placed in our web server's root:
        //
        // <mydomain>/acme-challenge/<token>
        //
        // The important thing is that it's accessible over the
        // web for the domain we are trying to get a
        // certificate for:
        //
        // http://mydomain.io/.well-known/acme-challenge/<token>
        let chall = auths[0]
            .http_challenge()
            .ok_or(anyhow!("no HTTP challenge accessible"))?;

        // The token is the filename.
        let token = chall.http_token();

        // The proof is the contents of the file
        let proof = chall.http_proof()?;

        // Place the file/contents in the correct place.
        let path = format!("acme-challenge/{}", token);
        fs::write(&path, &proof)?;

        // After the file is accessible from the web, the calls
        // this to tell the ACME API to start checking the
        // existence of the proof.
        //
        // The order at ACME will change status to either
        // confirm ownership of the domain, or fail due to the
        // not finding the proof. To see the change, we poll
        // the API with 5000 milliseconds wait between.
        chall.validate(Duration::from_millis(5000))?;

        // Update the state against the ACME API.
        ord_new.refresh()?;
    };

    // Ownership is proven. Create a private key for
    // the certificate. These are provided for convenience; we
    // could provide our own keypair instead if we want.
    let pkey_pri = create_p384_key()?;

    // Submit the CSR. This causes the ACME provider to enter a
    // state of "processing" that must be polled until the
    // certificate is either issued or rejected. Again we poll
    // for the status change.
    let ord_cert = ord_csr.finalize_pkey(pkey_pri, Duration::from_millis(5000))?;

    // Now download the certificate. Also stores the cert in
    // the persistence.
    let cert = ord_cert.download_cert()?;

    // Stop temporary server for ACME challenge
    srv_handle.stop(true).await;
    srv_task.await??;

    // Delete acme-challenge dir
    fs::remove_dir_all("./acme-challenge")?;

    Ok(cert)
}

// "Hello world" example
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // IMPORTANT: Use your own email and domain!
    let email = "example@example.com";
    let domain = "mydomain.io";

    //   Load keys
    // ==============================================
    // = IMPORTANT:                                 =
    // = This process has to be repeated            =
    // = before the certificate expires (< 90 days) =
    // ==============================================
    // Obtain TLS certificate
    let cert = gen_tls_cert(email, domain).await?;
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

    // Get and add private key
    let pkey_der = PKey::private_key_from_der(&cert.private_key_der()?)?;
    ssl_builder.set_private_key(&pkey_der)?;

    // Get and add certificate
    let cert_der = X509::from_der(&cert.certificate_der()?)?;
    ssl_builder.set_certificate(&cert_der)?;

    // Get and add intermediate certificate to the chain
    let icert_url = "https://letsencrypt.org/certs/lets-encrypt-r3.der";
    let icert_bytes = reqwest::get(icert_url).await?.bytes().await?;
    let intermediate_cert = X509::from_der(&icert_bytes)?;
    ssl_builder.add_extra_chain_cert(intermediate_cert)?;

    // NOTE:
    // Storing pkey_der, cert_der and intermediate_cert somewhere
    // (in order to avoid unnecessarily regeneration of TLS/SSL) is recommended

    log::info!("starting HTTP server at http://localhost:443");

    // Start HTTP server!
    let srv = HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind_openssl((domain, 443), ssl_builder)?
        .run();

    let srv_handle = srv.handle();

    let _auto_shutdown_task = rt::spawn(async move {
        // Shutdown server every 4 weeks so that TLS certs can be regenerated if needed.
        // This is only appropriate in contexts like Kubernetes which can orchestrate restarts.
        rt::time::sleep(Duration::from_secs(60 * 60 * 24 * 28)).await;
        srv_handle.stop(true).await;
    });

    srv.await?;

    Ok(())
}

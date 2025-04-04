use std::time::Duration;

use acme::{Certificate, Directory, DirectoryUrl, create_p256_key};
use actix_files::Files;
use actix_web::{App, HttpRequest, HttpServer, Responder, rt, web};
use eyre::eyre;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, pem::PemObject};
use tokio::fs;

const CHALLENGE_DIR: &str = "./acme-challenges";
const DOMAIN_NAME: &str = "example.org";
const CONTACT_EMAIL: &str = "contact@example.org";

pub async fn gen_tls_cert(user_domain: &str, contact_email: &str) -> eyre::Result<Certificate> {
    // Create acme-challenge dir.
    fs::create_dir(CHALLENGE_DIR).await?;

    let domain = user_domain.to_owned();

    // Create temporary Actix Web server for ACME challenge.
    let srv = HttpServer::new(|| {
        App::new().service(
            Files::new("/.well-known/acme-challenge", "acme-challenge").show_files_listing(),
        )
    })
    .bind((domain, 80))?
    .shutdown_timeout(0)
    .disable_signals()
    .run();

    let srv_handle = srv.handle();
    let srv_task = rt::spawn(srv);

    // Use DirectoryUrl::LetsEncryptStaging for dev/testing.
    let url = DirectoryUrl::LetsEncrypt;

    // Create a directory entrypoint.
    let dir = Directory::fetch(url).await?;

    // Our contact addresses; note the `mailto:`
    let user_email_mailto = format!("mailto:{contact_email}");
    let contact = vec![user_email_mailto];

    // Generate a private key and register an account with our ACME provider.
    // We should write it to disk any use `load_account` afterwards.
    let acc = dir.register_account(Some(contact.clone())).await?;

    // Load an account from string
    let priv_key = acc.acme_private_key_pem()?;
    let acc = dir.load_account(&priv_key, Some(contact)).await?;

    // Order a new TLS certificate for the domain.
    let mut ord_new = acc.new_order(user_domain, &[]).await?;

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
        let auths = ord_new.authorizations().await?;

        // For HTTP, the challenge is a text file that needs to be placed so it
        // is accessible to our web server:
        //
        // ./acme-challenge/<token>
        //
        // The important thing is that it's accessible over the
        // web for the domain we are trying to get a
        // certificate for:
        //
        // http://example.org/.well-known/acme-challenge/<token>
        let challenge = auths[0]
            .http_challenge()
            .ok_or_else(|| eyre!("no HTTP challenge accessible"))?;

        // The token is the filename.
        let token = challenge.http_token();

        // The proof is the contents of the file
        let proof = challenge.http_proof()?;

        // Place the file/contents in the correct place.
        let path = format!("acme-challenge/{token}");
        fs::write(&path, &proof).await?;

        // After the file is accessible from the web, the calls
        // this to tell the ACME API to start checking the
        // existence of the proof.
        //
        // The order at ACME will change status to either
        // confirm ownership of the domain, or fail due to the
        // not finding the proof. To see the change, we poll
        // the API with 5000 milliseconds wait between.
        challenge.validate(Duration::from_millis(5000)).await?;

        // Update the state against the ACME API.
        ord_new.refresh().await?;
    };

    // Ownership is proven. Create a private key for
    // the certificate. These are provided for convenience; we
    // could provide our own keypair instead if we want.
    let signing_key = create_p256_key();

    // Submit the CSR. This causes the ACME provider to enter a
    // state of "processing" that must be polled until the
    // certificate is either issued or rejected. Again we poll
    // for the status change.
    let ord_cert = ord_csr
        .finalize(signing_key, Duration::from_secs(5))
        .await?;

    // Now download the certificate. Also stores the cert in
    // the persistence.
    let cert = ord_cert.download_cert().await?;

    // Stop temporary server for ACME challenge
    srv_handle.stop(true).await;
    srv_task.await??;

    // Delete acme-challenge dir
    fs::remove_dir_all(CHALLENGE_DIR).await?;

    Ok(cert)
}

// "Hello world" example
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    color_eyre::install()?;

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    //   Load keys
    // ==============================================
    // = IMPORTANT:                                 =
    // = This process has to be repeated            =
    // = before the certificate expires (< 90 days) =
    // ==============================================
    // Obtain TLS certificate
    //
    // NOTE: Persisting the private key and certificate chain somewhere is
    // recommended in order to avoid unnecessarily regenerating of TLS certs.
    let cert = gen_tls_cert(DOMAIN_NAME, CONTACT_EMAIL).await?;

    let rustls_config = load_rustls_config(cert)?;

    log::info!("starting HTTP server at https://{DOMAIN_NAME}:443");

    // Start HTTP server!
    let srv = HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind_rustls_0_23(("0.0.0.0", 443), rustls_config)?
        .run();

    let srv_handle = srv.handle();

    let _auto_shutdown_task = rt::spawn(async move {
        // Shutdown server every 4 weeks so that TLS certs can be regenerated if
        // needed. This is only appropriate in contexts like Kubernetes which
        // can orchestrate restarts.
        rt::time::sleep(Duration::from_secs(60 * 60 * 24 * 28)).await;
        srv_handle.stop(true).await;
    });

    srv.await?;

    Ok(())
}

fn load_rustls_config(cert: Certificate) -> eyre::Result<rustls::ServerConfig> {
    // init server config builder with safe defaults
    let config = rustls::ServerConfig::builder().with_no_client_auth();

    // convert ACME-obtained private key
    let private_key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(cert.private_key_der()?));

    // convert ACME-obtained certificate chain
    let cert_chain = CertificateDer::pem_slice_iter(cert.certificate().as_bytes())
        .flatten()
        .collect();

    Ok(config.with_single_cert(cert_chain, private_key)?)
}

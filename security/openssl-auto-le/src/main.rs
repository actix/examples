use actix_web::{rt::System, web, App, HttpRequest, HttpServer, Responder};
use actix_files::Files;
use openssl::{
    ssl::{SslAcceptor, SslMethod},
    pkey::PKey,
    x509::X509,
};
use std::sync::mpsc;
use std::thread;
use acme_micro::{Error, Certificate, Directory, DirectoryUrl};
use acme_micro::create_p384_key;
use std::time::Duration;
use futures::executor;
use std::fs;


pub fn gen_tls_cert(
    user_email: &str,
    user_domain: &str,
) -> Result<Certificate, Error> {

    // Create acme-challenge dir
    fs::create_dir("./acme-challenge").unwrap();

    // Create temporary Actix server for ACME challenge
    let (tx, rx) = mpsc::channel();
    let domain = user_domain.to_string();
    thread::spawn(move || {
        let sys = System::new("http-server");

        let srv = HttpServer::new(|| {
            App::new()
                .service(
                    Files::new(
                        // HTTP route
                        "/.well-known/acme-challenge",
                        // Server's dir
                        "acme-challenge",
                    ).show_files_listing()
                )
        })
        .bind((domain, 80))?
        .shutdown_timeout(0)  // seconds to shutdown after stop signal: 0
        .run();

        let _ = tx.send(srv);
        sys.run()
    });
    let srv = rx.recv().unwrap();

    // Use DirectoryUrl::LetsEncrypStaging for dev/testing.
    let url = DirectoryUrl::LetsEncrypt;

    // Create a directory entrypoint.
    let dir = Directory::from_url(url)?;

    // Our contact addresses; note the `mailto:`
    let user_email_mailto: String = "mailto:{email}"
        .replace("{email}", user_email);
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
        let chall = auths[0].http_challenge().unwrap();

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
    let ord_cert = ord_csr
        .finalize_pkey(pkey_pri, Duration::from_millis(5000))?;

    // Now download the certificate. Also stores the cert in
    // the persistence.
    let cert = ord_cert.download_cert()?;

    // Stop temporary server for ACME challenge
    executor::block_on(srv.stop(true));

    // Delete acme-challenge dir
    fs::remove_dir_all("./acme-challenge").unwrap();

    Ok(cert)

}


// "Hello world" example
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello world!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // IMPORTANT: Use your own email and domain!
    let email = "example@example.com";
    let domain = "mydomain.io";

    //                 Load SSL keys
    // ==============================================
    // = IMPORTANT:                                 =
    // = This process has to be repeated            =
    // = before the certificate expires (< 90 days) =
    // ==============================================
    // Obtain TLS/SSL certificate
    let cert = gen_tls_cert(email, domain).unwrap();
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .unwrap();
    // Get and add private key
    let pkey_der = PKey::private_key_from_der(
        &cert.private_key_der().unwrap()
    ).unwrap();
    ssl_builder.set_private_key(&pkey_der).unwrap();
    // Get and add certificate
    let cert_der = X509::from_der(
        &cert.certificate_der().unwrap()
    ).unwrap();
    ssl_builder.set_certificate(&cert_der).unwrap();
    // Get and add intermediate certificate to the chain
    let icert_url =
        "https://letsencrypt.org/certs/lets-encrypt-r3.der";
    let icert_bytes = reqwest::blocking::get(icert_url)
        .unwrap().bytes().unwrap();
    let intermediate_cert = X509::from_der(&icert_bytes).unwrap();
    ssl_builder.add_extra_chain_cert(intermediate_cert).unwrap();
    // NOTE:
    // Storing pkey_der, cert_der and intermediate_cert somewhere
    // (in order to avoid unnecessarily regeneration of TLS/SSL) is recommended

    // Start server!
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind_openssl((domain, 443), ssl_builder)?
        .run()
        .await
}

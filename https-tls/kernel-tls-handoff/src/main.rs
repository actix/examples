//! Demonstrates Linux kernel TLS with a Rustls handshake and an Actix HTTP service.

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
fn main() -> std::io::Result<()> {
    linux::main()
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("This example requires Linux.");
}

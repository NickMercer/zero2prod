use std::net::TcpListener;

use zero2prod::{configuration, startup};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = configuration::get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))
        .expect("Failed to bind random port");
    startup::run(listener)?.await
}

use std::net::TcpListener;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::{configuration, startup, telemetry};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    
    let config = configuration::get_configuration().expect("Failed to read configuration.");
    
    let connection_pool = PgPool::connect_lazy_with(config.database.with_db());

    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address)
        .expect("Failed to bind random port");

    startup::run(listener, connection_pool)?.await
}
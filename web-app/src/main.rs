use std::net::TcpListener;
use web_app::configuration::get_configuration;
use web_app::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().unwrap();

    println!("Watchman web application is running on http://127.0.0.1:8000");
    run(TcpListener::bind("127.0.0.1:8000")?, config).await
}

use std::fs;
use std::fs::FileType;
use std::net::TcpListener;
use web_app::configuration::get_configuration;
use web_app::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().unwrap();

    // let mut interval_timer = tokio::time::interval(chrono::Duration::seconds(10).to_std().unwrap());
    // loop {
    //     interval_timer.tick().await;
    //
    //     tokio::spawn(async move {
    //         println!("hey");
    //     });
    // }
    //
    run(TcpListener::bind("127.0.0.1:8000")?, config).await
}

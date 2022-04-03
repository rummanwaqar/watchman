use std::fs;
use std::fs::FileType;
use std::net::TcpListener;
use web_app::configuration::get_configuration;
use web_app::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().unwrap();

    // for entry in fs::read_dir(config.data_directory)? {
    //     println!("{:?}", entry);
    //     if let Ok(entry) = entry {
    //         println!("{:?}", entry.path().as_path().extension().unwrap());
    //         println!("{:?}", entry.metadata()?.created().unwrap());
    //     }
    // }
    //
    // Ok(())

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

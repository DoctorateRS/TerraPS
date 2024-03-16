mod config;
mod constants;
mod errors;
mod forms;
mod init;
mod routes;
mod utils;
use forms::Config;

#[tokio::main]
async fn main() {
    init::init().await;
    let config = forms::Config::try_load().unwrap();
    print!("{:#?}", config);
}

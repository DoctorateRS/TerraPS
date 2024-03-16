mod config;
mod constants;
mod errors;
mod forms;
mod init;
mod routes;
mod utils;
use errors::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init::init().await;
    let mut config = forms::Config::try_load().unwrap();
    print!("{:#?}", config);
    config.user_config.theme = "tm_rainbowsix_1".to_string();
    config.user_config.background = "bg_rainbowsix_1".to_string();
    config.user_config.secretary = "char_4123_ela".to_string();
    config.user_config.secretary_skin_id = "char_4123_ela@rainbow6#2".to_string();
    config.try_save().unwrap();
    Ok(())
}

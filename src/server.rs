mod constants;
mod core;
mod debug;
mod game;
mod routes;
mod utils;

use axum::serve;
use tokio::net::TcpListener as Listener;
use tracing::{info, Level};
use tracing_subscriber::fmt as subscriber_fmt;
use utils::json::read_json;

#[tokio::main]
async fn main() {
    // TRACING
    subscriber_fmt().with_max_level(Level::DEBUG).init();

    println!(
        r#"
           DNSTC (Do Not Sell This Crap) Public License
        Copyright (c) Every-fucking-one, except the Author

Everyone is permitted to copy, distribute, modify, merge, publish,
sublicense or whatever the fuck they want, except selling or making
any form of financial profits, with this software but at their OWN 
RISK.

                         Preamble

The author has absolutely no fucking clue what the code in this project
does. It might just fucking work or not, there is no third option.


            DO NOT SELL THIS CRAP PUBLIC LICENSE
TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION, AND MODIFICATION

0. You can DO WHATEVER THE FUCK YOU WANT TO, except for selling or making
any form of financial profits, as long as you NEVER LEAVE A FUCKING TRACE
TO TRACK THE AUTHOR of the original product to blame for or held responsible.

IN NO EVENT SHALL THE AUTHORS BE HELD LIABLE FOR ANY CLAIM, DAMAGES OR 
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR 
OTHER DEALINGS IN THE SOFTWARE.

Good luck and Godspeed.
"#
    );

    // SERVER
    let server_address = &get_server_address();
    let listener = match Listener::bind(server_address).await {
        Ok(listener) => listener,
        Err(e) => {
            panic!("Failed to bind to address: {}", e);
        }
    };
    info!("Server started at: {}", server_address);
    match serve(listener, routes::routes()).await {
        Ok(_) => (),
        Err(e) => {
            panic!("Failed to start server: {}", e);
        }
    };
}

fn get_server_address() -> String {
    let config = read_json(constants::config::CONFIG_JSON_PATH);
    let server_config = &config["server"];
    let host = server_config["host"].as_str().unwrap();
    let port = server_config["port"].as_u64().unwrap();
    format!("{}:{}", host, port)
}

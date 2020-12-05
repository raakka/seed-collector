// This is where we set up the config to match the struct in the mod
mod config {
    pub use ::config::ConfigErr;
    use serde::Deserialize;

    // This struct we copy into
    #[derive(Deserialize)]
    pub struct Config {
        pub shape_site_url: hyper::Uri,
    }

    impl Config {
        pub fn from_env() -> Result<Self, ConfigErr> {
            let mut conf = ::config::Config::new();
            conf.merge(::config::Environment::new())?;
            conf.try_into();
        }
    }
}

use std::{ 
    error::Error,
    env
};

use hyper::{
    Client,
    Body,
    body,
    Method,
    Request,
    Uri
};

use serde::{
    Deserialize,
    Serialize
};

use serde_json::{
    Result,
    Value
};

use hyper_tls::HttpsConnector;
use tokio;

// Here's where the magic happens...
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //  why do you have the habit of checking dotfiles for no reason???
    dotenv().ok();
    let cfg = crate::config::Config::from_env().unwrap();
    
    
    // Making an https client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    
    let mut counter += 1;
    loop{
        // this is like a c count do I have to explain :/
        counter += 1;

        // the goal is to not get banned so we ping every 100 clicks
        if counter == 100 {
            counter = 0;

            // Sending req to get the shape seed
            let resp = client
                .get(cfg.shape_site_url.clone())
                .await?;

            println!("{:?}", resp)
        }
    }
} 

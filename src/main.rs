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
use dotenv::dotenv;

#[tokio::main]
async fn main() -> std::result::Result<
    (), // this kinda hard to read mb
    Box<dyn std::error::Error + Send + Sync>
    > {
    //  why do you have the habit of checking dotfiles for no reason???
    dotenv().ok();
    
    // Making an https client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    // let target_uri: hyper::Uri = env::var("shape_site_url").unwrap().parse()?;  
    let target_uri: hyper::Uri = "https://assets.targetimg1.com/ssx/ssx.mod.js".parse()?;
        
    let mut counter: u8 = 0;
    loop{
        // this is like a c count do I have to explain :/
        counter += 1;

        // the goal is to not get banned so we ping every 100 clicks
        if counter == 100 {
            counter = 0;

            // Sending req to get the shape seed
            let resp = client
                .get(target_uri.clone())
                .await?;

            println!("{:?}", resp)
        }
    }
} 

use std::{ 
    error::Error,
    env,
    fs,
    path::Path
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

// Seed json object 
#[derive(Serialize, Deserialize)]
struct Seedobj {
    seed: String,
    v: String,
    i: String,
}

#[tokio::main]
async fn main() -> std::result::Result<
    (), /* this kinda hard to read mb */
    Box<dyn std::error::Error + Send + Sync>
    > {

    //  why do you have the habit of checking dotfiles for no reason???
    dotenv().ok();
    
///////////////////////////////////////////////////////////////////////////////////////////////////
// FILE HANDLING 

    // TODO(XVI): Make this a mod this is a mess
    // Checking if we have our JSON output file made...
    let json_seed_file = fs::File::open("seed-json/greenhouse.json");
    let _ = match json_seed_file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => match fs::File::create("seed-json/greenhouse.json") {
                Ok(fc) => fc,
                Err(e) => panic!("Could not create greenhouse.json: {:?}", e),
                _ => panic!("Other error creating greenhouse.json!"),
            },
            _ => panic!("Error I don't know how to handle"),
        },
    };
    
    // Does it read?
    let mut seed_data = fs::read_to_string("seed-json/greenhouse.json")
        .expect("Can't read greenhouse.json");

    let mut shape_seeds: Vec<Seedobj> = Vec::new();
    if fs::metadata("seed-json/greenhouse.json").unwrap().len() != 0 {
        shape_seeds = serde_json::from_str(&seed_data)?;
    }
    // OKAY! After all that we can now shape_seeds.push(<Seedobj>)

///////////////////////////////////////////////////////////////////////////////////////////////////
// REQUESTS !

    // Making an https client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    // let target_uri: hyper::Uri = env::var("shape_site_url").unwrap().parse()?;  
    let target_uri: hyper::Uri = "https://assets.targetimg1.com/ssx/ssx.mod.js".parse()?;
        
    // Monitor Loop
    let mut counter: u8 = 0;
    loop{
        // this is like a c count do I have to explain :/
        counter += 1;

        if counter != 254 {
            continue;
        }

        // the goal is to not get banned so we ping every 254 clicks
        // also I'm too lazy to change this from u8 to something else lol
        if counter == 254 {
            counter = 0;
            
            // update the file from the last read
            seed_data = fs::read_to_string("seed-json/greenhouse.json")
                .expect("Can't read greenhouse.json");
            if fs::metadata("seed-json/greenhouse.json").unwrap().len() != 0 {
                shape_seeds = serde_json::from_str(&seed_data)?;
            }

            // Sending req to get the shape seed
            let resp = client
                .get(target_uri.clone())
                .await?;

            // since we need to convert the body from stream -> str
            let byte_bod = body::to_bytes(resp.into_body()).await?;
            let body = String::from_utf8(byte_bod.to_vec())
                .expect("resp body not utf8");
            
            // Parsing body for our Json values
            let found_values: Vec<_> = body.split('"').collect();
            
            let this_seed = Seedobj {
                seed: found_values[1].to_string(),
                v: found_values[3].to_string(),
                i: found_values[5].to_string()
            };
            
            // okay okay I know this looks like garbage but whatever
            // this checks to see if we have the same seed in our json
            for seed in shape_seeds.into_iter() {
                if seed.seed == this_seed.seed {
                    continue;
                } else {
                    //  pushing our new object to this array
                    shape_seeds.push(this_seed);
            
                    // save to json
                    let njson_seeds: String = serde_json::to_string(&shape_seeds)?;
                    fs::write("seed-json/greenhouse.json", &njson_seeds)
                        .expect("Failed to write to file!");
                }
            }
            // JSON body
            // let json_body: Value = serde_json::from_str(&body)?;
            // we don't need this beause the body is JS not JSON stupid...

            println!("{:?}", body)
        }
    }
} 

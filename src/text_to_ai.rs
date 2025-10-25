use reqwest;
use serde::{Serialize,Deserialize};
use serde_json::Value;
use std::error::Error;
use dotenv::dotenv;
use std::env;

use reqwest::{Client,header::HeaderMap, Url};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};


#[derive(Serialize)]
struct Message {
    model: String,
    input: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    
    // OPENAI_API_KEY load 
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let api_key_str : String = api_key.parse().expect("Not able to parse API_KEY.");

    // Which model to use
    let model = env::var("MODEL").expect("MODEL not set");
    let model_str : String = model.parse().expect("Not able to parse MODEL.");

    // create Client, responsible for managing HTTP connections
    let client = Client::new();     

    // create headers
    let mut headers: HeaderMap = HeaderMap::new();
    let header_string = format!("Bearer {}", api_key_str).parse::<String>().unwrap();
    let header_value = HeaderValue::from_str(&header_string).unwrap();
    headers.insert(AUTHORIZATION, header_value);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Doing Request to AI Model 
    let request_body_string = &Message {
        model: model_str,
        input: "Tell me an sleep story about an unicorn".to_string(),
    };

    let url = Url::parse(&format!("{}", "https://api.openai.com/v1/responses")).unwrap();
    let response = client
            .post(url)
            .headers(headers.clone())
            .body(serde_json::to_string_pretty(request_body_string).unwrap())
            .send().await.unwrap();

    let parsed_response: Value = match serde_json::from_str(&response.text().await.unwrap()) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
            return Err(err.into());
        } 
    };

    println!("Output:\n{}", parsed_response["output"]);

    Ok(())
}


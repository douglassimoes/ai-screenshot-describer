mod alternate_text_window;

use eframe::egui::debug_text::print;
use reqwest::{self, Request};
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use std::error::Error;
use dotenv::dotenv;
use std::{fs,env};

use base64::{engine::general_purpose, Engine as _};

use reqwest::{Client,header::HeaderMap, Url};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {   
    // OPENAI_API_KEY load 
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let api_key_str : String = api_key.parse().expect("Not able to parse API_KEY.");

    // Which model to use
    let model = env::var("IMG_MODEL").expect("IMG_MODEL not set");
    let model_str : String = model.parse().expect("Not able to parse IMG_MODEL.");

    // Loads the prompt
    let img_prompt = env::var("IMG_PROMPT").expect("IMG_PROMPT not set");
    let img_prompt_str : String = img_prompt.parse().expect("Not able to parse IMG_PROMPT.");

    // Reads the image file from screenshot
    let image_bytes = fs::read("screenshot.png")?;

    // Encodes the image in base64
    let image_b64 = general_purpose::STANDARD.encode(image_bytes);

    // Creates JSON payload
    let body = json!({
        "model": model_str, // ou gpt-4o, gpt-4.1
        "messages": [
            {
                "role": "user",
                "content": [
                    {"type": "text", "text": img_prompt_str},
                    {"type": "image_url", "image_url": {
                        "url": format!("data:image/png;base64,{}", image_b64)
                    }}
                ]
            }
        ]
    });

    // Sends POST request
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?;

    // Parses JSON response
    let parsed_response: Value = match serde_json::from_str(&response.text().await.unwrap()) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
            return Err(err.into());
        } 
    };
    

    alternate_text_window::run_tooltip_window(parsed_response["choices"][0]["message"]["content"].to_string());
    // println!("{}",parsed_response);

    Ok(())
}


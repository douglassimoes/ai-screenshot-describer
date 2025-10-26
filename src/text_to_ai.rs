use reqwest::{self, Request};
use serde::{Serialize,Deserialize};
use serde_json::Value;
use std::error::Error;
use dotenv::dotenv;
use std::env;

use reqwest::{Client,header::HeaderMap, Url};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};


#[derive(Clone,Serialize)]
struct Message {
    role: String,
    content: String,
}


impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{role: {}, content: {}}}", self.role, self.content)
    }
}

#[derive(Serialize)]
struct RequestBody{
    model: String,
    // we need a Vector of messages because sometimes there will be one message
    // sometimes many
    input: Vec<Message>
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

    // Loads the prompt
    let prompt = env::var("PROMPT").expect("PROMPT not set");
    let prompt_str : String = prompt.parse().expect("Not able to parse PROMPT.");

    // What is the role of AI on this prompt
    let sysrole = env::var("SYSTEM_ROLE").expect("SYSTEM_ROLE not set");
    let sysrole_str : String = sysrole.parse().expect("Not able to parse SYSTEM_ROLE.");

    // create Client, responsible for managing HTTP connections
    let client = Client::new();     

    // create headers
    let mut headers: HeaderMap = HeaderMap::new();
    let header_string = format!("Bearer {}", api_key_str).parse::<String>().unwrap();
    let header_value = HeaderValue::from_str(&header_string).unwrap();
    headers.insert(AUTHORIZATION, header_value);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Doing Request to AI Model 
    let request_body = RequestBody {
        model: model_str,
        input:[ 
            Message {
            role: "system".to_string(),
            content:sysrole_str,
            },
            Message {
            role: "user".to_string(),
            content:prompt_str,
            }
        ].to_vec()
    };

    let url = Url::parse(&format!("{}", "https://api.openai.com/v1/responses")).unwrap();
    let response = client
            .post(url)
            .headers(headers.clone())
            .body(serde_json::to_string_pretty(&request_body).unwrap())
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


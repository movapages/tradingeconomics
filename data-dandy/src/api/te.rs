// TradingEconomics API logic stub

use dotenv::dotenv;
use reqwest::blocking::Client;
use std::env;

pub fn fetch_crude_oil_prices() {
    // TODO: Implement API call
    println!("[API] Fetching crude oil prices (stub)");
}

pub fn test_api_connection(log: &mut Vec<String>) {
    dotenv().ok(); // Loads .env
    log.push("Launching Data Dandy dashboard...".to_string());
    let api_key = env::var("TE_API_KEY").unwrap_or_else(|_| "NO_API_KEY".to_string());

    let url = format!(
        "https://api.tradingeconomics.com/markets/historical/CL1:COM?c={}&f=json",
        api_key
    );

    let client = Client::new();
    let resp = client.get(&url).send();

    match resp {
        Ok(response) => {
            log.push(format!("Status: {}", response.status()));
            let text = response
                .text()
                .unwrap_or_else(|_| "Failed to read body".to_string());
            log.push(format!("Body: {}", &text[..text.len().min(500)]));
        }
        Err(e) => {
            log.push(format!("Error connecting to TradingEconomics API: {e}"));
        }
    }
}

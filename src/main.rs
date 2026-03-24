use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Quote {
    #[serde(rename = "01. symbol")]
    symbol: String,

    #[serde(rename = "09. change")]
    change: String,
    
    #[serde(rename = "10. change percent")]
    change_percent: String,
    
    #[serde(rename = "05. price")]
    price: String,
    
    #[serde(rename = "06. volume")]
    volume: String,
    
    #[serde(rename = "07. latest trading day")]
    latest_trading_day: String,
    
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    #[serde(rename = "Global Quote")]
    global_quote: Quote,
}

fn fetch_quote(ticker: &str) -> Result<Quote, Box<dyn Error>> {
    let api_key = std::env::var("ALPHA_VANTAGE_KEY")
    .expect("ALPHA_VANTAGE_KEY not set");

    let url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}", 
        ticker, api_key
    );
    
    let response = reqwest::blocking::get(url)?.json::<ApiResponse>()?; // parse the JSON response into an ApiResponse struct   
    
    Ok(response.global_quote)
}

fn main() {
    match fetch_quote("AAPL") {
        Ok(quote) => {
            println!("Symbol:       {}", quote.symbol);
            println!("Price:        ${}", quote.price);
            println!("Change:       {}", quote.change);
            println!("Change %:     {}", quote.change_percent);
            println!("Volume:       {}", quote.volume);
            println!("Last trading: {}", quote.latest_trading_day);
        }
        Err(e) => println!("Error: {}", e),
    }
}
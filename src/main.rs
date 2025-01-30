use reqwest;
use serde_json::Value;
use std::env;
use std::process;

#[tokio::main]
async fn main() {
    // Get the amount and currencies from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: cargo run <amount> <from_currency> <to_currency>");
        process::exit(1);
    }

    let amount: f64 = match args[1].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid amount: {}", args[1]);
            process::exit(1);
        }
    };

    let from_currency = args[2].to_uppercase();
    let to_currency = args[3].to_uppercase();

    // API URL
    let api_url = format!("https://v6.exchangerate-api.com/v6/2928ee1ec4008f00faf3c444/latest/{}", from_currency);

    // Fetch the exchange rate
    let response = reqwest::get(&api_url).await;

    // Handle failure to fetch data
    let response = match response {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Failed to fetch exchange rate: {}", err);
            process::exit(1);
        }
    };

    // Parse the JSON response
    let data: Result<Value, _> = response.json().await;

    // Handle failure to parse the response
    let data = match data {
        Ok(d) => d,
        Err(err) => {
            eprintln!("Failed to parse response: {}", err);
            process::exit(1);
        }
    };

    // Extract the exchange rate for the target currency
    let rate = data["conversion_rates"][&to_currency].as_f64();

    match rate {
        Some(exchange_rate) => {
            let converted = amount * exchange_rate;
            println!("{:.2} {} is approximately {:.2} {}", amount, from_currency, converted, to_currency);
        }
        None => {
            eprintln!("Could not retrieve exchange rate for {}.", to_currency);
            process::exit(1);
        }
    }
}

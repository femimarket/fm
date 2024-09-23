use serde_json::Value;
use crate::trade::Summary;

pub fn oanda_account_summary(
    oanda_account:&str,
    oanda_secret:&str
)->Summary{
    let url = format!(
        "https://api-fxpractice.oanda.com/v3/accounts/{oanda_account}/summary"
    );
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", oanda_secret))
        .send().
        unwrap();
    let summary = match resp.status().is_client_error() {
        true => {
            let url = resp.url().clone();
            println!("{} {}", resp.text().unwrap(), url);
            panic!("ff");
        }
        false => {
            let j = resp.json::<Value>().unwrap();
            let account = j["account"].as_object().unwrap();
            let pl = account["pl"].as_str().unwrap().to_string();
            let balance = account["balance"].as_str().unwrap().to_string();
            Summary {
                pl:pl.parse::<f64>().unwrap(),
                balance:balance.parse::<f64>().unwrap(),
            }
        }
    };
    summary
}
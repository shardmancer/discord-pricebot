use reqwest::Client;
use serde::Deserialize;
use serenity::http::Http;
use serenity::model::id::ChannelId;
use serenity::builder::EditChannel;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
struct CoinPaprikaTicker {
    quotes: CoinPaprikaQuotes,
}

#[derive(Debug, Deserialize)]
struct CoinPaprikaQuotes {
    #[serde(rename = "USD")]
    usd: CoinPaprikaQuote,
}

#[derive(Debug, Deserialize)]
struct CoinPaprikaQuote {
    price: f64,
}

async fn get_price(client: &Client, coin_id: &str) -> Result<f64, reqwest::Error> {
    let url = format!("https://api.coinpaprika.com/v1/tickers/{}", coin_id);
    let response = client.get(&url).send().await?;

    let response = response.error_for_status()?;
    let ticker: CoinPaprikaTicker = response.json().await?;
    Ok(ticker.quotes.usd.price)
}

async fn update_channel_name(http: &Http, channel_id: u64, new_name: &str) {
    let channel_id = ChannelId::new(channel_id);
    let edit_channel = EditChannel::new().name(new_name);

    if let Err(why) = channel_id.edit(http, edit_channel).await {
        eprintln!("Failed to update channel {}: {:?}", channel_id, why);
    } else {
        println!("Updated channel {} to {}", channel_id, new_name);
    }
}

async fn send_price_message(webhook_url: &str, message: &str) {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "content": message
    });

    if let Err(err) = client.post(webhook_url).json(&payload).send().await {
        eprintln!("Failed to send message: {:?}", err);
    } else {
        println!("Message sent to webhook");
    }
}

fn format_price_fullwidth(price: f64) -> String {
    let formatted_price = format!("{:.2}", price);
    formatted_price
        .chars()
        .map(|c| match c {
            '0' => '０',
            '1' => '１',
            '2' => '２',
            '3' => '３',
            '4' => '４',
            '5' => '５',
            '6' => '６',
            '7' => '７',
            '8' => '８',
            '9' => '９',
            '.' => '．',
            _ => c,
        })
        .collect::<String>()
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let discord_token = env::var("DISCORD_BOT_TOKEN").expect("Missing DISCORD_BOT_TOKEN");
    let http = Http::new(&discord_token);
    let client = Client::new();

    let btc_channel_id: u64 = env::var("BTC_CHANNEL_ID").unwrap().parse().unwrap();
    let ton_channel_id: u64 = env::var("TON_CHANNEL_ID").unwrap().parse().unwrap();
    let sol_channel_id: u64 = env::var("SOL_CHANNEL_ID").unwrap().parse().unwrap();
    let bnb_channel_id: u64 = env::var("BNB_CHANNEL_ID").unwrap().parse().unwrap();
    let eth_channel_id: u64 = env::var("ETH_CHANNEL_ID").unwrap().parse().unwrap();

    let btc_webhook = env::var("BTC_WEBHOOK").unwrap();
    let ton_webhook = env::var("TON_WEBHOOK").unwrap();
    let sol_webhook = env::var("SOL_WEBHOOK").unwrap();
    let bnb_webhook = env::var("BNB_WEBHOOK").unwrap();
    let eth_webhook = env::var("ETH_WEBHOOK").unwrap();

    let btc_id = "btc-bitcoin";
    let ton_id = "ton-toncoin";
    let sol_id = "sol-solana";
    let bnb_id = "bnb-binance-coin";
    let eth_id = "eth-ethereum";

    loop {
        let prices = tokio::join!(
            get_price(&client, btc_id),
            get_price(&client, ton_id),
            get_price(&client, sol_id),
            get_price(&client, bnb_id),
            get_price(&client, eth_id)
        );

        let symbols = vec!["BTC", "TON", "SOL", "BNB", "ETH"];
        let results = vec![prices.0, prices.1, prices.2, prices.3, prices.4];

        let mut price_map = std::collections::HashMap::new();

        for (i, result) in results.into_iter().enumerate() {
            match result {
                Ok(price) => {
                    price_map.insert(symbols[i], price);
                }
                Err(e) => {
                    eprintln!("Failed to fetch {} price: {:?}", symbols[i], e);
                    price_map.insert(symbols[i], -1.0);
                }
            }
        }

        let assets = vec![
            ("BTC", price_map["BTC"], &btc_webhook, btc_channel_id, "📈"),
            ("TON", price_map["TON"], &ton_webhook, ton_channel_id, "💎"),
            ("SOL", price_map["SOL"], &sol_webhook, sol_channel_id, "🌞"),
            ("BNB", price_map["BNB"], &bnb_webhook, bnb_channel_id, "🟡"),
            ("ETH", price_map["ETH"], &eth_webhook, eth_channel_id, "🟣"),
        ];

        for (ticker, price, webhook_url, channel_id, emoji) in assets {
            if price == -1.0 {
                eprintln!("Skipping {} due to previous fetch error", ticker);
                continue;
            }

            let message = format!("{} {}: ${:.2}", emoji, ticker, price);
            send_price_message(webhook_url, &message).await;

            let formatted_price_fullwidth = format_price_fullwidth(price);
            let channel_name = format!("{}{}{}", emoji, ticker.to_lowercase(), formatted_price_fullwidth);

            update_channel_name(&http, channel_id, &channel_name).await;

            sleep(Duration::from_secs(5)).await;
        }

        sleep(Duration::from_secs(300)).await;
    }
}
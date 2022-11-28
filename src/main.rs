use std::{f64, fs::write, time::SystemTime};

use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Coin {
    name: String,
    price: f64,
    symbol: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct List {
    date: SystemTime,
    coins: Vec<Coin>,
}
impl List {
    pub fn new() -> Self {
        let coins = fetch_coins();
        let date = SystemTime::now();
        Self { coins, date }
    }
    fn save(&self) {
        let s = serde_json::to_string_pretty(&self).unwrap();
        write("list.json", &s).unwrap();
    }
}

fn get_text(row: ElementRef, selector: &Selector) -> String {
    match row.select(selector).next() {
        Some(r) => {
            let s = r.text().collect::<String>();
            s.trim().to_string().replace("\n", "")
        }
        _ => String::new(),
    }
}

fn parse_price(s: &String) -> f64 {
    s.replace("$", "")
        .replace(",", "")
        .parse::<f64>()
        .unwrap_or(0.0)
}

// Load top 100 coins
fn fetch_coins() -> Vec<Coin> {
    let url = "https://www.coingecko.com/";
    let resp = reqwest::blocking::get(url).unwrap();
    let html_text = resp.text().unwrap();

    let fragment = Html::parse_fragment(&html_text);

    let rows_selector = Selector::parse("[data-target=\"currencies.contentBox\"] tr").unwrap();

    let row_name_selector = Selector::parse("td.coin-name a span:first-child").unwrap();
    let row_symbol_selector = Selector::parse("td.coin-name a span:last-child").unwrap();
    let row_price_selector = Selector::parse("td.price").unwrap();
    // let row_1h_selector = Selector::parse("td.change1h").unwrap();
    // let row_24h_selector = Selector::parse("td.change24h").unwrap();
    // let row_7d_selector = Selector::parse("td.change7d").unwrap();

    let mut coins = vec![];

    for row in fragment.select(&rows_selector) {
        let row_name = get_text(row, &row_name_selector);
        let row_price = get_text(row, &row_price_selector);
        let row_symbol = get_text(row, &row_symbol_selector);
        // let row_1h = get_text(row, &row_1h_selector);
        // let row_24h = get_text(row, &row_24h_selector);
        // let row_7d = get_text(row, &row_7d_selector);

        let c = Coin {
            name: row_name,
            price: parse_price(&row_price),
            symbol: row_symbol,
        };
        coins.push(c);
    }
    coins
}

fn main() {
    let list = List::new();
    list.save();
}

// setup a local DeepLX service: https://github.com/OwO-Network/DeepLX
static DEEPLX: &str = "http://localhost:1188/translate";

use crate::convert::Food;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct TransRequest<'a> {
    text: &'a str,
    source_lang: &'a str,
    target_lang: &'a str,
}
impl TransRequest<'_> {
    fn new(text: &str) -> TransRequest {
        TransRequest {
            text: text,
            source_lang: "EN",
            target_lang: "ZH",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TransResult {
    alternatives: Vec<String>,
    code: i32,
    data: String,
    id: i64,
}

pub fn translate(foods: &mut Vec<Food>) {
    let length = foods.len();
    let client = reqwest::blocking::Client::new();

    for (index, food) in foods.iter_mut().enumerate() {
        println!("{}/{} translating: {}", index, length, food.name);
        let trans_req = TransRequest::new(&food.name);
        let response = client.post(DEEPLX).json(&trans_req).send();
        match response {
            Ok(r) => {
                let result = r.json::<TransResult>();
                match result {
                    Ok(r) => {
                        food.chinese_name = r.data;
                    }
                    Err(e) => {
                        println!("failed to translate {}: {}", food.name, e.to_string())
                    }
                }
            }
            Err(e) => {
                println!("failed to request {}: {}", food.name, e.to_string())
            }
        }
    }
}

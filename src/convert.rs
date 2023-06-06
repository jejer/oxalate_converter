use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Food {
    pub name: String,
    pub quantity: String,
    pub oxalate: String,
    pub source: String,
    pub mg: String,
    #[serde(skip_deserializing)]
    pub chinese_name: String,
    #[serde(skip_deserializing)]
    pub oxalate_per_100g: i32,
    #[serde(skip_deserializing)]
    pub risk: i32,
}

impl Food {
    fn calc_risk(&mut self, o: i32) {
        self.oxalate_per_100g = o;
        match self.oxalate_per_100g {
            -1 => self.risk = -1,     // unknown
            0 => self.risk = 0,       // 0
            1..=20 => self.risk = 1,  // low
            21..=60 => self.risk = 2, // medium
            _ => self.risk = 3,       // high
        }
    }
    fn oxalate_to_f64(&self) -> f64 {
        let re = Regex::new(r"(\d+(?:\.\d+)?) mg").unwrap();
        if let Some(cap) = re.captures(&self.oxalate) {
            let oxalate_str = &cap[1];
            let oxalate: f64 = oxalate_str.parse().unwrap();
            return oxalate;
        }
        return 0.0;
    }
}

pub fn convert(foods: &mut Vec<Food>) {
    for food in foods.iter_mut() {
        if food.oxalate == "0 mg" || food.mg == "0" {
            food.calc_risk(0);
            continue;
        }

        // 100 g
        if food.quantity.contains("100 g") {
            food.calc_risk(food.mg.parse().unwrap());
            continue;
        }

        // oz 1 oz = 28.34952 g
        if food.quantity.contains(" oz") {
            let re = Regex::new(r"(\d+(?:\.\d+)?) oz").unwrap();
            if let Some(cap) = re.captures(&food.quantity) {
                let oz_str = &cap[1];
                let oz: f64 = oz_str.parse().unwrap();

                let o = (100.0 * food.oxalate_to_f64()) / (28.34952 * oz);
                food.calc_risk(o.round() as i32);
                continue;
            }
        }
        if food.quantity.contains("3.5oz") {
            let o = (100.0 * food.oxalate_to_f64()) / (28.34952 * 3.5);
            food.calc_risk(o.round() as i32);
            continue;
        }

        // cup 1 cup = 240 g
        if food.quantity.contains(" cup") {
            if food.quantity.contains("1 1/3") {
                let o = (100.0 * food.oxalate_to_f64()) / (1.33333 * 240.0);
                food.calc_risk(o.round() as i32);
                continue;
            }
            if food.quantity.contains("1 1/4") {
                let o = (100.0 * food.oxalate_to_f64()) / (1.25 * 240.0);
                food.calc_risk(o.round() as i32);
                continue;
            }
            if food.quantity.contains("1/2 cup") {
                let o = (100.0 * food.oxalate_to_f64()) / (0.5 * 240.0);
                food.calc_risk(o.round() as i32);
                continue;
            }
            if food.quantity.contains("3/4 cup") {
                let o = (100.0 * food.oxalate_to_f64()) / (0.75 * 240.0);
                food.calc_risk(o.round() as i32);
                continue;
            }
            if food.quantity.contains("1/3 cup") {
                let o = (100.0 * food.oxalate_to_f64()) / (0.33333 * 240.0);
                food.calc_risk(o.round() as i32);
                continue;
            }
            if food.quantity.contains("2/3 cup") {
                let o = (100.0 * food.oxalate_to_f64()) / (0.66666 * 240.0);
                food.calc_risk(o.round() as i32);
                continue;
            }
            if food.quantity.contains("1/4 cup") {
                let o = (100.0 * food.oxalate_to_f64()) / (0.25 * 240.0);
                food.calc_risk(o.round() as i32);
                continue;
            }
            if food.quantity.contains("1 cup") {
                let o = (100.0 * food.oxalate_to_f64()) / 240.0;
                food.calc_risk(o.round() as i32);
                continue;
            }
            println!("uncatched cup size: {}", food.quantity);
            food.calc_risk(-1);
            continue;
        }

        // tbs 1 tablespoon = 15 grams
        if food.quantity.to_lowercase().contains("tbs")
            || food.quantity.to_lowercase().contains("tsp")
        {
            let re = Regex::new(r"(\d+) ((t|T)bs|tsp)").unwrap();
            if let Some(cap) = re.captures(&food.quantity) {
                let tbs_str = &cap[1];
                let tbs: f64 = tbs_str.parse().unwrap();

                let o = (100.0 * food.oxalate_to_f64()) / (15.0 * tbs);
                food.calc_risk(o.round() as i32);
                continue;
            }

            if food.quantity.contains("3/4 Tbs") {
                let o = (100.0 * food.oxalate_to_f64()) / (0.75 * 15.0);
                food.calc_risk(o.round() as i32);
                continue;
            }
        }

        println!("uncatched quantity: {}", food.quantity);
        food.calc_risk(-1);
        continue;
    }
}

pub fn save(foods: &Vec<Food>) {
    let result = serde_json::to_string(&foods).unwrap();

    fs::write("result.json", result).expect("Unable to write file")
}

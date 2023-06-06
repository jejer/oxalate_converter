use crate::convert::Food;

static DATA_LINK: &str = "https://oxalate.org/data.php";

pub fn fetch() -> Result<Vec<Food>, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(DATA_LINK)?
        .json::<Vec<Food>>()
        .unwrap();
    Ok(resp)
}

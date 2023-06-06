mod convert;
mod fetch;
mod translate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading data...");
    let mut foods = fetch::fetch().expect("Failed to fetch data from https://oxalate.org/data.php");
    println!("Converting...");
    convert::convert(&mut foods);
    println!("Translating...");
    translate::translate(&mut foods);
    convert::save(&foods);
    println!("Done!");
    return Ok(());
}

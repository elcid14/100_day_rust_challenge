//import necesssary libraries here
use std::{path::Path, error, mem::replace, io::{self, BufRead}, fs::File};
use serde::Deserialize;

// Define the struct for the object expected for the json file
#[derive(Debug, serde::Deserialize)]
pub struct Stock {
    pub name: String,
    pub symbol: String,
    pub price: f64,
    pub total_valuation:f64
}

pub fn parse_to_struct() -> Vec<Stock>{
    let json_file = File::open("sample.json").expect("Could not open json file");
    println!("{:?}", json_file);
    let stocks:Vec<Stock> = serde_json::from_reader(json_file).expect("error reading json file");
    println!("{:?}", stocks);
    return stocks;
}

pub fn transform_and_write_to_json(mut stocks:Vec<Stock>){
// let json_file = File::open("sample.json").expect("Could not open json file");
// let mut new_stocks:Vec<Stock> = serde_json::from_reader(json_file).expect("error reading json file");
for stock in &mut stocks{
    let updated_sym = stock.symbol.to_ascii_uppercase();
    stock.symbol = updated_sym;
    println!("{}", stock.symbol)
}
println!("{:?}", stocks);

}
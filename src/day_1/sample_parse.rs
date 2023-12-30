//import necesssary libraries here
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};

//define enum for result

// Define the struct for the object expected for the json file
#[derive(Debug, Deserialize, Serialize)]
#[derive(PartialEq)]
pub struct Stock {
    pub name: String,
    pub symbol: String,
    pub price: f64,
    pub total_valuation:f64
}

pub fn parse_to_struct(file_name: String) -> Vec<Stock>{
    let json_file = File::open(file_name).expect("Could not open json file");
    println!("{:?}", json_file);
    let stocks:Vec<Stock> = serde_json::from_reader(json_file).expect("error reading json file");
    println!("Stocks from origional file{:?}", stocks);
    return stocks;
}

pub fn transform_and_write_to_json(mut stocks:Vec<Stock>) -> std::io::Result<Vec<Stock>>{
for stock in &mut stocks{
    let updated_sym = stock.symbol.to_ascii_uppercase();
    stock.symbol = updated_sym;
}
println!("Updated stocks{:?}", stocks);
let seralized_struct = serde_json::to_string_pretty(&stocks).unwrap();
let mut file_write = std::fs::OpenOptions::new().write(true).create(true).open("sample.json").unwrap();
file_write.write_all(seralized_struct.as_bytes())?;
Ok(stocks)
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_get_file_and_parse_json_to_struct(){
        let response: Vec<Stock>= vec![
        Stock { name: "apple".to_string(), symbol: "appl".to_string(), price: 45.0, total_valuation: 4000000000.0 }, 
        Stock { name: "meta".to_string(), symbol: "meta".to_string(), price: 99.999, total_valuation: 9647832999.545 }, 
        Stock { name: "google".to_string(), symbol: "goog".to_string(), price: 45.0, total_valuation: 4000000000.0 }];
        let test_func = parse_to_struct("test_files/day_1_test.json".to_string());
        assert_eq!(response,test_func);

    }

}
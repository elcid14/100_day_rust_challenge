//import necesssary libraries here
use std::{path::Path, error, mem::replace, io::{self, BufRead}, fs::File};
use serde::{Deserialize, Serialize};
use regex::Regex;


// Define the struct for the object expected for the json file
//  #[derive(Serialize, Deserialize, Debug)]
struct Stock {
    name: String,
    symbol: String,
    price: f64,
    total_valuation:f64
}

pub fn parse_to_struct(){
    
}

pub fn parse_to_json(){


}
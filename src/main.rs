use dotenv::dotenv;

mod day_1;
fn main() -> std::io::Result<()> {
    dotenv().ok();
    let json_to_struct_stocks = day_1::sample_parse::parse_to_struct(std::env::var("DAY_1_FILE_NAME").expect("Could not load env var"));
    day_1::sample_parse::transform_and_write_to_json(json_to_struct_stocks).expect("Could not parse and write back to JSON file");
    Ok(())
    
}

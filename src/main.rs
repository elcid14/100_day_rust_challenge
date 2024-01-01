use dotenv::dotenv;

mod day_1;
mod day_2;
fn main() -> std::io::Result<()> {
    dotenv().ok();
    //day_1 code
    
    // let json_to_struct_stocks = day_1::sample_parse::parse_to_struct(std::env::var("DAY_1_FILE_NAME").expect("Could not load env var"));
    // day_1::sample_parse::transform_and_write_to_json(json_to_struct_stocks).expect("Could not parse and write back to JSON file");
    // Ok(())

    //day_2 code
    match day_2::reqwest_demo::get_reqwest(){
        Ok(_) => println!("Success!"),
        Err(err) => eprintln!("Error: {:?}", err)
    }

    Ok(())
    
}

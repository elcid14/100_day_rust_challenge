use std::{env, path::Path};

mod day_1;
fn main() -> std::io::Result<()> {
    let json_to_struct_stocks = day_1::sample_parse::parse_to_struct();
    let stuct_to_json_file = day_1::sample_parse::transform_and_write_to_json(json_to_struct_stocks);
    Ok(())
    
}

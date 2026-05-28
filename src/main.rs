use std::{env, fs::File};

use bank_system_ya::{csv_format::YPBankCsv, format::BankFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = env::args()
        .nth(1)
        .ok_or("usage: cargo run --bin bank_system_ya -- <input.csv>")?;

    let file = File::open(input_path)?;
    let transactions = YPBankCsv::read(file)?;

    dbg!(transactions);

    Ok(())
}

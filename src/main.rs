//! Консольное приложение для конвертации файлов `YPBank` между поддерживаемыми форматами.
//!
//! Утилита читает транзакции из файла, формат которого задан аргументом
//! `--input-format`, и записывает результат в `stdout` в формате `--output-format`.

use bank_system_ya::cli_helpers::{
    Format, next_value, open_file, read_transactions, write_transactions,
};
use std::env;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

const USAGE: &str = "Usage: ypbank_converter --input <file> --input-format <csv|text|binary> --output-format <csv|text|binary>";

struct Config {
    input: PathBuf,
    input_format: Format,
    output_format: Format,
}

impl Config {
    fn from_args<I>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = String>,
    {
        let mut input = None;
        let mut input_format = None;
        let mut output_format = None;

        let mut args = args.into_iter();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--input" => {
                    input = Some(PathBuf::from(next_value(&mut args, "--input")?));
                }
                "--input-format" => {
                    input_format = Some(next_value(&mut args, "--input-format")?.parse()?);
                }
                "--output-format" => {
                    output_format = Some(next_value(&mut args, "--output-format")?.parse()?);
                }
                _ => return Err(format!("Unknown argument: {arg}")),
            }
        }

        Ok(Self {
            input: input.ok_or("missing argument: --input")?,
            input_format: input_format.ok_or("Missing argument: --input-format")?,
            output_format: output_format.ok_or("Missing argument: --output-format")?,
        })
    }
}

fn run() -> Result<(), String> {
    let config = Config::from_args(env::args().skip(1))?;

    let input = open_file(&config.input)?;
    let transactions = read_transactions(input, &config.input_format).map_err(|error| {
        format!(
            "Could not read input file as {} format: {}",
            config.input_format, error
        )
    })?;

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    write_transactions(&mut writer, &config.output_format, &transactions).map_err(|error| {
        format!(
            "Could not write transactions as {} format: {}",
            config.output_format, error
        )
    })?;
    writer
        .flush()
        .map_err(|error| format!("Could not flush stdout: {error}"))?;

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        eprintln!("{USAGE}");
        std::process::exit(1);
    }
}

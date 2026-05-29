//! Консольное приложение для конвертации файлов YPBank между поддерживаемыми форматами.
//!
//! Утилита читает транзакции из файла, формат которого задан аргументом
//! `--input-format`, и записывает результат в `stdout` в формате `--output-format`.

use bank_system_ya::errors::ParseResult;
use bank_system_ya::format::BankFormat;
use bank_system_ya::{Transaction, YPBankCsv, YpBankBin, YpBankText};
use std::env;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

enum Format {
    Binary,
    Csv,
    Text,
}
impl FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bin" | "binary" | "b" | "bytes" => Ok(Self::Binary),
            "csv" | "table" | "c" => Ok(Self::Csv),
            "txt" | "text" | "t" => Ok(Self::Text),
            _ => Err(format!("Unsupported format: {}", s)),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Binary => {
                write!(f, "binary")
            }
            Format::Csv => {
                write!(f, "csv")
            }
            Format::Text => {
                write!(f, "text")
            }
        }
    }
}

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
                _ => return Err(format!("unknown argument: {arg}")),
            }
        }

        Ok(Self {
            input: input.ok_or("missing argument: --input")?,
            input_format: input_format.ok_or("missing argument: --input-format")?,
            output_format: output_format.ok_or("missing argument: --output-format")?,
        })
    }
}

fn next_value<I>(args: &mut I, flag: &'static str) -> Result<String, String>
where
    I: Iterator<Item = String>,
{
    args.next()
        .ok_or_else(|| format!("missing value for argument: {flag}"))
}

fn read_transactions<R: Read>(reader: R, format: &Format) -> ParseResult<Vec<Transaction>> {
    match format {
        Format::Csv => YPBankCsv::read(reader),
        Format::Text => YpBankText::read(reader),
        Format::Binary => YpBankBin::read(reader),
    }
}

fn write_transactions<W: Write>(
    writer: W,
    format: &Format,
    transactions: &[Transaction],
) -> ParseResult<()> {
    match format {
        Format::Csv => YPBankCsv::write(writer, transactions),
        Format::Text => YpBankText::write(writer, transactions),
        Format::Binary => YpBankBin::write(writer, transactions),
    }
}

fn run() -> Result<(), String> {
    let config = Config::from_args(env::args().skip(1))?;

    let input = File::open(&config.input).map_err(|error| {
        format!(
            "Could not open input file '{}': {}",
            config.input.display(),
            error
        )
    })?;
    let transactions = read_transactions(input, &config.input_format).map_err(|error| {
        format!(
            "Could not read input file as {} format: {}",
            config.input_format, error
        )
    })?;

    let stdout = io::stdout();
    write_transactions(stdout.lock(), &config.output_format, &transactions).map_err(|error| {
        format!(
            "Could not write transactions as {} format: {}",
            config.output_format, error
        )
    })?;

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        eprintln!(
            "usage: ypbank_converter --input <file> --input-format <csv|text|binary> --output-format <csv|text|binary>"
        );
        std::process::exit(1);
    }
}

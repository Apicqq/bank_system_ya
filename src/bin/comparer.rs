use bank_system_ya::Transaction;
use bank_system_ya::cli_helpers::{Format, next_value, open_file, read_transactions};
use std::env;
use std::path::PathBuf;

const USAGE: &str = "usage: ypbank_compare --file1 <file> --format1 <csv|text|binary> --file2 <file> --format2 <csv|text|binary>";

struct Config {
    file1: PathBuf,
    file1_format: Format,
    file2: PathBuf,
    file2_format: Format,
}

enum ComparerError {
    Usage(String),
    Runtime(String),
    Difference(String),
}

impl Config {
    fn from_args<I>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = String>,
    {
        let mut file1 = None;
        let mut file2 = None;
        let mut file1_format = None;
        let mut file2_format = None;

        let mut args = args.into_iter();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--file1" => {
                    file1 = Some(PathBuf::from(next_value(&mut args, "--file1")?));
                }
                "--file2" => {
                    file2 = Some(PathBuf::from(next_value(&mut args, "--file2")?));
                }
                "--format1" => {
                    file1_format = Some(next_value(&mut args, "--format1")?.parse()?);
                }
                "--format2" => {
                    file2_format = Some(next_value(&mut args, "--format2")?.parse()?);
                }
                _ => return Err(format!("Unknown argument: {arg}")),
            }
        }
        Ok(Self {
            file1: file1.ok_or("Missing argument: --file1")?,
            file2: file2.ok_or("Missing argument: --file2")?,
            file1_format: file1_format.ok_or("Missing argument: --format1")?,
            file2_format: file2_format.ok_or("Missing argument: --format2")?,
        })
    }
}

fn compare(lhs: &[Transaction], rhs: &[Transaction]) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    if lhs.len() != rhs.len() {
        results.push(format!(
            "Transaction count differs: left has {}, right has {}",
            lhs.len(),
            rhs.len()
        ))
    };
    for (index, (lhs_transaction, rhs_transaction)) in lhs.iter().zip(rhs.iter()).enumerate() {
        if lhs_transaction != rhs_transaction {
            results.push(format!(
                "Transaction #{} differs:\nleft: {:?}\nright: {:?}",
                index + 1,
                lhs_transaction,
                rhs_transaction
            ))
        }
    }
    results
}
fn run() -> Result<(), ComparerError> {
    let config = Config::from_args(env::args().skip(1)).map_err(ComparerError::Usage)?;

    let lhs_input = open_file(&config.file1).map_err(ComparerError::Runtime)?;
    let rhs_input = open_file(&config.file2).map_err(ComparerError::Runtime)?;

    let lhs_transactions = read_transactions(lhs_input, &config.file1_format).map_err(|error| {
        ComparerError::Runtime(format!(
            "Could not read '{}' as {} format: {}",
            config.file1.display(),
            config.file1_format,
            error
        ))
    })?;

    let rhs_transactions = read_transactions(rhs_input, &config.file2_format).map_err(|error| {
        ComparerError::Runtime(format!(
            "Could not read '{}' as {} format: {}",
            config.file2.display(),
            config.file2_format,
            error
        ))
    })?;

    let errors = compare(&lhs_transactions, &rhs_transactions);

    if errors.is_empty() {
        println!(
            "The transaction records in '{}' and '{}' are identical.",
            config.file1.display(),
            config.file2.display()
        );
        return Ok(());
    }

    Err(ComparerError::Difference(format!(
        "The transaction records in '{}' and '{}' are different:\n{}",
        config.file1.display(),
        config.file2.display(),
        errors.join("\n")
    )))
}

fn main() {
    if let Err(error) = run() {
        match error {
            ComparerError::Usage(message) => {
                eprintln!("{message}");
                eprintln!("{USAGE}");
            }
            ComparerError::Runtime(message) | ComparerError::Difference(message) => {
                eprintln!("{message}");
            }
        }

        std::process::exit(1);
    }
}

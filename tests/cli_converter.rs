use bank_system_ya::{Transaction, YpBankBin, format::BankFormat};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn converter_bin() -> &'static str {
    env!("CARGO_BIN_EXE_bank_system_ya")
}

fn temp_dir(test_name: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after UNIX_EPOCH")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "bank_system_ya_{test_name}_{}_{}",
        std::process::id(),
        suffix
    ));

    fs::create_dir_all(&path).expect("test temp directory must be created");
    path
}

fn sample_csv() -> &'static str {
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,Initial account funding\n"
}

#[test]
fn converter_writes_text_to_stdout() {
    let dir = temp_dir("writes_text");
    let input = dir.join("records.csv");
    fs::write(&input, sample_csv()).expect("CSV fixture must be written");

    let output = Command::new(converter_bin())
        .args([
            "--input",
            input.to_str().expect("test path must be UTF-8"),
            "--input-format",
            "csv",
            "--output-format",
            "text",
        ])
        .output()
        .expect("converter must start");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("text output must be valid UTF-8");
    assert_eq!(
        stdout,
        "TX_ID: 1001\n\
TX_TYPE: DEPOSIT\n\
FROM_USER_ID: 0\n\
TO_USER_ID: 501\n\
AMOUNT: 50000\n\
TIMESTAMP: 1672531200000\n\
STATUS: SUCCESS\n\
DESCRIPTION: \"Initial account funding\"\n"
    );
}

#[test]
fn converter_writes_binary_to_stdout() {
    let dir = temp_dir("writes_binary");
    let input = dir.join("records.csv");
    fs::write(&input, sample_csv()).expect("CSV fixture must be written");

    let output = Command::new(converter_bin())
        .args([
            "--input",
            input.to_str().expect("test path must be UTF-8"),
            "--input-format",
            "csv",
            "--output-format",
            "binary",
        ])
        .output()
        .expect("converter must start");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let transactions = YpBankBin::read(&output.stdout[..]).expect("binary stdout must be parsed");
    assert_eq!(
        transactions,
        vec![Transaction {
            tx_id: 1001,
            tx_type: bank_system_ya::TxType::Deposit,
            from_user_id: 0,
            to_user_id: 501,
            amount: 50_000,
            timestamp: 1_672_531_200_000,
            status: bank_system_ya::TxStatus::Success,
            description: "Initial account funding".to_string(),
        }]
    );
}

#[test]
fn converter_returns_error_for_unsupported_format() {
    let dir = temp_dir("unsupported_format");
    let input = dir.join("records.csv");
    fs::write(&input, sample_csv()).expect("CSV fixture must be written");

    let output = Command::new(converter_bin())
        .args([
            "--input",
            input.to_str().expect("test path must be UTF-8"),
            "--input-format",
            "xml",
            "--output-format",
            "csv",
        ])
        .output()
        .expect("converter must start");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr must be valid UTF-8");
    assert!(stderr.contains("Unsupported format: xml"));
    assert!(stderr.contains("usage: ypbank_converter"));
}

#[test]
fn converter_returns_error_when_input_does_not_match_format() {
    let dir = temp_dir("wrong_input_format");
    let input = dir.join("records.txt");
    fs::write(
        &input,
        "TX_ID: 1001\n\
TX_TYPE: DEPOSIT\n\
FROM_USER_ID: 0\n\
TO_USER_ID: 501\n\
AMOUNT: 50000\n\
TIMESTAMP: 1672531200000\n\
STATUS: SUCCESS\n\
DESCRIPTION: \"Initial account funding\"\n",
    )
    .expect("TXT fixture must be written");

    let output = Command::new(converter_bin())
        .args([
            "--input",
            input.to_str().expect("test path must be UTF-8"),
            "--input-format",
            "binary",
            "--output-format",
            "csv",
        ])
        .output()
        .expect("converter must start");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr must be valid UTF-8");
    assert!(stderr.contains("Could not read input file as binary format"));
}

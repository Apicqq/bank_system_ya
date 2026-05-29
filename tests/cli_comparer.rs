use bank_system_ya::{Transaction, TxStatus, TxType, YpBankBin, format::BankFormat};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn comparer_bin() -> &'static str {
    env!("CARGO_BIN_EXE_comparer")
}

fn temp_dir(test_name: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after UNIX_EPOCH")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "bank_system_ya_comparer_{test_name}_{}_{}",
        std::process::id(),
        suffix
    ));

    fs::create_dir_all(&path).expect("test temp directory must be created");
    path
}

fn sample_transaction() -> Transaction {
    Transaction {
        tx_id: 1001,
        tx_type: TxType::Deposit,
        from_user_id: 0,
        to_user_id: 501,
        amount: 50_000,
        timestamp: 1_672_531_200_000,
        status: TxStatus::Success,
        description: "Initial account funding".to_string(),
    }
}

fn sample_csv() -> &'static str {
    "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,Initial account funding\n"
}

fn sample_text() -> &'static str {
    "TX_ID: 1001\n\
TX_TYPE: DEPOSIT\n\
FROM_USER_ID: 0\n\
TO_USER_ID: 501\n\
AMOUNT: 50000\n\
TIMESTAMP: 1672531200000\n\
STATUS: SUCCESS\n\
DESCRIPTION: \"Initial account funding\"\n"
}

#[test]
fn comparer_reports_identical_records_for_csv_and_text() {
    let dir = temp_dir("csv_text_identical");
    let csv = dir.join("records.csv");
    let text = dir.join("records.txt");
    fs::write(&csv, sample_csv()).expect("CSV fixture must be written");
    fs::write(&text, sample_text()).expect("text fixture must be written");

    let output = Command::new(comparer_bin())
        .args([
            "--file1",
            csv.to_str().expect("test path must be UTF-8"),
            "--format1",
            "csv",
            "--file2",
            text.to_str().expect("test path must be UTF-8"),
            "--format2",
            "text",
        ])
        .output()
        .expect("comparer must start");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("stdout must be valid UTF-8");
    assert!(stdout.contains("are identical"));
}

#[test]
fn comparer_reports_identical_records_for_csv_and_binary() {
    let dir = temp_dir("csv_binary_identical");
    let csv = dir.join("records.csv");
    let binary = dir.join("records.bin");
    fs::write(&csv, sample_csv()).expect("CSV fixture must be written");

    let mut binary_content = Vec::new();
    YpBankBin::write(&mut binary_content, &[sample_transaction()])
        .expect("binary fixture must be serialized");
    fs::write(&binary, binary_content).expect("binary fixture must be written");

    let output = Command::new(comparer_bin())
        .args([
            "--file1",
            csv.to_str().expect("test path must be UTF-8"),
            "--format1",
            "csv",
            "--file2",
            binary.to_str().expect("test path must be UTF-8"),
            "--format2",
            "binary",
        ])
        .output()
        .expect("comparer must start");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("stdout must be valid UTF-8");
    assert!(stdout.contains("are identical"));
}

#[test]
fn comparer_reports_changed_transaction() {
    let dir = temp_dir("changed_transaction");
    let left = dir.join("left.csv");
    let right = dir.join("right.csv");
    fs::write(&left, sample_csv()).expect("left fixture must be written");
    fs::write(
        &right,
        "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
1001,DEPOSIT,0,501,60000,1672531200000,SUCCESS,Initial account funding\n",
    )
    .expect("right fixture must be written");

    let output = Command::new(comparer_bin())
        .args([
            "--file1",
            left.to_str().expect("test path must be UTF-8"),
            "--format1",
            "csv",
            "--file2",
            right.to_str().expect("test path must be UTF-8"),
            "--format2",
            "csv",
        ])
        .output()
        .expect("comparer must start");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr must be valid UTF-8");
    assert!(stderr.contains("are different"));
    assert!(stderr.contains("Transaction #1 differs"));
    assert!(!stderr.contains("usage: ypbank_compare"));
}

#[test]
fn comparer_reports_transaction_count_difference() {
    let dir = temp_dir("count_difference");
    let left = dir.join("left.csv");
    let right = dir.join("right.csv");
    fs::write(&left, sample_csv()).expect("left fixture must be written");
    fs::write(
        &right,
        "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n",
    )
    .expect("right fixture must be written");

    let output = Command::new(comparer_bin())
        .args([
            "--file1",
            left.to_str().expect("test path must be UTF-8"),
            "--format1",
            "csv",
            "--file2",
            right.to_str().expect("test path must be UTF-8"),
            "--format2",
            "csv",
        ])
        .output()
        .expect("comparer must start");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr must be valid UTF-8");
    assert!(stderr.contains("Transaction count differs"));
    assert!(!stderr.contains("usage: ypbank_compare"));
}

#[test]
fn comparer_prints_usage_for_argument_error() {
    let output = Command::new(comparer_bin())
        .args(["--file1", "records.csv"])
        .output()
        .expect("comparer must start");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr must be valid UTF-8");
    assert!(stderr.contains("Missing argument: --file2"));
    assert!(stderr.contains("usage: ypbank_compare"));
}

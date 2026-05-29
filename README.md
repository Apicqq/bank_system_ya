# bank_system_ya

Проектная работа по чтению, парсингу, сериализации и сравнению финансовых данных YPBank на Rust.

Проект реализован как один Cargo package с несколькими crate targets:

- `src/lib.rs` - библиотека parser с общей моделью, ошибками и реализациями форматов.
- `src/main.rs` - CLI converter для преобразования файлов между форматами.
- `src/bin/comparer.rs` - CLI comparer для сравнения транзакций из двух файлов.

## Поддерживаемые форматы

- `csv` / `table` / `c` - YPBankCsv.
- `text` / `txt` / `t` - YPBankText.
- `binary` / `bin` / `b` / `bytes` - YPBankBin.

## Converter

Конвертер читает входной файл в указанном формате и пишет результат в `stdout`.

```bash
cargo run --bin ypbank_converter -- --input records_example.csv --input-format csv --output-format text
```

Пример записи результата в файл:

```bash
cargo run --bin ypbank_converter -- --input records_example.csv --input-format csv --output-format binary > output.bin
```

## Comparer

Comparer читает два файла в указанных форматах и сравнивает полученные списки транзакций.

```bash
cargo run --bin ypbank_compare -- --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv
```

Если записи совпадают, утилита выводит сообщение об идентичности. Если записи отличаются, утилита выводит найденные различия.

## Проверки

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps
```

## Структура

```text
src/
  lib.rs          # публичный API библиотеки
  errors.rs       # ошибки парсинга и сериализации
  format.rs       # общий трейт BankFormat
  model.rs        # Transaction, TxType, TxStatus
  csv_format.rs   # YPBankCsv
  txt_format.rs   # YPBankText
  bin_format.rs   # YPBankBin
  cli_helpers.rs  # общие CLI-хелперы
  main.rs         # converter
  bin/
    comparer.rs   # comparer
tests/
  cli_converter.rs
  cli_comparer.rs
```

# Parser Crate And Converter CLI

`src/lib.rs` содержит библиотечную часть проекта: общую модель транзакций, ошибки, трейт формата и реализации чтения/записи для YPBankCsv, YPBankText и YPBankBin.

## Parser

Основные публичные элементы библиотеки:

- `Transaction`, `TxType`, `TxStatus` - общая модель данных.
- `ParserError`, `ParseResult` - ошибки и результат операций парсинга.
- `BankFormat` - общий интерфейс форматов.
- `YPBankCsv`, `YpBankText`, `YpBankBin` - реализации поддерживаемых форматов.

Чтение и запись выполняются через стандартные трейты `Read` и `Write`, поэтому библиотека не привязана к конкретному источнику данных: один и тот же код работает с файлами, буферами и другими источниками/приёмниками.

## Converter

`src/main.rs` содержит converter - CLI-приложение, использующее библиотеку для преобразования файлов между форматами. Утилита читает входной файл, парсит его в общую модель `Transaction` и записывает результат в `stdout` в выбранном формате.

Запуск:

```bash
cargo run --bin ypbank_converter -- --input <file> --input-format <csv|text|binary> --output-format <csv|text|binary>
```

Пример конвертации CSV в текстовый формат:

```bash
cargo run --bin ypbank_converter -- --input records_example.csv --input-format csv --output-format text
```

Пример записи бинарного результата в файл:

```bash
cargo run --bin ypbank_converter -- --input records_example.csv --input-format csv --output-format binary > output.bin
```

Поддерживаемые алиасы форматов:

- `csv`, `table`, `c` - YPBankCsv.
- `text`, `txt`, `t` - YPBankText.
- `binary`, `bin`, `b`, `bytes` - YPBankBin.

Ошибки конфигурации, чтения и записи выводятся в `stderr`. Данные успешной конвертации выводятся только в `stdout`, поэтому результат можно перенаправлять в файл.

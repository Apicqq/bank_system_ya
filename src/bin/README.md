# Comparer CLI

`comparer.rs` содержит CLI-приложение для сравнения двух файлов с транзакциями YPBank. Файлы могут быть записаны в разных форматах: comparer читает оба файла через parser-библиотеку, приводит данные к общей модели `Transaction` и сравнивает полученные списки.

## Запуск

```bash
cargo run --bin ypbank_compare -- --file1 <file> --format1 <csv|text|binary> --file2 <file> --format2 <csv|text|binary>
```

Пример сравнения бинарного файла и CSV:

```bash
cargo run --bin ypbank_compare -- --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv
```

## Аргументы

- `--file1` - путь к первому файлу.
- `--format1` - формат первого файла.
- `--file2` - путь ко второму файлу.
- `--format2` - формат второго файла.

Поддерживаемые алиасы форматов:

- `csv`, `table`, `c` - YPBankCsv.
- `text`, `txt`, `t` - YPBankText.
- `binary`, `bin`, `b`, `bytes` - YPBankBin.

## Результат

Если транзакции совпадают, comparer выводит сообщение в `stdout`:

```text
The transaction records in 'records_example.bin' and 'records_example.csv' are identical.
```

Если транзакции отличаются, comparer завершает работу с ненулевым кодом и выводит найденные различия в `stderr`: номер транзакции, левое значение и правое значение. Если количество транзакций отличается, это также выводится отдельным сообщением.

Ошибки аргументов сопровождаются строкой использования. Ошибки чтения файлов и различия данных не печатают usage, чтобы не смешивать справку с результатом сравнения.

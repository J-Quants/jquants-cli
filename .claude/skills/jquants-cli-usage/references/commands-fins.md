# Category: fins (Financials) — Command Reference

## fins details — 財務諸表

JSON 出力で FS フィールド（財務数値）が完全表示される:

```sh
jquants fins details --code 86970
jquants fins details --date 2022-01-05
jquants --output json fins details --code 86970   # FS フィールド完全表示
```

## fins dividend — 配当金情報

```sh
jquants fins dividend --code 27800
jquants fins dividend --date 2021-09-01
jquants fins dividend --from 2021-09-01 --to 2021-12-31
```

## fins summary — 財務情報サマリー

```sh
jquants fins summary --code 86970
jquants fins summary --date 2022-01-05
```

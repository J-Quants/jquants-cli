# Category: fins (Financials) — Command Reference

## fins details — 財務諸表

JSON 出力で FS フィールド（財務数値）が完全表示される:

```sh
jquants fins details --code 86970
jquants fins details --date 2022-01-05
jquants --output json fins details --code 86970   # FS フィールド完全表示

# 差分取得（cursor 指定）
jquants fins details --date 2022-01-05 --cursor eyJkIjoiMjAy...
```

### cursor について

`--cursor` を指定すると前回取得以降の差分のみ取得できる。
データ出力後に cursor 値が stderr に出力されるので、次回の `--cursor` に渡す。

```sh
# 初回（全件 + cursor 取得）
jquants fins details --date 2022-01-05
# → stderr: cursor: eyJkIjoiMjAy...

# 次回（差分のみ取得）
jquants fins details --date 2022-01-05 --cursor eyJkIjoiMjAy...
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

# 差分取得（cursor 指定）
jquants fins summary --date 2022-01-05 --cursor eyJkIjoiMjAy...
```

### cursor について

`--cursor` を指定すると前回取得以降の差分のみ取得できる。
データ出力後に cursor 値が stderr に出力されるので、次回の `--cursor` に渡す。

```sh
# 初回（全件 + cursor 取得）
jquants fins summary --date 2022-01-05
# → stderr: cursor: eyJkIjoiMjAy...

# 次回（差分のみ取得）
jquants fins summary --date 2022-01-05 --cursor eyJkIjoiMjAy...
```

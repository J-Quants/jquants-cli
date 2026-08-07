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

## fins earnings-date — 決算発表予定日

決算期によらず、報告を行った全上場銘柄（REIT等を含む）の決算発表予定日。
変更・未定の履歴を公表日単位で提供する。

```sh
jquants fins earnings-date --code 86970                 # 銘柄の予定日公表履歴
jquants fins earnings-date --date 2026-06-03            # 指定日に公表・変更された全銘柄の予定日
jquants fins earnings-date --scheduled-date 2026-08-05  # 指定日を現在有効な予定日とする全銘柄
```

### 制約・特殊仕様

- `--code` / `--date` / `--scheduled-date` は**いずれか1つの指定が必須**（2つ以上の同時指定は不可。CLI が実行前に拒否する）
- 予定日が未定の場合、`SchDate` は**空文字**で返る
- `--scheduled-date` は「現在有効な予定日」でのみヒットする。予定日がその後変更された場合、変更前の日付ではヒットしない
- 旧 `eq earnings-calendar`（3・9月期決算会社のみ・翌営業日分）とは別の API

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

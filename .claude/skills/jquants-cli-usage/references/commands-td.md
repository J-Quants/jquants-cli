# Category: td (TDnet/適時開示) — Command Reference

TDnet/適時開示インデックス情報を取得します。利用には **TDnet/適時開示情報アドオン** が必要です。
過去5年間のデータを取得可能。

## td list — 適時開示インデックス一覧

```sh
# 日付指定（その日の全適時開示）
jquants td list --date 2025-04-01

# 銘柄コード指定（全期間）
jquants td list --code 86970

# 銘柄コード + 期間指定
jquants td list --code 86970 --from 2025-01-01 --to 2025-03-31

# 公開項目コードでフィルタ（AND条件、カンマ区切り）
jquants td list --date 2025-04-01 --disc-items 140120

# ページネーションカーソル指定（当日データのリアルタイム取得に使用）
jquants td list --date 2025-04-01 --cursor <cursor_value>

# JSON出力（全フィールド）
jquants --output json td list --date 2025-04-01

# CSV保存
jquants --output csv --save td.csv td list --code 86970
```

### パラメータ

| オプション | 説明 | 備考 |
|-----------|------|------|
| `--date` | 開示日 (YYYYMMDD or YYYY-MM-DD) | `--code` とどちらか必須 |
| `--code` | 銘柄コード (例: 86970 or 8697) | `--date` とどちらか必須 |
| `--from` | 取得開始日 (YYYY-MM-DD) | `--code` との組み合わせで使用 |
| `--to` | 取得終了日 (YYYY-MM-DD) | `--code` との組み合わせで使用 |
| `--disc-items` | 公開項目コード（カンマ区切り、AND条件） | 任意 |
| `--cursor` | ページネーション用カーソル | 当日データのポーリングに使用 |

### cursor について

当日を `--date` で指定した場合、データ出力後に cursor 値が stderr に出力される。
次のポーリングで `--cursor` に渡すことで差分のみ取得できる。

```sh
# 初回（当日全件 + cursor 取得）
jquants td list --date 2025-05-19
# → stderr: cursor: eyJkIjoiMjAy...

# 次回（差分のみ取得）
jquants td list --date 2025-05-19 --cursor eyJkIjoiMjAy...
```

### レスポンスフィールド

| フィールド | 型 | 説明 |
|-----------|---|------|
| `DiscNo` | string | 開示番号（14桁） |
| `Code` | string | 銘柄コード |
| `Name` | string | 会社名 |
| `DiscDate` | string | 開示日 (YYYY-MM-DD) |
| `DiscTime` | string | 開示時刻 (HH:MM) |
| `Title` | string | 開示タイトル |
| `DiscStatus` | string? | 取扱属性（null=通常/revision=修正/delete=削除） |
| `RevNo` | string | 開示履歴番号（1〜99、APIが文字列で返す） |
| `DiscItems` | array | 公開項目コードリスト |
| `Docs` | array | 書類タイプ（g=PDF/s=サマリー/x=XBRL） |

### スキーマ確認

```sh
jquants schema td.list
jquants --output json schema td.list
```

---

## td files — 適時開示ファイルURL取得

開示番号に対応するファイル（PDF/XBRL）のダウンロードURLを取得します。**URLの有効期限は15分**。

```sh
# 基本使用（--disc-no は必須）
jquants td files --disc-no 20250401130100

# ファイル種類を絞り込み（g=PDF全文/s=サマリPDF/x=XBRL）
jquants td files --disc-no 20250401130100 --docs g

# ファイルを直接ダウンロード（pdf_<discNo>.pdf 等のファイル名で保存）
jquants td files --disc-no 20250401130100 --download

# PDF全文のみダウンロード
jquants td files --disc-no 20250401130100 --docs g --download

# JSON出力（URL確認に最適）
jquants --output json td files --disc-no 20250401130100
```

### パラメータ

| オプション | 説明 | 備考 |
|-----------|------|------|
| `--disc-no` | 開示番号（14桁）| 必須 |
| `--docs` | ファイル種類フィルタ（g/s/x） | 任意 |
| `--download` | ファイルを直接ダウンロード | 任意 |

### ダウンロードファイル名

`--download` 指定時のファイル名は `{種別}_{開示番号}.{拡張子}` の形式：

| 種別 | ファイル名例 |
|------|------------|
| PDF全文 (g) | `pdf_20250401130100.pdf` |
| サマリPDF (s) | `summary_20250401130100.pdf` |
| XBRL (x) | `xbrl_20250401130100.zip` |

`--docs` でフィルタした場合は対象ファイルのみダウンロード。null のファイルはスキップ。

### レスポンスフィールド

| フィールド | 型 | 説明 |
|-----------|---|------|
| `discNo` | string | 開示番号 |
| `files.pdf` | string? | 全文PDF ダウンロードURL（有効期限15分） |
| `files.summaryPdf` | string? | サマリPDF ダウンロードURL（有効期限15分） |
| `files.xbrl` | string? | XBRLファイル ダウンロードURL（有効期限15分） |

### 典型的な使用フロー

```sh
# 1. td list で開示番号を確認
jquants td list --date 2025-04-01 | head

# 2a. 開示番号でファイルを直接ダウンロード
jquants td files --disc-no 20250401130100 --download

# 2b. URLだけ確認したい場合
jquants --output json td files --disc-no 20250401130100
```

### スキーマ確認

```sh
jquants schema td.files
jquants --output json schema td.files
```

---

## td bulk — 適時開示一括ダウンロードURL取得

過去5年分の適時開示情報をまとめた gzip 圧縮 CSV ファイルのダウンロード URL を取得します。**URLの有効期限は15分**。

```sh
# URL取得（パラメータ不要）
jquants td bulk

# ファイルを直接ダウンロード
jquants td bulk --download

# JSON出力
jquants --output json td bulk
```

### パラメータ

| オプション | 説明 | 備考 |
|-----------|------|------|
| `--download` | gzip CSVを直接ダウンロード | 任意 |

### レスポンスフィールド

| フィールド | 型 | 説明 |
|-----------|---|------|
| `lastUpdated` | string | CSVファイルの最終更新日時（ISO 8601） |
| `url` | string | gzip圧縮CSVのダウンロードURL（有効期限15分） |

### CSVに含まれるフィールド

`DiscNo`, `Code`, `Name`, `DiscDate`, `DiscTime`, `Title`, `DiscStatus`, `RevNo`, `DiscItems`, `Docs`

### スキーマ確認

```sh
jquants schema td.bulk
jquants --output json schema td.bulk
```

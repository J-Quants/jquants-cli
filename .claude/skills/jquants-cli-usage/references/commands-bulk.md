# Category: bulk (Bulk Download) — Command Reference

## bulk get — バルクURL取得・ダウンロード

```sh
jquants bulk get --key "path/to/file.gz"                       # URL取得
jquants bulk get --endpoint /equities/bars/daily --date 2026-03
jquants bulk get --key "path/to/file.gz" --download            # ダウンロード
```

## bulk list — ダウンロード可能ファイル一覧

```sh
jquants bulk list
jquants bulk list --endpoint /equities/bars/daily
jquants bulk list --date 2026-03
jquants bulk list --from 2026-03-01 --to 2026-03-14
```

## バルクダウンロードのベストプラクティス

### いつバルクを使うか

| 条件 | 推奨 |
|---|---|
| 特定銘柄・直近数日 | 個別 API（`eq daily --code X`） |
| 全銘柄・1日分 | 個別 API でも可、バルクを推奨 |
| 全銘柄・複数月 / 長期間 | **バルク必須** |
| 分足（全銘柄・長期間） | **バルク推奨**（銘柄単位は `eq minute` で取得可） |
| ティックデータ | `eq trades` を使用（`--date YYYY-MM --download`） |
| バックテスト用ヒストリカル | **バルク必須** |

**原則: 「全銘柄」「長期間（1ヶ月以上）」「大量」のいずれかに該当 → バルクダウンロードを使う**

### バルク対応エンドポイント一覧

| カテゴリ | エンドポイント | データ名 |
|---|---|---|
| 株式 | `/equities/master` | 上場銘柄一覧 |
| 株式 | `/equities/bars/daily` | 株価四本値 |
| 株式 | `/equities/bars/minute` *(Add-on)* | 株価分足 |
| 株式 | `/equities/trades` *(Add-on)* | 株価ティック |
| 株式 | `/equities/investor-types` | 投資部門別情報 |
| 財務 | `/fins/summary` | 財務情報 |
| 財務 | `/fins/details` | 財務諸表（BS/PL/CF） |
| 財務 | `/fins/dividend` | 配当金情報 |
| 指数 | `/indices/bars/daily/topix` | TOPIX 四本値 |
| 指数 | `/indices/bars/daily` | 指数四本値 |
| デリバティブ | `/derivatives/bars/daily/options/225` | 日経225オプション四本値 |
| デリバティブ | `/derivatives/bars/daily/futures` | 先物四本値 |
| デリバティブ | `/derivatives/bars/daily/options` | オプション四本値 |
| 市場 | `/markets/margin-interest` | 信用取引週末残高 |
| 市場 | `/markets/short-ratio` | 業種別空売り比率 |
| 市場 | `/markets/short-sale-report` | 空売り残高報告 |
| 市場 | `/markets/margin-alert` | 日々公表信用取引残高 |
| 市場 | `/markets/breakdown` | 売買内訳データ |
| 市場 | `/markets/calendar` | 取引カレンダー |

### 典型ワークフロー

**パターン A: エンドポイント＋月指定で直接ダウンロード（推奨）**

```sh
# 月次の全銘柄株価四本値をダウンロード
jquants bulk get --endpoint /equities/bars/daily --date 2026-03 --download

# 保存先を確認（カレントディレクトリに .gz ファイルが作成される）
ls *.gz

# 解凍して CSV として利用
gunzip *.gz
```

**パターン B: 一覧確認 → Key 指定でダウンロード**

```sh
# 利用可能なファイルを確認
jquants bulk list --endpoint /equities/bars/daily --date 2026-03

# Key 列の値を使ってダウンロード
jquants bulk get --key "equities/bars/daily/2026/03/bars_daily_20260301.csv.gz" --download
```

**パターン C: ティックデータ（eq trades 専用）**

```sh
# ティックデータは bulk ではなく eq trades を使う
jquants eq trades --date 2025-12 --download
```

### 注意事項

- バルクファイルは GZ 圧縮で保存される（自動展開なし）→ `gunzip *.gz` で解凍してから CSV として読む
- `bulk get` が返す Presigned URL の有効期限は **5分**（URL 取得後すぐにダウンロードする）
- `--date` に `YYYY-MM` 形式（月指定）が使用可能

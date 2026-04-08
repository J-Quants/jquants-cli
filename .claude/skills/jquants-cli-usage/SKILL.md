---
name: jquants-cli-usage
description: >
  J-Quants CLI (jquants) で日本株式市場データを取得するコマンドを構築・実行する。
  株価・四本値・分足・ティック、信用取引残高・空売り比率、先物・オプション、
  財務諸表・配当金、TOPIX・指数データの取得に対応。
  プランごとの API 利用可否・取得可能データ期間（Free/Light/Standard/Premium）の確認にも対応。
  トリガー: jquants, J-Quants, 株価, 株価データ, 銘柄, 四本値, 分足,
  OHLCV, JPX, 東証, TOPIX, 日経225, 先物, オプション, デリバティブ,
  空売り, 信用取引, 配当, 財務諸表, 決算, PER, PBR, ROE,
  指数, バルクダウンロード, 日本株, Japanese stock,
  stock price, market data, financial statements,
  プラン, サブスクリプション, subscription, plan, Free, Light, Standard, Premium,
  取得可能期間, データ範囲, data range, API availability,
  更新時刻, データ更新, 更新タイミング, データが古い, 最新データ, いつ更新,
  data freshness, update schedule, update time, when is data updated, latest data
---

# J-Quants CLI 使用スキル

## Role & Decision Flow

jquants コマンドを構築するときは、以下の順序で判断する:

1. **データ種別を決める** — 株式(eq)・市場(mkt)・デリバティブ(deriv)・財務(fins)・指数(idx)
2. **プラン制限を確認する** — ユーザーのプランで利用可能か確認する（下記「Subscription Plans & API Availability」参照）。プランが不明な場合は、コマンドを提示しつつ必要プランを明記する
3. **フィールド名を確認する** — `-f` で絞り込む前に `jquants schema <endpoint>` で正確な名前を調べる（推測しない）
4. **個別API vs バルクを選ぶ** — 「全銘柄」「1ヶ月以上」「大量」のいずれかに該当 → バルク（references/commands-bulk.md 参照）。個別APIを大量に繰り返すとレートリミットに抵触するため（下記 Rate Limits 参照）。**Free プランは CSV/バルク不可**（`mkt calendar` を除く）
5. **コマンドを組み立てる** — グローバルオプション（`--output`, `--save`, `-f`）をサブコマンドの前に置く
6. **パイプを考慮する** — パイプ先がある場合は `--output` を省略するだけで自動的に CSV が流れる

## Prerequisites

**推奨: `jquants login` でブラウザログイン（API キーを自動取得・保存）**

```sh
jquants login   # ブラウザが開き、Cognito OAuth2 PKCE でログイン
                # → ~/.config/jquants/credentials.json に API キーを自動保存
```

**フォールバック: 環境変数で直接指定**

```sh
export JQUANTS_API_KEY="your-api-key"
# または .env ファイルに記載
```

**認証優先順位:** `~/.config/jquants/credentials.json` の api_key → `JQUANTS_API_KEY` 環境変数 → エラー（`jquants login` を促す）

> **`jquants schema` は認証不要。**ログイン前でもフィールド名の確認に使える。

## Command Syntax Rules

**グローバルオプション (`--output`, `--save`, `-f`) はサブコマンドの前に置く:**

```sh
# ✅ 正しい
jquants --output csv --save out.csv eq daily --code 86970

# ❌ 誤り（サブコマンド後は無効）
jquants eq daily --code 86970 --output csv
```

> **なぜ:** clap の設計上、サブコマンド以降のフラグはサブコマンド自身のオプションとして解析される。サブコマンドの後に置くと「不明なフラグ」エラーになる。

**パイプ時の自動CSV切替:**

stdout が TTY でない場合（パイプ先がある場合）、`--output table`（デフォルト）は自動的にCSV形式で出力される。

```sh
# パイプすると自動でCSVが流れる
jquants eq daily --code 86970 | head -5

# 明示的にJSONをパイプしたい場合は指定が必要
jquants --output json eq daily --code 86970 | jq '.[] | .AdjustmentClose'
```

**フィールド選択 (`--fields` / `-f`) でカラムを絞り込む:**

```sh
# 日付・コード・調整済終値だけを取得
jquants -f Date,Code,AdjustmentClose eq daily --code 86970

# 不正なフィールド名を指定するとエラーで利用可能フィールド一覧が表示される
```

## Schema-First: フィールド名の確認

`--fields` でカラムを絞り込む前に、`jquants schema <endpoint>` で正確なフィールド名を確認する。フィールド名を推測しない（PascalCase が多いが一致しないことがある）。

```sh
jquants schema                  # 全エンドポイント一覧（キー・説明・フィールド数）
jquants schema eq.daily         # eq.daily の全フィールド（名前・型・説明）
jquants --output json schema    # JSON 形式で出力
```

エンドポイントキーは `カテゴリ.コマンド` 形式（例: `eq.daily`, `fins.summary`, `mkt.short-ratio`）。

**手順:**
1. `jquants schema eq.daily` でフィールド一覧を取得
2. 必要なフィールド名をコピーして `-f` に指定
3. 不正なフィールド名はエラーで拒否される（利用可能フィールド一覧が表示される）

## Quick Reference

### 認証 & ユーティリティ
| やりたいこと | コマンド |
|---|---|
| ログイン（初回セットアップ） | `jquants login` |
| ログアウト | `jquants logout` |
| API フィールド一覧を確認 | `jquants schema` |
| 特定エンドポイントのフィールド詳細 | `jquants schema eq.daily` |

### 株式データ（eq）
| やりたいこと | コマンド | 更新時刻 |
|---|---|---|
| 全銘柄マスタ | `jquants eq master` | 日次 17:30頃 |
| 株価四本値 | `jquants eq daily` | 日次 16:30頃 |
| 前場四本値 | `jquants eq am` | 日次 12:00頃 |
| 分足データ | `jquants eq minute` | 日次 16:30頃 |
| 投資部門別売買状況 | `jquants eq investor-types` | 週次(木) 18:00頃 |
| 決算発表予定日 | `jquants eq earnings-calendar` | 不定期 19:00頃 |
| 株価ティック | `jquants eq trades` | 日次 16:30頃 |

### 市場データ（mkt）
| やりたいこと | コマンド | 更新時刻 |
|---|---|---|
| 売買内訳 | `jquants mkt breakdown` | 日次 18:00頃 |
| 日々公表信用残 | `jquants mkt margin-alert` | 日次 16:30頃 |
| 信用週末残高 | `jquants mkt margin-interest` | 週次(火) 16:30頃 |
| 取引カレンダー | `jquants mkt calendar` | 不定期 |
| 業種別空売り比率 | `jquants mkt short-ratio` | 日次 16:30頃 |
| 空売り残高報告 | `jquants mkt short-sale-report` | 日次 17:30頃 |

### デリバティブ（deriv）
| やりたいこと | コマンド | 更新時刻 |
|---|---|---|
| 先物四本値 | `jquants deriv futures` | 日次 27:00頃 |
| オプション四本値 | `jquants deriv options` | 日次 27:00頃 |
| 日経225オプション | `jquants deriv options-225` | 日次 27:00頃 |

### 財務データ（fins）
| やりたいこと | コマンド | 更新時刻 |
|---|---|---|
| 財務諸表 | `jquants fins details` | 日次 18:00頃(速報) / 24:30頃(確報) |
| 配当金情報 | `jquants fins dividend` | 日次 12〜19時頃 |
| 財務情報サマリー | `jquants fins summary` | 日次 18:00頃(速報) / 24:30頃(確報) |

### 指数データ（idx）
| やりたいこと | コマンド | 更新時刻 |
|---|---|---|
| TOPIX指数四本値 | `jquants idx daily-topix` | 日次 16:30頃 |
| 指数四本値 | `jquants idx daily` | 日次 16:30頃 |

## Category Reference Files

各カテゴリの詳細なコマンド構文・フラグ・例はリファレンスファイルに記載されている。**Quick Reference の情報で十分な場合はリファレンスを読む必要はない。**

以下の場合にリファレンスを参照する:

| カテゴリ | リファレンス | 参照すべき場面 |
|---|---|---|
| eq | references/commands-eq.md | 銘柄コード・日付・期間フィルタ、`eq trades` の特殊動作 |
| mkt | references/commands-mkt.md | `--s33`, `--disc-date`/`--calc-date` 等の特殊フラグ |
| deriv | references/commands-deriv.md | `--category`, `--contract-flag` の使い方 |
| fins | references/commands-fins.md | `fins details` の FS フィールド（JSON出力必須） |
| idx | references/commands-idx.md | 指数コード指定・TOPIX |
| bulk | references/commands-bulk.md | バルクDLの判断基準・ワークフロー・エンドポイント一覧 |
| plans | references/plans.md | プラン別 API 可否・取得可能期間・アドオン情報 |
| schedule | references/data-update-schedule.md | データの鮮度判断・更新タイミングの詳細・シナリオ別の判断例が必要なとき |

## Output, Fields, and Save

```sh
# 出力形式: table（デフォルト）、json、csv、parquet
jquants --output json eq master
jquants --output csv eq master
jquants --output parquet --save master.parquet eq master   # Parquet は --save 必須

# ファイル保存（--save は --output table と併用不可）
jquants --output csv --save master.csv eq master
jquants --output json --save daily.json eq daily --code 86970

# フィールド選択: -f でカラムを絞り込み（全形式で有効）
jquants -f Date,Code,AdjustmentClose eq daily --code 86970
jquants --output csv -f Date,Code,AdjustmentClose --save daily.csv eq daily --code 86970

# パイプでデータを加工
jquants eq daily --code 86970 | head -5                         # 自動CSV
jquants --output json eq daily --code 86970 | jq '.[0]'        # JSON明示
```

## Workflow Recipes

### 特定銘柄の株価トレンド分析

```sh
# 1. 銘柄コードを確認
jquants eq master --code 72030

# 2. フィールド名を確認
jquants schema eq.daily

# 3. 過去3ヶ月の日次データを CSV で保存
jquants --output csv --save toyota.csv \
  -f Date,AdjustmentClose,Volume \
  eq daily --code 72030 --from 2026-01-01 --to 2026-03-31
```

### 全銘柄の月次データ取得（バルク）

```sh
# 個別APIでループしない。全銘柄は bulk を使う。
jquants bulk get --endpoint /equities/bars/daily --date 2026-03 --download

# 解凍して CSV として利用
gunzip *.gz
```

### 財務情報の取得

```sh
# 配当情報（期間指定）
jquants --output csv --save div.csv fins dividend --from 2026-01-01 --to 2026-03-31

# 財務サマリー（銘柄指定、JSON推奨）
jquants --output json fins summary --code 86970

# 財務諸表の完全データ（FS フィールドは JSON 必須）
jquants --output json fins details --code 86970
```

## Special Behaviors

### eq trades はバルク API を内部的に使用する

`eq trades` は他の `eq` コマンドと異なり、内部で `/equities/trades` のバルク API を呼び出す。そのため:
- `--code` フラグは使用不可
- `--date YYYY-MM` 形式の月指定が可能（個別 API の `YYYY-MM-DD` とは異なる）
- `--download` フラグでファイルをカレントディレクトリに保存する

```sh
jquants eq trades --date 2025-12             # URL取得のみ
jquants eq trades --date 2025-12 --download  # ファイルダウンロード
```

### fins details の FS フィールド

`fins details` の FS（Financial Statement）フィールドはネストされた JSON オブジェクト。テーブル表示では `"N items"` と省略される。**完全なデータを取得するには `--output json` を使う。**

### deriv の --contract-flag

`--contract-flag 1` は中心限月（最も取引が活発な限月）のみを取得する。省略すると全限月のデータが返る。`deriv futures` と `deriv options` の両方で使用可能。

## Rate Limits

J-Quants API はプランごとに1分あたりのリクエスト上限が設定されている。上限を意識してコマンドを設計することで、不要な遮断を避けられる。

### プラン別上限（リクエスト/分）

| プラン | 上限 |
|--------|------|
| Free | 5 |
| Light | 60 |
| Standard | 120 |
| Premium | 500 |

### アドオン別上限（プランとは独立して適用）

| アドオン | 上限 |
|----------|------|
| 株価 分足・ティック | 60 |

### 制限超過時の挙動

- HTTP `429 Too Many Requests` が返される
- 大幅超過が続くと**約5分間**アクセスが完全遮断される（即時リトライは逆効果）

### 回避のポイント

- **全銘柄 × 全日付の個別ループは使わない** — `bulk get` で1リクエスト・全銘柄を一括取得する
- **429 を受けたら即時リトライしない** — しばらく待機（数十秒〜数分）してから再試行する
- **クエリパラメータで絞り込む** — 不要なフィールドや日付範囲を減らし、1リクエストで必要なデータを取り切る

## Subscription Plans & API Availability

各 API はプランによってアクセス可否と取得可能期間が異なる。コマンドを提案する前にプランの制限を確認する。プランが不明な場合は、コマンドを提示しつつ必要プランを明記する。

### 最低必要プラン早見表

| 最低必要プラン | CLI コマンド |
|---|---|
| **Free** | `eq master`, `eq daily`, `eq earnings-calendar`, `fins summary`, `mkt calendar` |
| **Light** | `eq investor-types`, `idx daily-topix` |
| **Standard** | `mkt margin-interest`, `mkt short-ratio`, `mkt short-sale-report`, `mkt margin-alert`, `idx daily`, `deriv options-225` |
| **Premium** | `eq am`, `fins details`, `fins dividend`, `mkt breakdown`, `deriv futures`, `deriv options` |
| **Add-on** | `eq minute`, `eq trades`（分足・ティック専用アドオン契約が必要） |

> **プラン階層:** Free < Light < Standard < Premium（上位プランは下位の全 API を含む）

> **Free プランの制限:** CSV/バルクダウンロード不可（`mkt calendar` を除く）。API アクセスのみ。データは「12週間前〜2年12週間前」のウィンドウに限定（最新12週と2年超の過去データは取得不可）。

詳細な取得可能期間・データ範囲は **references/plans.md** を参照。

## Data Freshness（更新タイミング）

J-Quants API のデータは市場イベント後に順次更新される。**現在時刻と更新時刻の関係**によって取得できるデータの鮮度が変わる。

### 時間帯別更新サマリー（JST）

| 更新時刻目安 | 反映されるデータ |
|---|---|
| 〜12:00 | 前場四本値（`eq am`）が当日分に更新 |
| 〜16:30 | 株価四本値・分足・ティック・指数・空売り比率・信用残（日次）が当日分に更新 |
| 〜17:30 | 銘柄マスタ・空売り残高報告が更新 |
| 〜18:00 | 売買内訳・投資部門別（週次）・財務情報（速報）が更新 |
| 〜19:00 | 決算発表予定日が更新（不定期・JPX連動） |
| 〜24:30 | 財務情報（確報）・財務諸表（確報）が更新 |
| 〜27:00 | デリバティブ（先物・オプション）が当日分に更新（27:00 = 翌日3:00 AM） |

### エージェント判断の指針

- **更新時刻前に当日データを聞かれた場合:** 「○○:○○頃に当日分が反映されます。現時点では前営業日のデータが最新です」と伝える
- **週次データ（信用週末残高・投資部門別）:** 信用残は火曜16:30、投資部門別は木曜18:00が更新目安。それ以外の曜日は前週分が最新
- **デリバティブは27:00（翌日3:00 AM）更新:** 翌営業日の早朝にならないと前日分が揃わない
- **「データが古い」と言われたとき:** 更新時刻・週次サイクル・休祝日・プランの期間制限を順に確認する

詳細な判断フローとシナリオ例は **references/data-update-schedule.md** を参照。

## Common Pitfalls

| 誤り | 正しい書き方 | なぜ |
|---|---|---|
| `eq daily --code X --output csv` | `--output csv eq daily --code X` | clap がサブコマンド後のフラグをサブコマンドのオプションとして解析するため |
| `--save out.csv eq daily`（output 未指定） | `--output csv --save out.csv eq daily` | table 形式は `--save` 非対応。保存には csv/json/parquet を明示する |
| `mkt short-ratio --code 0050` | `mkt short-ratio --s33 0050` | short-ratio は銘柄ではなく33業種分類でフィルタする API 設計のため |
| `mkt short-sale-report --date 2024-08-01` | `--disc-date` または `--calc-date` を使う | 公表日と計算日は別概念。`--date` フラグは存在しない |
| `eq daily --date 2026-03`（月のみ） | `--date` は YYYY-MM-DD 形式 | 月指定は bulk 系のみ対応。個別 API は日単位 |
| 全銘柄ループで `eq daily --code X` を繰り返す | `bulk get --endpoint /equities/bars/daily --date YYYY-MM --download` | API rate limit 回避 & 効率。1ファイルで全銘柄取得可能 |
| `--output parquet eq daily`（`--save` 未指定） | `--output parquet --save out.parquet eq daily` | Parquet はストリーミング出力できないため、ファイル保存が必須 |
| `-f date,code`（小文字のフィールド名） | `-f Date,Code`（`jquants schema <endpoint>` で確認） | API フィールド名は PascalCase。大文字小文字を区別する |
| `fins details` でテーブル表示すると FS が `"N items"` になる | `--output json fins details` を使う | FS フィールドはネストされた JSON オブジェクト。テーブル表示では要約される |
| 429 を受けてすぐリトライする | 数十秒〜数分待機してから再試行する | 大幅超過が続くと約5分間遮断される。即時リトライは遮断を長引かせる |
| Free プランで `bulk get` や `--output csv` を使う | 個別 API（`eq daily --code X`）を使う、または上位プランへアップグレードを促す | Free プランは CSV/バルクダウンロード非対応（`mkt calendar` を除く）。API のみ利用可能 |
| Light プランで `idx daily` や `deriv options-225` を使う | Standard プランが必要である旨を伝える | Standard 以上が必要な API。プランとコマンドの対応は references/plans.md を参照 |
| 16:00に `eq daily` で「今日の株価」を取得して最新と思い込む | 「16:30頃に当日分が反映されます。現時点では前営業日のデータです」と伝える | 株価四本値は16:30頃に当日分が更新される。更新前は前営業日のデータが最新 |
| 月曜朝に `mkt margin-interest` を取得して先週分と思い込む | 前々週金曜時点のデータ。火曜16:30以降に前週金曜分が反映されると伝える | 信用週末残高は第2営業日（通常火曜）16:30頃に更新される週次データ |

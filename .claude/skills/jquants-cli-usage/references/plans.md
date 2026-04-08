# サブスクリプションプラン別 API 利用可否・データ取得範囲

情報ソース: https://jpx-jquants.com/ja/spec/data-spec

## プラン別 API 利用可否テーブル

`-` = 利用不可。`API` = API のみ（CSV/バルク不可）。`API/CSV` = API とバルクダウンロード両方可。

| CLI コマンド | データ名 | Free | Light | Standard | Premium |
|---|---|---|---|---|---|
| `eq master` | 上場銘柄一覧 | API: 12w〜2y12w | API/CSV: 5年前まで | API/CSV: 10年前まで | API/CSV: 20年前まで |
| `eq daily` | 株価四本値 | API: 12w〜2y12w | API/CSV: 5年前まで | API/CSV: 10年前まで | API/CSV: 20年前まで |
| `eq am` | 前場四本値 | - | - | - | API: 直近のみ |
| `eq investor-types` | 投資部門別情報 | - | API/CSV: 5年前まで | API/CSV: 10年前まで | API/CSV: 20年前まで |
| `eq earnings-calendar` | 決算発表予定日 | API: 直近のみ | API: 直近のみ | API: 直近のみ | API: 直近のみ |
| `fins summary` | 財務情報 | API: 12w〜2y12w | API/CSV: 5年前まで | API/CSV: 10年前まで | API/CSV: 20年前まで |
| `fins details` | 財務諸表（BS/PL/CF） | - | - | - | API/CSV: 20年前まで |
| `fins dividend` | 配当金情報 | - | - | - | API/CSV: 20年前まで |
| `mkt calendar` | 取引カレンダー | API/CSV: 12w〜2y12w | API/CSV: 翌年末〜5年前 | API/CSV: 翌年末〜10年前 | API/CSV: 翌年末〜20年前 |
| `mkt margin-interest` | 信用取引週末残高 | - | - | API/CSV: 10年前まで | API/CSV: 20年前まで |
| `mkt short-ratio` | 業種別空売り比率 | - | - | API/CSV: 10年前まで | API/CSV: 20年前まで |
| `mkt short-sale-report` | 空売り残高報告 | - | - | API/CSV: 10年前まで | API/CSV: 20年前まで |
| `mkt margin-alert` | 日々公表信用取引残高 | - | - | API/CSV: 10年前まで | API/CSV: 20年前まで |
| `mkt breakdown` | 売買内訳データ | - | - | - | API/CSV: 20年前まで |
| `idx daily-topix` | TOPIX 四本値 | - | API/CSV: 5年前まで | API/CSV: 10年前まで | API/CSV: 20年前まで |
| `idx daily` | 指数四本値 | - | - | API/CSV: 10年前まで | API/CSV: 20年前まで |
| `deriv options-225` | 日経225オプション四本値 | - | - | API/CSV: 10年前まで | API/CSV: 20年前まで |
| `deriv futures` | 先物四本値 | - | - | - | API/CSV: 20年前まで |
| `deriv options` | オプション四本値 | - | - | - | API/CSV: 20年前まで |

### データ範囲記法の凡例

- `12w〜2y12w` = 12週間前〜2年12週間前（ローリングウィンドウ。最新12週と2年12週より前は取得不可）
- `5年前まで` = 現在から5年前まで遡れる
- `翌年末〜5年前` = 取引カレンダーは未来（翌年末）まで参照可能
- `直近のみ` = 直近の最新データのみ（歴史データなし）

## アドオン（プランとは独立して契約）

`eq minute` と `eq trades` は通常プランとは別に「株価 分足・ティック」アドオンの契約が必要。

| CLI コマンド | データ名 | 取得方法 | 取得可能期間 |
|---|---|---|---|
| `eq minute` | 株価分足 | API/CSV | 2年前まで |
| `eq trades` | 株価ティック | CSV（バルク専用） | 2年前まで |

**アドオンのレートリミット:** 60 リクエスト/分（プランのレートリミットとは独立して適用）

## 最低必要プラン早見表

コマンドから必要プランを逆引きする。

| 最低必要プラン | CLI コマンド |
|---|---|
| **Free** | `eq master`, `eq daily`, `eq earnings-calendar`, `fins summary`, `mkt calendar` |
| **Light** | `eq investor-types`, `idx daily-topix` |
| **Standard** | `mkt margin-interest`, `mkt short-ratio`, `mkt short-sale-report`, `mkt margin-alert`, `idx daily`, `deriv options-225` |
| **Premium** | `eq am`, `fins details`, `fins dividend`, `mkt breakdown`, `deriv futures`, `deriv options` |
| **Add-on** | `eq minute`, `eq trades` |

**プラン階層:** Free < Light < Standard < Premium（上位プランは下位の全 API を含む）

## Free プランの制限事項

1. **CSV/バルクダウンロード不可** — `mkt calendar` を除き、CSV 形式でのデータ取得およびバルクダウンロード（`bulk get`）は利用不可。API（個別エンドポイント）のみ使用可能
2. **データウィンドウ制限** — 利用可能なデータは「12週間前〜2年12週間前」のローリングウィンドウに限定。最新12週のデータも、2年12週より古いデータも取得不可
3. **利用可能 API が限定** — `eq master`, `eq daily`, `eq earnings-calendar`, `fins summary`, `mkt calendar` の5コマンドのみ

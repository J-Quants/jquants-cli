# データ更新スケジュール（Data Update Schedule）

情報ソース: https://jpx-jquants.com/ja/spec/data-update

**注意:** 更新時刻は目安であり、利用者への事前通知なく変更される可能性がある。「〜頃」は数十分の前後を含む。

---

## 全エンドポイント更新スケジュール

| CLI コマンド | API エンドポイント | 更新頻度 | 更新時刻 | 備考 |
|---|---|---|---|---|
| `eq master` | `/equities/master` | 日次 | 17:30頃 / 翌営業日8:00頃 | 翌営業日の銘柄情報は17:30以降に取得可能。翌朝8:00に再更新あり |
| `eq daily` | `/equities/bars/daily` | 日次 | 16:30頃 | — |
| `eq am` | `/equities/bars/daily/am` | 日次 | 12:00頃 | 前場終了後に当日分が反映 |
| `eq minute` | `/equities/bars/minute` | 日次 | 16:30頃 | — |
| `eq trades` | `/equities/trades` | 日次 | 16:30頃 | — |
| `eq investor-types` | `/equities/investor-types` | 週次（第4営業日） | 18:00頃 | 通常は木曜日。祝日がある週は後ろ倒し |
| `eq earnings-calendar` | `/equities/earnings-calendar` | 不定期 | 19:00頃 | JPX公式ページ更新時のみ反映 |
| `fins summary` | `/fins/summary` | 日次 | 18:00頃(速報) / 24:30頃(確報) | 速報は決算公表直後、確報は正式データ |
| `fins details` | `/fins/details` | 日次 | 18:00頃(速報) / 24:30頃(確報) | fins summary と同じサイクル |
| `fins dividend` | `/fins/dividend` | 日次 | 12〜19時（毎時00分頃） | 内容に更新がない場合もある |
| `mkt calendar` | `/markets/calendar` | 不定期 | 不定期 | 毎年3月末頃に翌年分を追加更新 |
| `mkt margin-interest` | `/markets/margin-interest` | 週次（第2営業日） | 16:30頃 | 通常は火曜日。祝日がある週は後ろ倒し |
| `mkt short-ratio` | `/markets/short-ratio` | 日次 | 16:30頃 | — |
| `mkt short-sale-report` | `/markets/short-sale-report` | 日次 | 17:30頃 | — |
| `mkt margin-alert` | `/markets/margin-alert` | 日次 | 16:30頃 | — |
| `mkt breakdown` | `/markets/breakdown` | 日次 | 18:00頃 | — |
| `idx daily-topix` | `/indices/bars/daily/topix` | 日次 | 16:30頃 | — |
| `idx daily` | `/indices/bars/daily` | 日次 | 16:30頃 | — |
| `deriv options-225` | `/derivatives/bars/daily/options/225` | 日次 | 27:00頃 | 27:00 = 翌日3:00 AM（JST） |
| `deriv futures` | `/derivatives/bars/daily/futures` | 日次 | 27:00頃 | 27:00 = 翌日3:00 AM（JST） |
| `deriv options` | `/derivatives/bars/daily/options` | 日次 | 27:00頃 | 27:00 = 翌日3:00 AM（JST） |

---

## 時刻表記について

- 全時刻は **JST（日本標準時）**
- **「27:00」は翌日 3:00 AM** を意味する（夜間取引データを含むため翌日未明に更新される）
- **「速報」**: 決算短信公表後の初回データ（後から修正が入ることがある）
- **「確報」**: 正式データ。速報よりも正確だが遅い

---

## 更新頻度の分類

### 日次更新

市場の **営業日ごと** に更新される。休日・祝日は更新なし。

### 週次更新

| データ | 更新日 | 基準 |
|---|---|---|
| 信用取引週末残高（`mkt margin-interest`） | 第2営業日（通常 **火曜**） | 前週金曜日時点のデータ |
| 投資部門別売買状況（`eq investor-types`） | 第4営業日（通常 **木曜**） | 前週分のデータ |

週次データは更新日以外に取得しても前週分が最新のまま。

### 不定期更新

| データ | 更新契機 |
|---|---|
| 決算発表予定日（`eq earnings-calendar`） | JPX公式ページの更新に連動（不定期） |
| 取引カレンダー（`mkt calendar`） | 毎年3月末頃に翌年分を追加 |

---

## エージェント向け判断フロー

### 「今日のデータは取れるか？」の判断

```
現在時刻（JST）を確認
  ├─ 対象APIの更新時刻より前
  │    → 前営業日までのデータが最新
  │    → 「○○:○○頃に当日分が反映されます」とユーザーに伝える
  └─ 対象APIの更新時刻以降
       → 当日分を含むデータが取得可能
```

### 「データが古い・足りない」と言われたときの確認事項

1. **更新時刻前ではないか？** → 上記テーブルで対象 API の更新時刻を確認
2. **週次データではないか？** → 信用残は火曜、投資部門別は木曜にしか更新されない
3. **休日・祝日ではないか？** → `jquants mkt calendar --hol-div 1` で休場日を確認
4. **Free プランの期間制限ではないか？** → Free は「12週前〜2年12週前」のウィンドウのみ

---

## 具体的なシナリオ例

| 状況 | 判断 |
|---|---|
| 月曜15:00に `eq daily` を取得 | 前週金曜のデータが最新。当日分は **16:30以降** に反映 |
| 月曜10:00に `mkt margin-interest` を取得 | 前々週金曜時点のデータ。**火曜16:30以降** に前週金曜分が反映 |
| 火曜20:00に `fins summary` を取得 | 速報データは18:00に反映済み。確報は **24:30頃** |
| 水曜朝に `deriv futures` を取得 | 火曜のデータが最新（27:00=水曜3:00に更新済み） |
| 木曜19:00に `eq investor-types` を取得 | 第4営業日（木曜）18:00以降なので当日更新済み |
| 金曜16:00に `mkt short-sale-report` を取得 | 更新時刻（17:30）前なので **木曜分** が最新 |
| 月曜朝に `eq master` を取得 | 前週金曜の17:30または土曜8:00更新分が最新。月曜の新規情報は17:30以降 |

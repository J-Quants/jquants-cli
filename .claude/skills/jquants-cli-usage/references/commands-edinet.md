# Category: edinet (EDINET書類由来データ) — Command Reference

いずれも **Standard プラン以上**で利用可能（Free / Light では 403）。
過去データの参照範囲は Standard=10年 / Premium=20年。
CSV/Bulk ダウンロードは非対応（API 経由のみ）。

## 共通クエリ仕様（3コマンド共通）

- `--edinet-code` / `--code` / `--date` はすべて**任意指定**
- すべて省略した場合、**API 実行日に提出された全書類**のデータが返る
- `--edinet-code` と `--code` の**同時指定は不可**（CLI が実行前に拒否する）

```sh
jquants edinet major-shareholders                           # 実行日提出分の一覧
jquants edinet major-shareholders --code 86970              # 銘柄コード指定
jquants edinet cross-shareholdings --edinet-code E02367     # EDINETコード指定
jquants edinet large-volume-shareholders --date 2026-07-01  # 提出日指定
```

## ネスト構造の扱い（重要）

3コマンドともレスポンスにネスト項目を含む:

| コマンド | ネスト項目 | 内容 |
|---|---|---|
| major-shareholders | `Hldrs`（配列） | 大株主明細（Rank/HldrName/HldrAddr/ShsHeld/ShsRatio） |
| cross-shareholdings | `Report` / `Largest` / `SecondLargest`（オブジェクト） | 保有主体ブロック。内部に `Spec[]`（特定投資株式）/ `Deem[]`（みなし保有株式）の銘柄明細 |
| large-volume-shareholders | `Hldrs`（配列） | 提出者・共同保有者明細。内部に `AcqDisp[]` / `BrwList[]` / `CredList[]` |

- **テーブル表示**: ネストは `"N items"` / `"N keys"` に略される
- **JSON 出力**: 完全表示（`--output json` を推奨）
- **CSV 出力**: ネストは JSON 文字列として 1 セルに収められる

```sh
jquants --output json edinet major-shareholders --code 86970    # 完全表示
jquants --output csv --save mjr.csv edinet major-shareholders --date 2026-06-11
```

## edinet major-shareholders — 大株主状況

有価証券報告書（第三号様式）記載の大株主の状況。データ提供期間は 2016-06-01 以降。

- 大株主は通常上位10名だが、同順位タイで11件以上・100%子会社等で1件のみのケースあり
- `ShsRatio` は小数表現（0.1704 = 17.04%）

## edinet cross-shareholdings — 政策保有株式

有価証券報告書「株式の保有状況」記載の政策保有株式。データ提供期間は 2020-03-31 以降。

- 提出会社（`Report`）/ 連結最大保有会社（`Largest`）/ 連結第二最大保有会社（`SecondLargest`）の3スコープ
- 各スコープに上場/非上場別の集計と `Spec[]` / `Deem[]` の銘柄明細
- 本 API のデータは LLM によるデータ修正が行われている

## edinet large-volume-shareholders — 大量保有報告書

大量保有報告書・変更報告書（書類種別コード 350）の発行者・提出者情報。提出日 2021-07-01 以降。

- `LargeHldgTypeCode`: 1=大量保有報告書 / 2=変更報告書 / 3=変更報告書(短期大量譲渡) / 4・5=特例対象 / 0=不明
- `ChgRsn` / `TotalShsRatioLast` / `ShsRatioLast` は**変更報告書のみ**記載（それ以外は空）
- `--edinet-code` / `--code` は**発行者**（保有される側）のコードを指定する

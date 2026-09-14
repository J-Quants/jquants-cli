# Category: eq (Equities) — Command Reference

## eq master — 銘柄マスタ

```sh
jquants eq master                           # 全銘柄
jquants eq master --code 86970             # 銘柄コードでフィルタ
jquants eq master --date 2026-03-14        # 日付でフィルタ
```

## eq daily — 株価四本値

```sh
jquants eq daily --code 86970
jquants eq daily --date 2026-03-14
jquants eq daily --code 86970 --from 2026-03-01 --to 2026-03-31
```

## eq am — 前場四本値

```sh
jquants eq am                    # 全銘柄
jquants eq am --code 27800
```

## eq minute — 分足データ

```sh
jquants eq minute --code 27800
jquants eq minute --date 2026-03-14
jquants eq minute --code 27800 --from 2026-03-14 --to 2026-03-21
```

## eq valuation — バリュエーション指標

決算短信の開示内容と株価から算出した日次の指標（EPS / FwdEPS / BPS / ROE / FwdROE / PER / FwdPER / PBR / MktCap）。
`--code` または `--date` のいずれかの指定が必須。

```sh
jquants eq valuation --code 86970
jquants eq valuation --date 2026-03-14                        # 全上場銘柄の指定日
jquants eq valuation --code 86970 --from 2026-03-01 --to 2026-03-31
jquants -f Date,Code,PER,PBR eq valuation --code 86970
```

**注意点:**

- `ROE` / `FwdROE` は**小数**（`0.2310` = 23.1%）。パーセントではない
- `MktCap` は**百万円単位**。自己株式を控除した株式数×当日終値で算出するため、`eq daily` の `MktCap`（自己株式を含む）とは値が一致しない場合がある（`eq daily` の `MktCap` は削除予定）
- 実績値（EPS/ROE/PER）は直近12ヶ月（TTM）、予想値（Fwd系）は進行期予想にもとづく
- ETF・ETN・優先出資証券など算出対象外の銘柄も行は返るが全指標が Null。REIT 等は指標が Null でも `MktCap` に値が入る場合がある
- 収録開始当初（2008〜2010年頃）は Null となる銘柄・項目が多い

## eq investor-types — 投資部門別売買状況

オプション `--section`: `TSEPrime`, `TSEStandard`, `TSEGrowth` など

```sh
jquants eq investor-types
jquants eq investor-types --section TSEPrime
jquants eq investor-types --from 2021-09-01 --to 2021-09-07
```

## eq earnings-calendar — 決算発表予定日

```sh
jquants eq earnings-calendar
```

## eq trades — 株価ティック

**注意: 内部的にバルク API を使用する特殊コマンド**

- `--code` フラグは使用不可（全銘柄一括取得のみ）
- `--date` は `YYYY-MM` 形式（月指定）が可能（他の eq コマンドの `YYYY-MM-DD` と異なる）
- `--download` でファイルをカレントディレクトリに保存する

```sh
jquants eq trades --date 2025-12             # Presigned URL取得のみ
jquants eq trades --date 2025-12 --download  # ファイルダウンロード
```

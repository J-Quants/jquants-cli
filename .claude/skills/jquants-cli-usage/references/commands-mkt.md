# Category: mkt (Markets) — Command Reference

## mkt breakdown — 売買内訳

```sh
jquants mkt breakdown --code 27800
jquants mkt breakdown --date 2021-09-01
jquants mkt breakdown --code 27800 --from 2021-09-01 --to 2021-09-30
```

## mkt margin-alert — 日々公表信用取引残高

```sh
jquants mkt margin-alert --code 13260
jquants mkt margin-alert --date 2024-02-08
jquants mkt margin-alert --from 2024-02-01 --to 2024-02-28
```

## mkt margin-interest — 信用取引週末残高

```sh
jquants mkt margin-interest --code 27800
jquants mkt margin-interest --date 2021-09-01
jquants mkt margin-interest --from 2021-09-01 --to 2021-09-30
```

## mkt calendar — 取引カレンダー

オプション `--hol-div`: `1` = 休日, `0` = 営業日

```sh
jquants mkt calendar
jquants mkt calendar --hol-div 1
jquants mkt calendar --from 2026-03-01 --to 2026-03-14
```

## mkt short-ratio — 業種別空売り比率

**注意: `--code` ではなく `--s33`（業種コード33分類）を使う**

```sh
jquants mkt short-ratio --s33 0050          # 業種コードでフィルタ
jquants mkt short-ratio --date 2022-10-25
jquants mkt short-ratio --s33 0050 --from 2022-10-01 --to 2022-10-31
```

## mkt short-sale-report — 空売り残高報告

**注意: `--date` ではなく `--disc-date`（公表日）または `--calc-date`（計算日）を使う**

- `--disc-date`: 空売り残高が公的に開示された日付（公表日）
- `--calc-date`: 残高を算出した日付（計算日）。通常は公表日の1〜2営業日前

```sh
jquants mkt short-sale-report --code 13660
jquants mkt short-sale-report --disc-date 2024-08-01                           # 公表日（単日）
jquants mkt short-sale-report --disc-date-from 2024-08-01 --disc-date-to 2024-08-31  # 公表日（期間）
jquants mkt short-sale-report --calc-date 2024-07-31                           # 計算日
```

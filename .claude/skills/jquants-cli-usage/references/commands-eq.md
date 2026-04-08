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

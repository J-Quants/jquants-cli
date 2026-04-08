# Category: deriv (Derivatives) — Command Reference

## deriv options-225 — 日経225オプション四本値

```sh
jquants deriv options-225 --date 2021-09-01
```

## deriv futures — 先物四本値

```sh
jquants deriv futures --date 2021-09-01
jquants deriv futures --date 2021-09-01 --category TOPIXF
jquants deriv futures --date 2021-09-01 --contract-flag 1   # 中心限月のみ
```

## deriv options — オプション四本値

```sh
jquants deriv options --date 2021-09-01
jquants deriv options --date 2021-09-01 --category TOPIXE
jquants deriv options --date 2021-09-01 --code 86970        # 原資産コードでフィルタ
jquants deriv options --date 2021-09-01 --contract-flag 1   # 中心限月のみ
```

`--category` の例: `TOPIXF`（先物）、`TOPIXE`（オプション）

`--contract-flag 1`: 中心限月（最も取引が活発な限月）のみ取得する。省略すると全限月のデータが返る。`futures` と `options` の両方で使用可能。

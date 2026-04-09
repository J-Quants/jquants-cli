# Security Policy

## サポートバージョン

| バージョン | サポート状態 |
| ---------- | ------------ |
| 最新リリース | サポート中 |
| それ以前 | サポート外 |

## 脆弱性の報告

セキュリティ上の問題を発見した場合は、**公開 Issue には投稿しないでください**。

### 報告方法

GitHub の **Private vulnerability reporting** を使用してください:

1. [Security タブ](https://github.com/J-Quants/jquants-cli/security) を開く
2. "Report a vulnerability" をクリック
3. 詳細を記入して送信

### 報告内容

報告時に以下の情報をご提供ください:

- 脆弱性の種類と影響範囲
- 再現手順
- 影響を受けるバージョン
- 可能であれば、修正案や回避策

### 対応フロー

1. 報告受領後、3営業日以内に確認の連絡をします
2. 調査・修正後、修正バージョンをリリースします
3. リリース後、[GitHub Security Advisories](https://github.com/J-Quants/jquants-cli/security/advisories) で詳細を公開します

## ビルドの完全性検証

リリースバイナリは [SLSA](https://slsa.dev/) provenance attestation により、GitHub Actions 上でビルドされたことを暗号学的に証明しています。

```sh
gh attestation verify <downloaded-binary> --repo J-Quants/jquants-cli
```

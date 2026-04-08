# Contributing

コントリビューションを歓迎しています。バグ報告・機能要望・ドキュメント改善・コードの改修など、あらゆる形での貢献を受け付けています。

## Issue の報告

バグ報告や機能要望は [GitHub Issues](https://github.com/J-Quants/jquants-cli/issues) から提出してください。

- **バグ報告**: 提出前に既存の Issue を検索し、重複がないか確認してください。再現手順・期待する動作・実際の動作・環境情報（OS・Rust バージョン）を記載してください。
- **機能要望**: 解決したい課題と、提案する機能の概要を説明してください。

## 開発環境のセットアップ

1. [Rust](https://rustup.rs/)（stable toolchain、MSRV 1.78 以上）をインストールする
2. GitHub でリポジトリを **Fork** する（右上の "Fork" ボタン）
3. Fork したリポジトリをクローンする:
   ```sh
   git clone https://github.com/<your-username>/jquants-cli
   cd jquants-cli
   ```
4. オリジナルリポジトリを `upstream` として追加する:
   ```sh
   git remote add upstream https://github.com/J-Quants/jquants-cli
   ```

## ビルドとテスト

```sh
cargo build              # デバッグビルド
cargo build --release    # リリースビルド

cargo test               # テスト実行
cargo fmt                # コードフォーマット
cargo fmt --check        # フォーマットチェック（CI 用）
cargo clippy -- -D warnings  # lint
```

## コントリビューションの流れ

1. **upstream と同期する**（作業前に必ず実施）:
   ```sh
   git checkout main
   git fetch upstream
   git rebase upstream/main
   ```
2. **フィーチャーブランチを作成する**:
   ```sh
   git checkout -b feature/add-new-endpoint   # 新機能
   git checkout -b fix/fix-pagination-bug     # バグ修正
   ```
3. **変更を加える**（テストも合わせて更新してください）
4. **CI チェックをローカルで確認する**:
   ```sh
   cargo fmt --check && cargo clippy -- -D warnings && cargo test
   ```
5. **Fork 先にプッシュして PR を提出する**:
   ```sh
   git push origin feature/add-new-endpoint
   ```
   GitHub 上で `J-Quants/jquants-cli` の `main` ブランチへの Pull Request を作成してください。

## プルリクエストのガイドライン

- 1 つの PR は 1 つの変更・目的に絞ってください
- PR の説明には「何を・なぜ変更したか」を明記してください
- 関連する Issue がある場合は `Closes #123` のように参照してください
- すべての CI チェック（fmt / clippy / test）がパスしていることを確認してください
- レビューコメントには丁寧に対応し、議論が必要な場合は PR 上で行ってください

## コーディング規約

- **フォーマット**: `cargo fmt` を実行し、スタイルを統一してください
- **Lint**: `cargo clippy -- -D warnings` の警告をすべて解消してください
- **テスト**: 新機能・バグ修正には対応するテストを追加してください
- **コミットメッセージ**: Conventional Commits 形式（`feat:`, `fix:`, `chore:`, `docs:` 等）で記述してください

## プロジェクト構成

| ファイル | 役割 |
|----------|------|
| `src/main.rs` | エントリポイント |
| `src/lib.rs` | クレート構造定義（pub mod 宣言） |
| `src/cli.rs` | CLI コマンド定義（clap derive） |
| `src/config.rs` | 設定・認証ヘッダー解決 |
| `src/auth.rs` | Cognito OAuth2 PKCE |
| `src/client.rs` | J-Quants API クライアント |
| `src/models.rs` | レスポンス型定義 |
| `src/schema.rs` | エンドポイントごとのスキーマ定義 |
| `src/output.rs` | テーブル・JSON・CSV・Parquet 出力 |
| `src/error.rs` | エラー型 |
| `src/download.rs` | バルクダウンロード処理 |

## ライセンス

このリポジトリは [MIT License](LICENSE) で公開されています。Pull Request を提出することで、あなたの変更が同ライセンスのもとで配布されることに同意したものとみなします。

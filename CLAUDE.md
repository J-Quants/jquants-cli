# J-Quants CLI

J-Quants API V2 の CLI ツール（バイナリ名: `jquants`）

## Commands

```sh
cargo build                                                      # ビルド
cargo test                                                       # テスト実行
cargo fmt                                                        # フォーマット
cargo clippy -- -D warnings                                      # lint（警告ゼロ必須）
cargo fmt --check && cargo clippy -- -D warnings && cargo test   # ローカルCI一括
```

## Code Style

- import順序: `crate::` → 外部crate → `std::`（Rust標準と逆）
- エラー処理は `?` 演算子で `AppError` に統一。不変条件の `expect` は許容するが最小限に。テスト内の `unwrap()` は可
- 日本語コメント使用。セクション区切りに `// ── セクション名 ──…` を使用
- コードスタイルは `cargo fmt` + `cargo clippy -- -D warnings` で強制

## Testing

- テスト命名: `test_<what_is_being_tested>`
- env var を操作するテストでは `ENV_MUTEX.lock().unwrap()` で排他制御必須（`config.rs`, `auth.rs` にそれぞれ別の `static` 定義あり。同一モジュール内の直列化のみ保証。複数モジュールで同じ env var を操作する場合は注意）
- async テストは `#[tokio::test]` を使用
- モック不使用。`models.rs` のデシリアライズテストで API レスポンス形式を検証

## Git Workflow

- ブランチ命名: `feature/<説明>`, `fix/<説明>`
- コミット: Conventional Commits 形式（`feat:`, `fix:`, `chore:` 等）
- CI: push to main / PR で `fmt --check` → `clippy` → `test` が自動実行
- 詳細は CONTRIBUTING.md を参照

## Architecture

```
main.rs      -- エントリポイント。run() で .env ロード → Skills/Completions/Schema/Login/Logout を早期 return → Config読込 → Client生成 → サブコマンド実行
lib.rs       -- pub mod 宣言によるクレート構造定義
cli.rs       -- clap derive による CLI 定義
config.rs    -- 環境変数から Config 構築（dotenvy で .env 対応）
auth.rs      -- Cognito OAuth2 PKCE ログイン・ログアウト・API キー管理
client.rs    -- reqwest ベースの API クライアント。ページネーション自動処理
models.rs    -- serde による API レスポンスモデル定義
schema.rs    -- エンドポイントごとのスキーマ定義（SchemaInfo trait, FieldSchema, EndpointSchema）
error.rs     -- thiserror による AppError。hint()/why() で構造化エラーメッセージ
output.rs    -- TableDisplay trait による出力。TTY 検出でパイプ時 CSV 自動切替。Parquet 出力対応（--save 必須）
download.rs  -- バルクダウンロード処理
```

## 注意事項

- `models.rs` の `FlexString` 型: API が数値/文字列/null を混在して返すフィールドに使用（`String` や `f64` ではなく `FlexString` を選ぶべきケースに注意）
- `serde_json` の `preserve_order` feature: フィールド順序保持が `--fields` 選択に必要。削除禁止
- `include_dir!` マクロで `.claude/skills/` をコンパイル時に埋め込み。Skills ファイルの変更は再ビルドが必要
- 新エンドポイント追加時は `schema.rs` にスキーマ登録必須（`test_field_count_consistency` 等の整合性テストで検出）
- Parquet 出力（`--output parquet`）は `--save` フラグ必須
- バージョン `0.0.0-dev` はリリース CI（`v*` タグ）で git tag から自動差し替え
- `reqwest` は `rustls-tls` を使用（OpenSSL 不要）
- J-Quants API は HTTP 210 を「空の成功」として返す（`client.rs` で処理済み）
- 認証優先順: `~/.config/jquants/credentials.json` → `JQUANTS_API_KEY` 環境変数 → エラー
- `credentials.json` や API キーをリポジトリにコミットしないこと（`.env` は `.gitignore` 済み）

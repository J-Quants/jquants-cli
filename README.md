**Language:** 日本語 | [English](docs/en-US/README.md)

# jquants-cli

[![CI](https://github.com/J-Quants/jquants-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/J-Quants/jquants-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)

[J-Quants API V2](https://jpx-jquants.com/spec) を利用して日本株式市場のデータを取得できる CLI ツールです。

## 特徴

- J-Quants API V2 の全エンドポイントに対応（株式・市場・財務・指数・デリバティブ・バルクダウンロード）
- テーブル / JSON / CSV / Parquet の4種類の出力フォーマット
- パイプ接続時に自動で CSV 形式に切替（TTY 検出）
- OAuth2 PKCE によるセキュアなブラウザログイン
- 環境変数・`.env` ファイルによる API Key 管理
- シェル補完スクリプト生成（bash / zsh / fish / powershell）
- AI Agent 向け Skills ファイル対応
- macOS / Linux / Windows クロスプラットフォーム対応

## 前提条件

- J-Quants API V2 を利用するには、[こちら](https://jpx-jquants.com/register)よりアカウント登録が必要です
- データ取得には Free / Light / Standard / Premium のいずれかのプランを選択してください

## インストール

### Homebrew (macOS / Linux)

```sh
brew install J-Quants/tap/jquants
```

### GitHub Releases

[Releases ページ](https://github.com/J-Quants/jquants-cli/releases)から各プラットフォーム向けのビルド済みバイナリをダウンロードし、`PATH` の通ったディレクトリに配置してください。

| OS | アーキテクチャ | ファイル |
|---|---|---|
| macOS | Intel (x86_64) | `jquants-{version}-x86_64-apple-darwin.tar.gz` |
| macOS | Apple Silicon (ARM64) | `jquants-{version}-aarch64-apple-darwin.tar.gz` |
| Linux | x86_64 (musl) | `jquants-{version}-x86_64-unknown-linux-musl.tar.gz` |
| Linux | ARM64 (musl) | `jquants-{version}-aarch64-unknown-linux-musl.tar.gz` |
| Windows | x86_64 | `jquants-{version}-x86_64-pc-windows-msvc.zip` |

### ソースからビルド

[Rust](https://rustup.rs/)（stable、MSRV 1.78 以上）が必要です。

```sh
git clone https://github.com/J-Quants/jquants-cli
cd jquants-cli
cargo install --path .
```

## 認証

### 推奨: OAuth2 ブラウザログイン

```sh
jquants login
```

実行するとブラウザが自動で開き、J-Quants アカウントでログインすると API Key が `~/.config/jquants/credentials.json` に保存されます。

### API Key を直接指定

環境変数または `.env` ファイルで設定できます。

```sh
export JQUANTS_API_KEY=your_api_key_here
```

```sh
# .env ファイル
JQUANTS_API_KEY=your_api_key_here
```

API Key は [J-Quants Dashboard](https://jpx-jquants.com/dashboard/api-keys) から取得してください。

### ログアウト

```sh
jquants logout
```

ブラウザでセッションをクリアし、`~/.config/jquants/credentials.json` を削除します。

## 使い方

詳しい使い方ガイドは [J-Quants CLI](https://jpx-jquants.com/spec/jquants-cli) を参照してください。

### シェル補完

```sh
# bash
mkdir -p ~/.config/bash/completions
jquants completion bash > ~/.config/bash/completions/jquants.bash
source ~/.config/bash/completions/jquants.bash
# ~/.bashrc に追記して永続化
echo "source ~/.config/bash/completions/jquants.bash" >> ~/.bashrc

# zsh
mkdir -p ~/.zfunc
jquants completion zsh > ~/.zfunc/_jquants
# ~/.zshrc に追記して永続化
echo "fpath=(~/.zfunc $fpath)" >> ~/.zshrc
echo "autoload -Uz compinit && compinit" >> ~/.zshrc

# fish
mkdir -p ~/.config/fish/completions
jquants completion fish > ~/.config/fish/completions/jquants.fish
source ~/.config/fish/completions/jquants.fish
```

## AI Agent 連携

本ツールには AI Agent（Claude Code 等）向けの Skills ファイルが同梱されています。以下のコマンドでインストールしてください。

```sh
npx skills add J-Quants/jquants-cli
```

または、CLI から親ディレクトリを指定してインストールすることもできます。指定したディレクトリの直下に `jquants-cli-usage/` が作成されます。

```sh
# .claude/skills/jquants-cli-usage/ が作成される
jquants skills add --dir .claude/skills
```

## Contributing

コントリビューションを歓迎しています。詳細は [CONTRIBUTING.md](CONTRIBUTING.md) をご参照ください。

## License

MIT — Copyright © JPX Market Innovation and Research, Inc.

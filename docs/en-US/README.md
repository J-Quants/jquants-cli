**Language:** [日本語](../../README.md) | English

# jquants-cli

[![CI](https://github.com/J-Quants/jquants-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/J-Quants/jquants-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)

A CLI tool for querying the [J-Quants API V2](https://jpx-jquants.com/spec) to access Japanese stock market data.

## Features

- Full coverage of J-Quants API V2 endpoints (equities, markets, financials, indices, derivatives, bulk downloads)
- Four output formats: table / JSON / CSV / Parquet
- Automatic CSV output when piped (TTY detection)
- Secure browser-based login via OAuth2 PKCE
- API Key management via environment variables or `.env` files
- Shell completion scripts (bash / zsh / fish / powershell)
- AI Agent Skills file support
- Cross-platform: macOS / Linux / Windows

## Prerequisites

- An account is required to use J-Quants API V2. Register [here](https://jpx-jquants.com/register).
- Select one of the available plans (Free, Light, Standard, Premium) to access data.

## Installation

### Homebrew (macOS / Linux)

```sh
brew install J-Quants/tap/jquants
```

### GitHub Releases

Download a pre-built binary for your platform from the [Releases page](https://github.com/J-Quants/jquants-cli/releases) and place it somewhere on your `PATH`.

| OS | Architecture | File |
|---|---|---|
| macOS | Intel (x86_64) | `jquants-{version}-x86_64-apple-darwin.tar.gz` |
| macOS | Apple Silicon (ARM64) | `jquants-{version}-aarch64-apple-darwin.tar.gz` |
| Linux | x86_64 (musl) | `jquants-{version}-x86_64-unknown-linux-musl.tar.gz` |
| Linux | ARM64 (musl) | `jquants-{version}-aarch64-unknown-linux-musl.tar.gz` |
| Windows | x86_64 | `jquants-{version}-x86_64-pc-windows-msvc.zip` |

### Build from Source

Requires [Rust](https://rustup.rs/) (stable toolchain, MSRV 1.78+).

```sh
git clone https://github.com/J-Quants/jquants-cli
cd jquants-cli
cargo install --path .
```

## Authentication

### Recommended: OAuth2 Browser Login

```sh
jquants login
```

This opens a browser window. After logging in with your J-Quants account, your API Key is saved to `~/.config/jquants/credentials.json`.

### API Key

Set via environment variable or a `.env` file.

```sh
export JQUANTS_API_KEY=your_api_key_here
```

```sh
# .env file
JQUANTS_API_KEY=your_api_key_here
```

Retrieve your API Key from the [J-Quants Dashboard](https://jpx-jquants.com/dashboard/api-keys).

### Logout

```sh
jquants logout
```

Clears the Cognito session in the browser and removes `~/.config/jquants/credentials.json`.

## Usage

For a detailed usage guide, see [J-Quants CLI](https://jpx-jquants.com/spec/jquants-cli).

### Output Formats

Use the `--output` (`-o`) flag to choose the output format. This flag must be placed **before** the subcommand.

```sh
jquants eq daily --code 86970                               # table (default)
jquants --output json eq daily --code 86970                 # JSON (all fields)
jquants --output csv eq master                              # CSV
jquants --output csv --save out.csv eq master               # save CSV to file
jquants --output parquet --save out.parquet eq daily --code 86970  # save Parquet to file
```

**Automatic CSV when piped**: When stdout is connected to a pipe (e.g. `| head`), the output automatically switches to CSV format even with `--output table`.

```sh
jquants eq master | head -5
jquants eq master | awk -F, '{print $3}'
```

> `--output table` cannot be used with `--save`. File saving is only supported for JSON and CSV formats.
> `--output parquet` requires `--save`.

### Field Selection

Use the `-f` / `--fields` flag to select specific output fields (comma-separated API field names). Field names match the JSON/CSV/Parquet keys, not the abbreviated table headers.

```sh
jquants -f Date,Code,AdjC eq daily --code 86970
jquants --output csv -f Date,Open,High,Low,Close,Volume eq daily --code 86970
```

### Command Reference

#### eq — Equities

| Subcommand | Description | Key Options |
|---|---|---|
| `eq master` | Stock master data | `--code`, `--date` |
| `eq daily` | Daily OHLCV bars (adjusted) | `--code`, `--date`, `--from`, `--to` |
| `eq am` | Morning session bars | `--code` |
| `eq minute` | Minute bars | `--code`, `--date`, `--from`, `--to` |
| `eq earnings-calendar` | Earnings announcement schedule | — |
| `eq investor-types` | Trading by investor type | `--section`, `--from`, `--to` |
| `eq trades` | Tick data (bulk download) | `--date`, `--download` |

```sh
jquants eq master --code 86970
jquants eq daily --code 86970 --from 2024-01-01 --to 2024-03-31
jquants --output json eq earnings-calendar
```

#### mkt — Markets

| Subcommand | Description | Key Options |
|---|---|---|
| `mkt breakdown` | Trading breakdown | `--code`, `--date`, `--from`, `--to` |
| `mkt margin-alert` | Daily margin alert | `--code`, `--date`, `--from`, `--to` |
| `mkt margin-interest` | Weekly margin interest | `--code`, `--date`, `--from`, `--to` |
| `mkt calendar` | Trading calendar | `--hol-div`, `--from`, `--to` |
| `mkt short-ratio` | Sector short-selling ratio | `--s33`, `--date`, `--from`, `--to` |
| `mkt short-sale-report` | Short sale report | `--code`, `--disc-date`, `--disc-date-from`, `--disc-date-to`, `--calc-date` |

#### fins — Financials

| Subcommand | Description | Key Options |
|---|---|---|
| `fins details` | Financial statements (BS/PL/CF) | `--code`, `--date` |
| `fins dividend` | Dividend data | `--code`, `--date`, `--from`, `--to` |
| `fins summary` | Financial summary | `--code`, `--date` |

> In table mode, the `fins details` FS column shows only the item count. Use `--output json` to retrieve the full financial statement data.

#### idx — Indices

| Subcommand | Description | Key Options |
|---|---|---|
| `idx daily-topix` | TOPIX daily bars | `--from`, `--to` |
| `idx daily` | Index daily bars | `--code`, `--date`, `--from`, `--to` |

#### deriv — Derivatives

| Subcommand | Description | Key Options |
|---|---|---|
| `deriv options-225` | Nikkei 225 options daily bars | `--date` |
| `deriv futures` | Futures daily bars | `--category`, `--date`, `--contract-flag` |
| `deriv options` | Options daily bars | `--category`, `--code`, `--date`, `--contract-flag` |

#### bulk — Bulk Download

| Subcommand | Description | Key Options |
|---|---|---|
| `bulk list` | List downloadable files | `--endpoint`, `--date`, `--from`, `--to` |
| `bulk get` | Get download URL or download file | `--key` or `--endpoint` + `--date`, `--download` |

```sh
jquants bulk list --endpoint /equities/bars/daily --date 2024-01-04
jquants bulk get --endpoint /equities/bars/daily --date 2024-01-04 --download
```

#### schema — API Schema

Display the API response schema for endpoints.

```sh
jquants schema                         # list all endpoints
jquants schema eq.daily                # field details for daily bars
jquants --output json schema eq.daily  # JSON output
```

### Shell Completion

```sh
# bash
jquants completion bash > ~/.config/bash/completions/jquants.bash

# zsh
jquants completion zsh > ~/.zfunc/_jquants
fpath=(~/.zfunc $fpath)
autoload -Uz compinit && compinit

# fish
jquants completion fish > ~/.config/fish/completions/jquants.fish
```

## AI Agent Integration

This tool ships with a Skills file for AI agents (e.g. Claude Code). Install it with:

```sh
npx skills add J-Quants/jquants-cli
```

Or install directly from the CLI by specifying the target directory. A `jquants-cli-usage/` directory will be created inside the specified path.

```sh
# creates .claude/skills/jquants-cli-usage/
jquants skills add --dir .claude/skills
```

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](../../CONTRIBUTING.md) for details.

## License

MIT — Copyright © JPX Market Innovation and Research, Inc.

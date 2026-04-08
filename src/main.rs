use clap::{CommandFactory, Parser};
use include_dir::{include_dir, Dir};
use jquants_cli::auth::{login, logout};
use jquants_cli::cli::{
    BulkCommands, Cli, Commands, DerivativesCommands, EquitiesCommands, FinsCommands,
    IndicesCommands, MarketsCommands, OutputFormat, SkillsCommands,
};
use jquants_cli::client::JQuantsClient;
use jquants_cli::config::Config;
use jquants_cli::download::handle_bulk_download;
use jquants_cli::error;
use jquants_cli::output::{output, output_fins_details, output_margin_alert, FieldSelection};
use jquants_cli::schema::{all_endpoint_keys, all_endpoint_schemas, lookup_endpoint};

static SKILL_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/.claude/skills/jquants-cli-usage");

// ── シェル補完セットアップ ────────────────────────────────────────────────────

fn setup_shell_completion() -> Result<(), error::AppError> {
    use std::io::Write;

    // $SHELL 環境変数からシェル名を取得
    let shell_path = std::env::var("SHELL")
        .map_err(|_| error::AppError::Config("$SHELL 環境変数が設定されていません".into()))?;
    let shell_name = std::path::Path::new(&shell_path)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| error::AppError::Config("$SHELL の解析に失敗しました".into()))?
        .to_string();

    let home = dirs::home_dir()
        .ok_or_else(|| error::AppError::Config("ホームディレクトリが見つかりません".into()))?;

    let (rc_path, init_line) = match shell_name.as_str() {
        "zsh" => (
            home.join(".zshrc"),
            "if command -v jquants >/dev/null 2>&1; then eval \"$(command jquants completion zsh)\"; fi".to_string(),
        ),
        "bash" => (
            home.join(".bashrc"),
            "if command -v jquants >/dev/null 2>&1; then eval \"$(command jquants completion bash)\"; fi".to_string(),
        ),
        "fish" => (
            // fish は macOS/Linux ともに XDG 規約の ~/.config/fish/config.fish を使用
            home.join(".config").join("fish").join("config.fish"),
            "if command -q jquants; jquants completion fish | source; end".to_string(),
        ),
        other => {
            return Err(error::AppError::Usage(format!(
                "未対応のシェルです: {other}\n  jquants completion <shell> で手動生成できます (bash, zsh, fish, elvish, powershell)"
            )));
        }
    };

    // 冪等性チェック: すでに設定が存在するか確認
    if rc_path.exists() {
        let content = std::fs::read_to_string(&rc_path)?;
        if content.contains("jquants completion") {
            println!(
                "✓ {} に jquants の補完設定は既に存在します",
                rc_path.display()
            );
            return Ok(());
        }
    }

    // rc ファイルに init line を追記（親ディレクトリが存在しない場合は作成）
    if let Some(parent) = rc_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&rc_path)?;
    writeln!(file)?;
    writeln!(file, "# J-Quants CLI shell completion")?;
    writeln!(file, "{init_line}")?;

    println!(
        "✓ {} に jquants の補完設定を追加しました",
        rc_path.display()
    );
    println!("  反映するにはシェルを再起動するか、以下を実行してください:");
    println!("  source {}", rc_path.display());

    Ok(())
}

// ── エントリポイント ──────────────────────────────────────────────────────────

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        if let Some(why) = e.why() {
            eprintln!("  Why: {}", why);
        }
        if let Some(hint) = e.hint() {
            eprintln!("  Hint: {}", hint);
        }
        std::process::exit(1);
    }
}

async fn run_equities(
    client: &JQuantsClient,
    out_fmt: &OutputFormat,
    save: &Option<String>,
    fields: &FieldSelection,
    command: EquitiesCommands,
) -> Result<(), error::AppError> {
    match command {
        EquitiesCommands::Master { code, date } => {
            let results = client
                .get_stock_master(code.as_deref(), date.as_deref())
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
        EquitiesCommands::Am { code } => {
            let results = client.get_am_bars(code.as_deref()).await?;
            output(&results, out_fmt, save, fields)?;
        }
        EquitiesCommands::Minute {
            code,
            date,
            from,
            to,
        } => {
            let results = client
                .get_minute_bars(
                    code.as_deref(),
                    date.as_deref(),
                    from.as_deref(),
                    to.as_deref(),
                )
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
        EquitiesCommands::Daily {
            code,
            date,
            from,
            to,
        } => {
            let results = client
                .get_daily_bars(
                    code.as_deref(),
                    date.as_deref(),
                    from.as_deref(),
                    to.as_deref(),
                )
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
        EquitiesCommands::EarningsCalendar {} => {
            let results = client.get_earnings_calendar().await?;
            output(&results, out_fmt, save, fields)?;
        }
        EquitiesCommands::InvestorTypes { section, from, to } => {
            let results = client
                .get_investor_types(section.as_deref(), from.as_deref(), to.as_deref())
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
        EquitiesCommands::Trades { date, download } => {
            let url = client
                .get_bulk(None, Some("/equities/trades"), date.as_deref())
                .await?;
            handle_bulk_download(client.http_client(), &url, download, out_fmt, save).await?;
        }
    }
    Ok(())
}

async fn run_markets(
    client: &JQuantsClient,
    out_fmt: &OutputFormat,
    save: &Option<String>,
    fields: &FieldSelection,
    command: MarketsCommands,
) -> Result<(), error::AppError> {
    match command {
        MarketsCommands::Breakdown {
            code,
            date,
            from,
            to,
        } => {
            let results = client
                .get_breakdown(
                    code.as_deref(),
                    date.as_deref(),
                    from.as_deref(),
                    to.as_deref(),
                )
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
        MarketsCommands::MarginAlert {
            code,
            date,
            from,
            to,
        } => {
            let results = client
                .get_margin_alert(
                    code.as_deref(),
                    date.as_deref(),
                    from.as_deref(),
                    to.as_deref(),
                )
                .await?;
            output_margin_alert(&results, out_fmt, save, fields)?;
        }
        MarketsCommands::MarginInterest {
            code,
            date,
            from,
            to,
        } => {
            let results = client
                .get_margin_interest(
                    code.as_deref(),
                    date.as_deref(),
                    from.as_deref(),
                    to.as_deref(),
                )
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
        MarketsCommands::Calendar { hol_div, from, to } => {
            let results = client
                .get_calendar(hol_div.as_deref(), from.as_deref(), to.as_deref())
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
        MarketsCommands::ShortRatio {
            s33,
            date,
            from,
            to,
        } => {
            let results = client
                .get_short_ratio(
                    s33.as_deref(),
                    date.as_deref(),
                    from.as_deref(),
                    to.as_deref(),
                )
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
        MarketsCommands::ShortSaleReport {
            code,
            disc_date,
            disc_date_from,
            disc_date_to,
            calc_date,
        } => {
            let results = client
                .get_short_sale_report(
                    code.as_deref(),
                    disc_date.as_deref(),
                    disc_date_from.as_deref(),
                    disc_date_to.as_deref(),
                    calc_date.as_deref(),
                )
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
    }
    Ok(())
}

async fn run_derivatives(
    client: &JQuantsClient,
    out_fmt: &OutputFormat,
    save: &Option<String>,
    fields: &FieldSelection,
    command: DerivativesCommands,
) -> Result<(), error::AppError> {
    match command {
        DerivativesCommands::Options225 { date } => {
            let results = client.get_options_225_bars(date.as_deref()).await?;
            output(&results, out_fmt, save, fields)?;
        }
        DerivativesCommands::Futures {
            category,
            date,
            contract_flag,
        } => {
            let results = client
                .get_futures_bars(
                    category.as_deref(),
                    date.as_deref(),
                    contract_flag.as_deref(),
                )
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
        DerivativesCommands::Options {
            category,
            code,
            date,
            contract_flag,
        } => {
            let results = client
                .get_options_bars(
                    category.as_deref(),
                    code.as_deref(),
                    date.as_deref(),
                    contract_flag.as_deref(),
                )
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
    }
    Ok(())
}

async fn run_fins(
    client: &JQuantsClient,
    out_fmt: &OutputFormat,
    save: &Option<String>,
    fields: &FieldSelection,
    command: FinsCommands,
) -> Result<(), error::AppError> {
    match command {
        FinsCommands::Details { code, date } => {
            let results = client
                .get_fins_details(code.as_deref(), date.as_deref())
                .await?;
            output_fins_details(&results, out_fmt, save, fields)?;
        }
        FinsCommands::Dividend {
            code,
            date,
            from,
            to,
        } => {
            let results = client
                .get_fins_dividend(
                    code.as_deref(),
                    date.as_deref(),
                    from.as_deref(),
                    to.as_deref(),
                )
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
        FinsCommands::Summary { code, date } => {
            let results = client
                .get_fins_summary(code.as_deref(), date.as_deref())
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
    }
    Ok(())
}

async fn run_indices(
    client: &JQuantsClient,
    out_fmt: &OutputFormat,
    save: &Option<String>,
    fields: &FieldSelection,
    command: IndicesCommands,
) -> Result<(), error::AppError> {
    match command {
        IndicesCommands::DailyTopix { from, to } => {
            let results = client
                .get_topix_daily_bars(from.as_deref(), to.as_deref())
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
        IndicesCommands::Daily {
            code,
            date,
            from,
            to,
        } => {
            let results = client
                .get_index_daily_bars(
                    code.as_deref(),
                    date.as_deref(),
                    from.as_deref(),
                    to.as_deref(),
                )
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
    }
    Ok(())
}

fn run_schema(
    out_fmt: &OutputFormat,
    save: &Option<String>,
    fields: &FieldSelection,
    endpoint: Option<&str>,
) -> Result<(), error::AppError> {
    match endpoint {
        None => {
            let schemas = all_endpoint_schemas();
            output(&schemas, out_fmt, save, fields)?;
        }
        Some(key) => match lookup_endpoint(key) {
            Some(field_schemas) => {
                output(&field_schemas, out_fmt, save, fields)?;
            }
            None => {
                let keys = all_endpoint_keys();
                return Err(error::AppError::Usage(format!(
                    "不明なエンドポイント: {}\n  利用可能: {}",
                    key,
                    keys.join(", ")
                )));
            }
        },
    }
    Ok(())
}

async fn run_bulk(
    client: &JQuantsClient,
    out_fmt: &OutputFormat,
    save: &Option<String>,
    fields: &FieldSelection,
    command: BulkCommands,
) -> Result<(), error::AppError> {
    match command {
        BulkCommands::Get {
            key,
            endpoint,
            date,
            download,
        } => {
            let url = client
                .get_bulk(key.as_deref(), endpoint.as_deref(), date.as_deref())
                .await?;
            handle_bulk_download(client.http_client(), &url, download, out_fmt, save).await?;
        }
        BulkCommands::List {
            endpoint,
            date,
            from,
            to,
        } => {
            let results = client
                .get_bulk_list(
                    endpoint.as_deref(),
                    date.as_deref(),
                    from.as_deref(),
                    to.as_deref(),
                )
                .await?;
            output(&results, out_fmt, save, fields)?;
        }
    }
    Ok(())
}

async fn run() -> Result<(), error::AppError> {
    let cli = Cli::parse();

    // .env ファイルを早期ロード（login コマンドも環境変数を参照するため）
    let _ = dotenvy::dotenv();

    // API アクセス不要なコマンドを早期処理
    if let Commands::Skills { command } = &cli.command {
        match command {
            SkillsCommands::Add { dir } => {
                let base = std::path::Path::new(dir).join("jquants-cli-usage");
                fn extract_dir(
                    dir: &Dir<'_>,
                    base: &std::path::Path,
                ) -> Result<(), error::AppError> {
                    std::fs::create_dir_all(base)?;
                    for file in dir.files() {
                        let dest = base.join(file.path());
                        if let Some(parent) = dest.parent() {
                            std::fs::create_dir_all(parent)?;
                        }
                        std::fs::write(&dest, file.contents())?;
                        println!("Installed: {}", dest.display());
                    }
                    for sub in dir.dirs() {
                        extract_dir(sub, base)?;
                    }
                    Ok(())
                }
                extract_dir(&SKILL_DIR, &base)?;
                return Ok(());
            }
        }
    }

    if let Commands::Completion { shell } = cli.command {
        match shell {
            Some(s) => {
                clap_complete::generate(s, &mut Cli::command(), "jquants", &mut std::io::stdout());
            }
            None => {
                setup_shell_completion()?;
            }
        }
        return Ok(());
    }

    if matches!(cli.command, Commands::Login) {
        return login().await;
    }

    if matches!(cli.command, Commands::Logout) {
        return logout().await;
    }

    if cli.save.is_some() && matches!(cli.output, OutputFormat::Table) {
        return Err(error::AppError::Usage(
            "--save は --output table と併用できません".to_string(),
        ));
    }
    if matches!(cli.output, OutputFormat::Parquet) && cli.save.is_none() {
        return Err(error::AppError::Usage(
            "--output parquet には --save <path> が必要です".to_string(),
        ));
    }

    // --fields の空ベクタは未指定扱い（-f "" や -f "," のケース）
    let fields: FieldSelection = cli
        .fields
        .and_then(|v| if v.is_empty() { None } else { Some(v) });

    if let Commands::Schema { ref endpoint } = cli.command {
        return run_schema(&cli.output, &cli.save, &fields, endpoint.as_deref());
    }

    let config = Config::from_env().await?;
    let client = JQuantsClient::new(config);

    match cli.command {
        Commands::Equities { command } => {
            run_equities(&client, &cli.output, &cli.save, &fields, command).await?;
        }
        Commands::Markets { command } => {
            run_markets(&client, &cli.output, &cli.save, &fields, command).await?;
        }
        Commands::Derivatives { command } => {
            run_derivatives(&client, &cli.output, &cli.save, &fields, command).await?;
        }
        Commands::Fins { command } => {
            run_fins(&client, &cli.output, &cli.save, &fields, command).await?;
        }
        Commands::Indices { command } => {
            run_indices(&client, &cli.output, &cli.save, &fields, command).await?;
        }
        Commands::Bulk { command } => {
            run_bulk(&client, &cli.output, &cli.save, &fields, command).await?;
        }
        Commands::Schema { .. } => unreachable!("Schema command is handled above"),
        Commands::Skills { .. } => unreachable!("Skills commands are handled above"),
        Commands::Completion { .. } => unreachable!("Completion command is handled above"),
        Commands::Login => unreachable!("Login command is handled above"),
        Commands::Logout => unreachable!("Logout command is handled above"),
    }

    Ok(())
}

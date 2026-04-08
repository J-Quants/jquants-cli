use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Csv,
    Parquet,
}

#[derive(Parser)]
#[command(name = "jquants")]
#[command(about = "A CLI tool for querying the J-Quants API V2 (Japanese stock market data)")]
#[command(version)]
#[command(after_long_help = "\
Examples:
  jquants eq master                              # 全銘柄マスタ取得
  jquants eq master --code 86970                 # 特定銘柄のマスタ
  jquants eq daily --code 86970                  # 株価四本値取得
  jquants --output json eq daily --code 86970    # JSON出力（全フィールド）
  jquants --output csv --save out.csv eq master             # CSV保存
  jquants --output parquet --save out.parquet eq daily --code 86970  # Parquet保存
  jquants mkt calendar                                       # 取引カレンダー取得
  jquants fins details --code 86970                          # 財務諸表取得

Environment:
  JQUANTS_API_KEY   (required) J-Quants API key
  JQUANTS_BASE_URL  (optional) API base URL [default: https://api.jquants.com/v2]

Tip: Use --output json for full field details, --output csv for data analysis.
     Use --output parquet for columnar data analysis (requires --save).
     Pipe output to other tools: jquants eq master | head
     Use -f to select specific fields (API field names, same as JSON/CSV keys):
       jquants -f Date,Code,AdjC eq daily --code 86970")]
pub struct Cli {
    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Table)]
    pub output: OutputFormat,

    /// Save output to file
    #[arg(
        long,
        value_name = "PATH",
        long_help = "Save output to a file (CSV/JSON/Parquet; Parquet requires this flag)"
    )]
    pub save: Option<String>,

    /// Select output fields (comma-separated)
    #[arg(
        short = 'f',
        long,
        value_name = "FIELDS",
        value_delimiter = ',',
        long_help = "Select specific output fields (comma-separated API field names, e.g. Date,Code,AdjC).\nField names match JSON/CSV/Parquet keys (not the abbreviated table headers)."
    )]
    pub fields: Option<Vec<String>>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Equities-related API endpoints (株式)
    #[command(name = "eq")]
    #[command(after_long_help = "\
Examples:
  jquants eq master                           # 全銘柄マスタ取得
  jquants eq master --code 86970              # 特定銘柄のマスタ
  jquants eq daily --code 86970               # 株価四本値取得
  jquants eq daily --date 2026-03-14          # 特定日の全銘柄四本値
  jquants eq am                               # 前場四本値取得
  jquants eq minute --code 27800              # 分足データ取得
  jquants eq earnings-calendar                # 決算発表予定日取得
  jquants eq investor-types --section TSEPrime  # 投資部門別売買状況
  jquants eq trades --date 2025-12 --download   # 株価ティックダウンロード")]
    Equities {
        #[command(subcommand)]
        command: EquitiesCommands,
    },
    /// Markets-related API endpoints (市場)
    #[command(name = "mkt")]
    #[command(after_long_help = "\
Examples:
  jquants mkt breakdown --code 27800          # 売買内訳取得
  jquants mkt margin-alert --code 13260       # 日々公表信用取引残高
  jquants mkt margin-interest --code 27800    # 信用取引週末残高
  jquants mkt calendar                        # 取引カレンダー
  jquants mkt short-ratio --s33 0050          # 業種別空売り比率 (--s33 は業種コード、--code ではない)
  jquants mkt short-sale-report --code 13660  # 空売り残高報告")]
    Markets {
        #[command(subcommand)]
        command: MarketsCommands,
    },
    /// Bulk download API endpoints
    #[command(after_long_help = "\
Examples:
  jquants bulk list                                     # ダウンロード可能ファイル一覧
  jquants bulk list --endpoint /equities/bars/daily     # エンドポイントでフィルタ
  jquants bulk get --key \"path/to/file.gz\"            # キー指定でURL取得
  jquants bulk get --endpoint /equities/bars/daily --date 2026-03  # エンドポイント+日付
  jquants bulk get --key \"path/to/file.gz\" --download  # ファイルダウンロード

Note: --key と --endpoint+--date は排他。--key は bulk list で確認したキーを使用。")]
    Bulk {
        #[command(subcommand)]
        command: BulkCommands,
    },
    /// Derivatives-related API endpoints (デリバティブ)
    #[command(name = "deriv")]
    #[command(after_long_help = "\
Examples:
  jquants deriv options-225 --date 2021-09-01            # 日経225オプション
  jquants deriv futures --date 2021-09-01                # 先物四本値
  jquants deriv futures --date 2021-09-01 --category TOPIXF  # 商品区分でフィルタ
  jquants deriv options --date 2021-09-01                # オプション四本値
  jquants deriv options --date 2021-09-01 --category TOPIXE  # 商品区分でフィルタ")]
    Derivatives {
        #[command(subcommand)]
        command: DerivativesCommands,
    },
    /// Fins-related API endpoints (財務)
    #[command(name = "fins")]
    #[command(after_long_help = "\
Examples:
  jquants fins details --code 86970             # 財務諸表取得
  jquants --output json fins details --code 86970  # JSON出力で FS フィールドを完全表示
  jquants fins dividend --code 27800            # 配当金情報取得
  jquants fins summary --code 86970             # 財務情報サマリー取得

Note: fins details の FS フィールドはテーブル表示では \"N items\" と略される。
      完全な財務諸表データは --output json で取得。")]
    Fins {
        #[command(subcommand)]
        command: FinsCommands,
    },
    /// Indices-related API endpoints (指数)
    #[command(name = "idx")]
    #[command(after_long_help = "\
Examples:
  jquants idx daily-topix                                     # TOPIX指数四本値
  jquants idx daily-topix --from 2021-09-01 --to 2021-09-07  # 期間でフィルタ
  jquants idx daily --code 0028                               # 指数コード指定
  jquants idx daily --date 2021-09-07                         # 日付でフィルタ")]
    Indices {
        #[command(subcommand)]
        command: IndicesCommands,
    },
    /// Show API response schema
    #[command(
        long_about = "Show API response schema for endpoints (エンドポイントのスキーマ情報)"
    )]
    #[command(after_long_help = "\
Examples:
  jquants schema                         # 全エンドポイント一覧
  jquants schema eq.daily                # 株価四本値のフィールド詳細
  jquants --output json schema eq.daily  # JSON出力")]
    Schema {
        /// エンドポイントキー（例: eq.daily, mkt.breakdown）。省略で一覧表示
        endpoint: Option<String>,
    },
    /// AI agent skill file management
    Skills {
        #[command(subcommand)]
        command: SkillsCommands,
    },
    /// シェル補完の設定
    #[command(name = "completion")]
    Completion {
        /// 補完スクリプトを生成するシェル（省略時: 自動検出してrcファイルに設定を追加）
        shell: Option<clap_complete::Shell>,
    },
    /// Log in and save API key
    #[command(
        long_about = "Log in via Cognito OAuth2 PKCE and save API key to ~/.config/jquants/credentials.json"
    )]
    #[command(after_long_help = "\
Environment (optional overrides):
  COGNITO_DOMAIN      Cognito Hosted UI domain [built-in default]
  COGNITO_CLIENT_ID   Cognito app client ID [built-in default]
  COGNITO_SCOPES      OAuth2 scopes [default: openid]

After login, the saved API key is used automatically as x-api-key header.")]
    Login,
    /// Log out and remove credentials
    #[command(
        long_about = "Log out via Cognito session clear and remove saved credentials from ~/.config/jquants/credentials.json"
    )]
    #[command(after_long_help = "\
Environment (optional overrides):
  COGNITO_DOMAIN      Cognito Hosted UI domain [built-in default]
  COGNITO_CLIENT_ID   Cognito app client ID [built-in default]

Note: A browser window will open to clear the Cognito session.
      Local credentials (~/.config/jquants/credentials.json) are removed even if the browser step fails.")]
    Logout,
}

#[derive(Subcommand)]
pub enum EquitiesCommands {
    /// Fetch stock master data
    Master {
        /// Stock code
        #[arg(long)]
        code: Option<String>,

        /// Date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
    },
    /// Fetch morning session (AM) bar data
    Am {
        /// Stock code
        #[arg(long)]
        code: Option<String>,
    },
    /// Fetch minute bar data (分足)
    Minute {
        /// Stock code
        #[arg(long)]
        code: Option<String>,

        /// Date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,

        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,

        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Fetch daily bar data (株価四本値)
    Daily {
        /// Stock code
        #[arg(long)]
        code: Option<String>,

        /// Date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,

        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,

        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Fetch earnings calendar (決算発表予定日)
    #[command(name = "earnings-calendar")]
    EarningsCalendar {},
    /// Fetch tick data (株価ティック)
    #[command(long_about = "Fetch tick data (株価ティック) via bulk download")]
    Trades {
        /// Date (YYYY-MM-DD, YYYY-MM)
        #[arg(long)]
        date: Option<String>,

        /// Download the file to current directory
        #[arg(long)]
        download: bool,
    },
    /// Fetch investor type data (投資部門別)
    #[command(long_about = "Fetch investor type trading data (投資部門別売買状況)")]
    #[command(name = "investor-types")]
    InvestorTypes {
        /// Section (e.g., TSEPrime)
        #[arg(long)]
        section: Option<String>,

        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,

        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum MarketsCommands {
    /// Fetch trading breakdown data (売買内訳)
    Breakdown {
        /// Stock code
        #[arg(long)]
        code: Option<String>,

        /// Date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,

        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,

        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Fetch margin alert data (信用取引残高)
    #[command(long_about = "Fetch daily margin alert data (日々公表信用取引残高)")]
    #[command(name = "margin-alert")]
    MarginAlert {
        /// Stock code
        #[arg(long)]
        code: Option<String>,
        /// Date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Fetch margin interest data (週末残高)
    #[command(long_about = "Fetch weekly margin interest data (信用取引週末残高)")]
    #[command(name = "margin-interest")]
    MarginInterest {
        /// Stock code
        #[arg(long)]
        code: Option<String>,
        /// Date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Fetch trading calendar (営業日・休業日)
    Calendar {
        /// Holiday division (休日区分)
        #[arg(long)]
        hol_div: Option<String>,

        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,

        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Fetch short-selling ratio (空売り比率)
    #[command(long_about = "Fetch sector short-selling ratio data (業種別空売り比率)")]
    #[command(name = "short-ratio")]
    #[command(after_long_help = "\
Note: フィルタには --code ではなく --s33 (33業種コード) を使用。
      例: 0050=水産・農林業, 3050=食料品, 9050=サービス業

Examples:
  jquants mkt short-ratio --s33 0050           # 業種コード指定
  jquants mkt short-ratio --date 2022-10-25    # 日付指定")]
    ShortRatio {
        /// 33-sector code (e.g., 0050)
        #[arg(long)]
        s33: Option<String>,

        /// Date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,

        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,

        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Fetch short sale report (空売り残高報告)
    #[command(long_about = "Fetch short sale report data (空売り残高報告)")]
    #[command(name = "short-sale-report")]
    #[command(after_long_help = "\
Note: --disc-date は公表日、--calc-date は計算日（異なる概念）。
      通常は --disc-date を使用。--calc-date は公表日の1〜数日前の日付。

Examples:
  jquants mkt short-sale-report --code 13660            # 銘柄コード指定
  jquants mkt short-sale-report --disc-date 2024-08-01  # 公表日指定
  jquants mkt short-sale-report --calc-date 2024-07-31  # 計算日指定")]
    ShortSaleReport {
        /// Stock code (e.g., 13660)
        #[arg(long)]
        code: Option<String>,
        /// Disclosure date (YYYY-MM-DD)
        #[arg(long)]
        disc_date: Option<String>,
        /// Disclosure date from (YYYY-MM-DD)
        #[arg(long)]
        disc_date_from: Option<String>,
        /// Disclosure date to (YYYY-MM-DD)
        #[arg(long)]
        disc_date_to: Option<String>,
        /// Calculation date (YYYY-MM-DD)
        #[arg(long)]
        calc_date: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum DerivativesCommands {
    /// Fetch Nikkei 225 options (日経225OP)
    #[command(long_about = "Fetch Nikkei 225 options daily bar data (日経225オプション四本値)")]
    #[command(name = "options-225")]
    Options225 {
        /// Date (YYYY-MM-DD, required)
        #[arg(long)]
        date: Option<String>,
    },
    /// Fetch futures daily bar data (先物四本値)
    Futures {
        /// Product category (商品区分)
        #[arg(long)]
        category: Option<String>,

        /// Date (YYYY-MM-DD, required)
        #[arg(long)]
        date: Option<String>,

        /// Contract flag (中心限月フラグ)
        #[arg(long)]
        contract_flag: Option<String>,
    },
    /// Fetch options daily bar data (オプション四本値)
    Options {
        /// Product category (商品区分)
        #[arg(long)]
        category: Option<String>,

        /// Underlying code (対象銘柄コード)
        #[arg(long)]
        code: Option<String>,

        /// Date (YYYY-MM-DD, required)
        #[arg(long)]
        date: Option<String>,

        /// Contract flag (中心限月フラグ)
        #[arg(long)]
        contract_flag: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum FinsCommands {
    /// Fetch financial statements (財務諸表)
    #[command(long_about = "Fetch financial statements (BS/PL/CF) (財務諸表)")]
    #[command(after_long_help = "\
Note: FS フィールドはテーブル表示では \"N items\" と略される。
      完全な財務諸表データ（BS/PL/CF）を取得するには --output json を使用。

Examples:
  jquants fins details --code 86970                # テーブル表示
  jquants --output json fins details --code 86970  # 完全な財務データをJSON出力
  jquants fins details --date 2022-01-05           # 日付指定")]
    Details {
        /// Stock code
        #[arg(long)]
        code: Option<String>,

        /// Disclosure date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
    },
    /// Fetch dividend data (配当金情報)
    Dividend {
        /// Stock code
        #[arg(long)]
        code: Option<String>,

        /// Date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,

        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,

        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Fetch financial summary (財務情報サマリー)
    Summary {
        /// Stock code
        #[arg(long)]
        code: Option<String>,
        /// Disclosure date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum IndicesCommands {
    /// Fetch TOPIX daily bar data (TOPIX指数四本値)
    #[command(name = "daily-topix")]
    DailyTopix {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Fetch indices daily bar data (指数四本値)
    Daily {
        /// Index code
        #[arg(long)]
        code: Option<String>,

        /// Date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,

        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,

        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum SkillsCommands {
    /// Install AI agent skill file
    #[command(long_about = "Install AI agent skill file (SKILL.md) to the specified directory")]
    Add {
        /// Target directory
        #[arg(long, default_value = ".")]
        dir: String,
    },
}

#[derive(Subcommand)]
pub enum BulkCommands {
    /// Fetch bulk download URL
    #[command(long_about = "Fetch bulk download URL (ファイルダウンロード用URL取得)")]
    Get {
        /// File key from /bulk/list
        #[arg(long)]
        key: Option<String>,

        /// Endpoint name
        #[arg(long)]
        endpoint: Option<String>,

        /// Date (YYYY-MM-DD, YYYYMMDD, YYYY-MM, YYYYMM)
        #[arg(long)]
        date: Option<String>,

        /// Download the file to current directory
        #[arg(long)]
        download: bool,
    },
    /// List bulk downloadable files
    #[command(long_about = "List bulk downloadable files (ダウンロード可能ファイル一覧取得)")]
    List {
        /// Endpoint name
        #[arg(long)]
        endpoint: Option<String>,

        /// Date (YYYY-MM-DD, YYYYMMDD, YYYY-MM, YYYYMM)
        #[arg(long)]
        date: Option<String>,

        /// Start date
        #[arg(long)]
        from: Option<String>,

        /// End date
        #[arg(long)]
        to: Option<String>,
    },
}

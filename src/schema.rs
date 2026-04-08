use crate::models::{
    AmBar, Breakdown, BulkListItem, Calendar, DailyBar, EarningsCalendar, FinsDetails,
    FinsDividend, FinsSummary, FuturesBar, IndexDailyBar, InvestorType, MarginAlert,
    MarginInterest, MinuteBar, Options225Bar, OptionsBar, ShortRatio, ShortSaleReport, StockMaster,
    TopixDailyBar,
};
use serde::Serialize;

// ── スキーマ定義の構造体 ──────────────────────────────────────────────────────

/// エンドポイントの1フィールドのスキーマ情報
#[derive(Debug, Clone, Serialize)]
pub struct FieldSchema {
    /// APIフィールド名（JSON/CSV/Parquetのキー名）
    #[serde(rename = "Field")]
    pub name: &'static str,
    /// 型（string / number / number? / string? / integer / object）
    #[serde(rename = "Type")]
    pub field_type: &'static str,
    /// フィールドの説明（日本語）
    #[serde(rename = "Description")]
    pub description: &'static str,
}

/// エンドポイント一覧用のサマリー情報
#[derive(Debug, Clone, Serialize)]
pub struct EndpointSchema {
    /// ドット区切りのエンドポイントキー（例: eq.daily）
    #[serde(rename = "Endpoint")]
    pub key: &'static str,
    /// エンドポイントの説明（日本語）
    #[serde(rename = "Description")]
    pub description: &'static str,
    /// フィールド数
    #[serde(rename = "Fields")]
    pub field_count: usize,
}

// ── SchemaInfo トレイト ───────────────────────────────────────────────────────

/// 各APIレスポンスモデルにスキーマ情報を提供するトレイト
pub trait SchemaInfo {
    /// ドット区切りのエンドポイントキー（例: "eq.daily"）
    fn endpoint_key() -> &'static str;
    /// エンドポイントの説明（日本語）
    fn endpoint_description() -> &'static str;
    /// 全フィールドのスキーマ定義
    fn field_schemas() -> Vec<FieldSchema>;
    /// フィールド数（make_summary でのVecアロケーションを避けるためオーバーライド可）
    fn field_count() -> usize {
        Self::field_schemas().len()
    }
}

// ── SchemaInfo impls ──────────────────────────────────────────────────────────

impl SchemaInfo for StockMaster {
    fn endpoint_key() -> &'static str {
        "eq.master"
    }
    fn endpoint_description() -> &'static str {
        "銘柄マスタ（銘柄名・市場・業種・規模区分等）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "基準日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            FieldSchema {
                name: "CoName",
                field_type: "string",
                description: "銘柄名（日本語）",
            },
            FieldSchema {
                name: "CoNameEn",
                field_type: "string",
                description: "銘柄名（英語）",
            },
            FieldSchema {
                name: "S17",
                field_type: "string",
                description: "17業種コード",
            },
            FieldSchema {
                name: "S17Nm",
                field_type: "string",
                description: "17業種名",
            },
            FieldSchema {
                name: "S33",
                field_type: "string",
                description: "33業種コード",
            },
            FieldSchema {
                name: "S33Nm",
                field_type: "string",
                description: "33業種名",
            },
            FieldSchema {
                name: "ScaleCat",
                field_type: "string",
                description: "規模区分（TOPIX Large70等）",
            },
            FieldSchema {
                name: "Mkt",
                field_type: "string",
                description: "市場区分コード",
            },
            FieldSchema {
                name: "MktNm",
                field_type: "string",
                description: "市場区分名（プライム等）",
            },
            FieldSchema {
                name: "Mrgn",
                field_type: "string",
                description: "信用区分コード",
            },
            FieldSchema {
                name: "MrgnNm",
                field_type: "string",
                description: "信用区分名（信用/貸借）",
            },
        ]
    }
    fn field_count() -> usize {
        13
    }
}

impl SchemaInfo for AmBar {
    fn endpoint_key() -> &'static str {
        "eq.am"
    }
    fn endpoint_description() -> &'static str {
        "前場四本値（午前立会の始値・高値・安値・終値・出来高・売買代金）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "日付 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            FieldSchema {
                name: "MO",
                field_type: "number?",
                description: "前場始値",
            },
            FieldSchema {
                name: "MH",
                field_type: "number?",
                description: "前場高値",
            },
            FieldSchema {
                name: "ML",
                field_type: "number?",
                description: "前場安値",
            },
            FieldSchema {
                name: "MC",
                field_type: "number?",
                description: "前場終値",
            },
            FieldSchema {
                name: "MVo",
                field_type: "number?",
                description: "前場出来高（株）",
            },
            FieldSchema {
                name: "MVa",
                field_type: "number?",
                description: "前場売買代金（円）",
            },
        ]
    }
    fn field_count() -> usize {
        8
    }
}

impl SchemaInfo for MinuteBar {
    fn endpoint_key() -> &'static str {
        "eq.minute"
    }
    fn endpoint_description() -> &'static str {
        "株価分足データ（1分足の始値・高値・安値・終値・出来高・売買代金）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "日付 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Time",
                field_type: "string",
                description: "時刻 (HH:MM:SS)",
            },
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            FieldSchema {
                name: "O",
                field_type: "number",
                description: "始値",
            },
            FieldSchema {
                name: "H",
                field_type: "number",
                description: "高値",
            },
            FieldSchema {
                name: "L",
                field_type: "number",
                description: "安値",
            },
            FieldSchema {
                name: "C",
                field_type: "number",
                description: "終値",
            },
            FieldSchema {
                name: "Vo",
                field_type: "number",
                description: "出来高（株）",
            },
            FieldSchema {
                name: "Va",
                field_type: "number",
                description: "売買代金（円）",
            },
        ]
    }
    fn field_count() -> usize {
        9
    }
}

impl SchemaInfo for DailyBar {
    fn endpoint_key() -> &'static str {
        "eq.daily"
    }
    fn endpoint_description() -> &'static str {
        "株価四本値日足（始値・高値・安値・終値・出来高・売買代金・調整後価格・前場・後場内訳含む）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "日付 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            // ── 全市場合算 ──
            FieldSchema {
                name: "O",
                field_type: "number?",
                description: "始値",
            },
            FieldSchema {
                name: "H",
                field_type: "number?",
                description: "高値",
            },
            FieldSchema {
                name: "L",
                field_type: "number?",
                description: "安値",
            },
            FieldSchema {
                name: "C",
                field_type: "number?",
                description: "終値",
            },
            FieldSchema {
                name: "UL",
                field_type: "string?",
                description: "ストップ高フラグ",
            },
            FieldSchema {
                name: "LL",
                field_type: "string?",
                description: "ストップ安フラグ",
            },
            FieldSchema {
                name: "Vo",
                field_type: "number?",
                description: "出来高（株）",
            },
            FieldSchema {
                name: "Va",
                field_type: "number?",
                description: "売買代金（円）",
            },
            FieldSchema {
                name: "AdjFactor",
                field_type: "number?",
                description: "調整係数（分割・併合等）",
            },
            FieldSchema {
                name: "AdjO",
                field_type: "number?",
                description: "調整後始値",
            },
            FieldSchema {
                name: "AdjH",
                field_type: "number?",
                description: "調整後高値",
            },
            FieldSchema {
                name: "AdjL",
                field_type: "number?",
                description: "調整後安値",
            },
            FieldSchema {
                name: "AdjC",
                field_type: "number?",
                description: "調整後終値",
            },
            FieldSchema {
                name: "AdjVo",
                field_type: "number?",
                description: "調整後出来高",
            },
            // ── 前場 ──
            FieldSchema {
                name: "MO",
                field_type: "number?",
                description: "前場始値",
            },
            FieldSchema {
                name: "MH",
                field_type: "number?",
                description: "前場高値",
            },
            FieldSchema {
                name: "ML",
                field_type: "number?",
                description: "前場安値",
            },
            FieldSchema {
                name: "MC",
                field_type: "number?",
                description: "前場終値",
            },
            FieldSchema {
                name: "MUL",
                field_type: "string?",
                description: "前場ストップ高フラグ",
            },
            FieldSchema {
                name: "MLL",
                field_type: "string?",
                description: "前場ストップ安フラグ",
            },
            FieldSchema {
                name: "MVo",
                field_type: "number?",
                description: "前場出来高",
            },
            FieldSchema {
                name: "MVa",
                field_type: "number?",
                description: "前場売買代金",
            },
            FieldSchema {
                name: "MAdjO",
                field_type: "number?",
                description: "前場調整後始値",
            },
            FieldSchema {
                name: "MAdjH",
                field_type: "number?",
                description: "前場調整後高値",
            },
            FieldSchema {
                name: "MAdjL",
                field_type: "number?",
                description: "前場調整後安値",
            },
            FieldSchema {
                name: "MAdjC",
                field_type: "number?",
                description: "前場調整後終値",
            },
            FieldSchema {
                name: "MAdjVo",
                field_type: "number?",
                description: "前場調整後出来高",
            },
            // ── 後場 ──
            FieldSchema {
                name: "AO",
                field_type: "number?",
                description: "後場始値",
            },
            FieldSchema {
                name: "AH",
                field_type: "number?",
                description: "後場高値",
            },
            FieldSchema {
                name: "AL",
                field_type: "number?",
                description: "後場安値",
            },
            FieldSchema {
                name: "AC",
                field_type: "number?",
                description: "後場終値",
            },
            FieldSchema {
                name: "AUL",
                field_type: "string?",
                description: "後場ストップ高フラグ",
            },
            FieldSchema {
                name: "ALL",
                field_type: "string?",
                description: "後場ストップ安フラグ",
            },
            FieldSchema {
                name: "AVo",
                field_type: "number?",
                description: "後場出来高",
            },
            FieldSchema {
                name: "AVa",
                field_type: "number?",
                description: "後場売買代金",
            },
            FieldSchema {
                name: "AAdjO",
                field_type: "number?",
                description: "後場調整後始値",
            },
            FieldSchema {
                name: "AAdjH",
                field_type: "number?",
                description: "後場調整後高値",
            },
            FieldSchema {
                name: "AAdjL",
                field_type: "number?",
                description: "後場調整後安値",
            },
            FieldSchema {
                name: "AAdjC",
                field_type: "number?",
                description: "後場調整後終値",
            },
            FieldSchema {
                name: "AAdjVo",
                field_type: "number?",
                description: "後場調整後出来高",
            },
        ]
    }
    fn field_count() -> usize {
        42
    }
}

impl SchemaInfo for EarningsCalendar {
    fn endpoint_key() -> &'static str {
        "eq.earnings-calendar"
    }
    fn endpoint_description() -> &'static str {
        "決算発表予定日カレンダー（銘柄・決算期・セクター等）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "決算発表予定日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            FieldSchema {
                name: "CoName",
                field_type: "string",
                description: "銘柄名",
            },
            FieldSchema {
                name: "FY",
                field_type: "string",
                description: "決算期（会計年度）",
            },
            FieldSchema {
                name: "SectorNm",
                field_type: "string",
                description: "セクター名",
            },
            FieldSchema {
                name: "FQ",
                field_type: "string",
                description: "決算四半期区分（1Q/2Q/3Q/通期）",
            },
            FieldSchema {
                name: "Section",
                field_type: "string",
                description: "市場区分",
            },
        ]
    }
    fn field_count() -> usize {
        7
    }
}

impl SchemaInfo for InvestorType {
    fn endpoint_key() -> &'static str {
        "eq.investor-types"
    }
    fn endpoint_description() -> &'static str {
        "投資部門別売買状況（投資家種別ごとの売買金額・残高）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "PubDate",
                field_type: "string",
                description: "公表日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "StDate",
                field_type: "string",
                description: "集計開始日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "EnDate",
                field_type: "string",
                description: "集計終了日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Section",
                field_type: "string",
                description: "市場区分（TSEPrime等）",
            },
            // ── 自己 ──
            FieldSchema {
                name: "PropSell",
                field_type: "string",
                description: "自己売り",
            },
            FieldSchema {
                name: "PropBuy",
                field_type: "string",
                description: "自己買い",
            },
            FieldSchema {
                name: "PropTot",
                field_type: "string",
                description: "自己合計",
            },
            FieldSchema {
                name: "PropBal",
                field_type: "string",
                description: "自己差引",
            },
            // ── 委託 ──
            FieldSchema {
                name: "BrkSell",
                field_type: "string",
                description: "委託売り",
            },
            FieldSchema {
                name: "BrkBuy",
                field_type: "string",
                description: "委託買い",
            },
            FieldSchema {
                name: "BrkTot",
                field_type: "string",
                description: "委託合計",
            },
            FieldSchema {
                name: "BrkBal",
                field_type: "string",
                description: "委託差引",
            },
            // ── 総計 ──
            FieldSchema {
                name: "TotSell",
                field_type: "string",
                description: "総売り",
            },
            FieldSchema {
                name: "TotBuy",
                field_type: "string",
                description: "総買い",
            },
            FieldSchema {
                name: "TotTot",
                field_type: "string",
                description: "総合計",
            },
            FieldSchema {
                name: "TotBal",
                field_type: "string",
                description: "総差引",
            },
            // ── 個人 ──
            FieldSchema {
                name: "IndSell",
                field_type: "string",
                description: "個人売り",
            },
            FieldSchema {
                name: "IndBuy",
                field_type: "string",
                description: "個人買い",
            },
            FieldSchema {
                name: "IndTot",
                field_type: "string",
                description: "個人合計",
            },
            FieldSchema {
                name: "IndBal",
                field_type: "string",
                description: "個人差引",
            },
            // ── 外国人 ──
            FieldSchema {
                name: "FrgnSell",
                field_type: "string",
                description: "外国人売り",
            },
            FieldSchema {
                name: "FrgnBuy",
                field_type: "string",
                description: "外国人買い",
            },
            FieldSchema {
                name: "FrgnTot",
                field_type: "string",
                description: "外国人合計",
            },
            FieldSchema {
                name: "FrgnBal",
                field_type: "string",
                description: "外国人差引",
            },
            // ── 証券会社 ──
            FieldSchema {
                name: "SecCoSell",
                field_type: "string",
                description: "証券会社売り",
            },
            FieldSchema {
                name: "SecCoBuy",
                field_type: "string",
                description: "証券会社買い",
            },
            FieldSchema {
                name: "SecCoTot",
                field_type: "string",
                description: "証券会社合計",
            },
            FieldSchema {
                name: "SecCoBal",
                field_type: "string",
                description: "証券会社差引",
            },
            // ── 投資信託 ──
            FieldSchema {
                name: "InvTrSell",
                field_type: "string",
                description: "投資信託売り",
            },
            FieldSchema {
                name: "InvTrBuy",
                field_type: "string",
                description: "投資信託買い",
            },
            FieldSchema {
                name: "InvTrTot",
                field_type: "string",
                description: "投資信託合計",
            },
            FieldSchema {
                name: "InvTrBal",
                field_type: "string",
                description: "投資信託差引",
            },
            // ── 事業法人 ──
            FieldSchema {
                name: "BusCoSell",
                field_type: "string",
                description: "事業法人売り",
            },
            FieldSchema {
                name: "BusCoBuy",
                field_type: "string",
                description: "事業法人買い",
            },
            FieldSchema {
                name: "BusCoTot",
                field_type: "string",
                description: "事業法人合計",
            },
            FieldSchema {
                name: "BusCoBal",
                field_type: "string",
                description: "事業法人差引",
            },
            // ── その他法人 ──
            FieldSchema {
                name: "OthCoSell",
                field_type: "string",
                description: "その他法人売り",
            },
            FieldSchema {
                name: "OthCoBuy",
                field_type: "string",
                description: "その他法人買い",
            },
            FieldSchema {
                name: "OthCoTot",
                field_type: "string",
                description: "その他法人合計",
            },
            FieldSchema {
                name: "OthCoBal",
                field_type: "string",
                description: "その他法人差引",
            },
            // ── 生損保 ──
            FieldSchema {
                name: "InsCoSell",
                field_type: "string",
                description: "生損保売り",
            },
            FieldSchema {
                name: "InsCoBuy",
                field_type: "string",
                description: "生損保買い",
            },
            FieldSchema {
                name: "InsCoTot",
                field_type: "string",
                description: "生損保合計",
            },
            FieldSchema {
                name: "InsCoBal",
                field_type: "string",
                description: "生損保差引",
            },
            // ── 都銀・地銀等 ──
            FieldSchema {
                name: "BankSell",
                field_type: "string",
                description: "都銀・地銀等売り",
            },
            FieldSchema {
                name: "BankBuy",
                field_type: "string",
                description: "都銀・地銀等買い",
            },
            FieldSchema {
                name: "BankTot",
                field_type: "string",
                description: "都銀・地銀等合計",
            },
            FieldSchema {
                name: "BankBal",
                field_type: "string",
                description: "都銀・地銀等差引",
            },
            // ── 信託銀行 ──
            FieldSchema {
                name: "TrstBnkSell",
                field_type: "string",
                description: "信託銀行売り",
            },
            FieldSchema {
                name: "TrstBnkBuy",
                field_type: "string",
                description: "信託銀行買い",
            },
            FieldSchema {
                name: "TrstBnkTot",
                field_type: "string",
                description: "信託銀行合計",
            },
            FieldSchema {
                name: "TrstBnkBal",
                field_type: "string",
                description: "信託銀行差引",
            },
            // ── その他金融機関 ──
            FieldSchema {
                name: "OthFinSell",
                field_type: "string",
                description: "その他金融機関売り",
            },
            FieldSchema {
                name: "OthFinBuy",
                field_type: "string",
                description: "その他金融機関買い",
            },
            FieldSchema {
                name: "OthFinTot",
                field_type: "string",
                description: "その他金融機関合計",
            },
            FieldSchema {
                name: "OthFinBal",
                field_type: "string",
                description: "その他金融機関差引",
            },
        ]
    }
    fn field_count() -> usize {
        56
    }
}

impl SchemaInfo for Breakdown {
    fn endpoint_key() -> &'static str {
        "mkt.breakdown"
    }
    fn endpoint_description() -> &'static str {
        "売買内訳データ（ロング・信用別の売買代金・株数）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "日付 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            FieldSchema {
                name: "LongSellVa",
                field_type: "number",
                description: "ロング売り売買代金",
            },
            FieldSchema {
                name: "ShrtNoMrgnVa",
                field_type: "number",
                description: "空売り（信用外）売買代金",
            },
            FieldSchema {
                name: "MrgnSellNewVa",
                field_type: "number",
                description: "信用新規売り売買代金",
            },
            FieldSchema {
                name: "MrgnSellCloseVa",
                field_type: "number",
                description: "信用返済売り売買代金",
            },
            FieldSchema {
                name: "LongBuyVa",
                field_type: "number",
                description: "ロング買い売買代金",
            },
            FieldSchema {
                name: "MrgnBuyNewVa",
                field_type: "number",
                description: "信用新規買い売買代金",
            },
            FieldSchema {
                name: "MrgnBuyCloseVa",
                field_type: "number",
                description: "信用返済買い売買代金",
            },
            FieldSchema {
                name: "LongSellVo",
                field_type: "number",
                description: "ロング売り出来高",
            },
            FieldSchema {
                name: "ShrtNoMrgnVo",
                field_type: "number",
                description: "空売り（信用外）出来高",
            },
            FieldSchema {
                name: "MrgnSellNewVo",
                field_type: "number",
                description: "信用新規売り出来高",
            },
            FieldSchema {
                name: "MrgnSellCloseVo",
                field_type: "number",
                description: "信用返済売り出来高",
            },
            FieldSchema {
                name: "LongBuyVo",
                field_type: "number",
                description: "ロング買い出来高",
            },
            FieldSchema {
                name: "MrgnBuyNewVo",
                field_type: "number",
                description: "信用新規買い出来高",
            },
            FieldSchema {
                name: "MrgnBuyCloseVo",
                field_type: "number",
                description: "信用返済買い出来高",
            },
        ]
    }
    fn field_count() -> usize {
        16
    }
}

impl SchemaInfo for MarginAlert {
    fn endpoint_key() -> &'static str {
        "mkt.margin-alert"
    }
    fn endpoint_description() -> &'static str {
        "日々公表信用取引残高（空売り・ロング残高・公表理由）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema { name: "PubDate", field_type: "string", description: "公表日 (YYYY-MM-DD)" },
            FieldSchema { name: "Code", field_type: "string", description: "銘柄コード" },
            FieldSchema { name: "AppDate", field_type: "string", description: "適用日 (YYYY-MM-DD)" },
            FieldSchema { name: "PubReason", field_type: "object", description: "公表理由（Restricted/DailyPublication/Monitoring/RestrictedByJSF/PrecautionByJSF/UnclearOrSecOnAlert）" },
            FieldSchema { name: "ShrtOut", field_type: "string", description: "空売り残高" },
            FieldSchema { name: "ShrtOutChg", field_type: "string", description: "空売り残高変化" },
            FieldSchema { name: "ShrtOutRatio", field_type: "string", description: "空売り残高比率" },
            FieldSchema { name: "LongOut", field_type: "string", description: "ロング残高" },
            FieldSchema { name: "LongOutChg", field_type: "string", description: "ロング残高変化" },
            FieldSchema { name: "LongOutRatio", field_type: "string", description: "ロング残高比率" },
            FieldSchema { name: "SLRatio", field_type: "string", description: "空売り・ロング比率" },
            FieldSchema { name: "ShrtNegOut", field_type: "string", description: "制度空売り残高" },
            FieldSchema { name: "ShrtNegOutChg", field_type: "string", description: "制度空売り残高変化" },
            FieldSchema { name: "ShrtStdOut", field_type: "string", description: "一般空売り残高" },
            FieldSchema { name: "ShrtStdOutChg", field_type: "string", description: "一般空売り残高変化" },
            FieldSchema { name: "LongNegOut", field_type: "string", description: "制度ロング残高" },
            FieldSchema { name: "LongNegOutChg", field_type: "string", description: "制度ロング残高変化" },
            FieldSchema { name: "LongStdOut", field_type: "string", description: "一般ロング残高" },
            FieldSchema { name: "LongStdOutChg", field_type: "string", description: "一般ロング残高変化" },
            FieldSchema { name: "TSEMrgnRegCls", field_type: "string", description: "東証信用規制区分" },
        ]
    }
    fn field_count() -> usize {
        20
    }
}

impl SchemaInfo for MarginInterest {
    fn endpoint_key() -> &'static str {
        "mkt.margin-interest"
    }
    fn endpoint_description() -> &'static str {
        "信用取引週末残高（空売り・ロング別の株数）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "日付 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            FieldSchema {
                name: "ShrtVol",
                field_type: "string",
                description: "空売り残高株数",
            },
            FieldSchema {
                name: "LongVol",
                field_type: "string",
                description: "ロング残高株数",
            },
            FieldSchema {
                name: "ShrtNegVol",
                field_type: "string",
                description: "制度空売り残高株数",
            },
            FieldSchema {
                name: "LongNegVol",
                field_type: "string",
                description: "制度ロング残高株数",
            },
            FieldSchema {
                name: "ShrtStdVol",
                field_type: "string",
                description: "一般空売り残高株数",
            },
            FieldSchema {
                name: "LongStdVol",
                field_type: "string",
                description: "一般ロング残高株数",
            },
            FieldSchema {
                name: "IssType",
                field_type: "string",
                description: "銘柄種別（信用/貸借）",
            },
        ]
    }
    fn field_count() -> usize {
        9
    }
}

impl SchemaInfo for Calendar {
    fn endpoint_key() -> &'static str {
        "mkt.calendar"
    }
    fn endpoint_description() -> &'static str {
        "取引カレンダー（営業日・休業日の区分）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "日付 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "HolDiv",
                field_type: "string",
                description: "休日区分（0=営業日, 1=休業日）",
            },
        ]
    }
    fn field_count() -> usize {
        2
    }
}

impl SchemaInfo for ShortRatio {
    fn endpoint_key() -> &'static str {
        "mkt.short-ratio"
    }
    fn endpoint_description() -> &'static str {
        "業種別空売り比率（33業種コード別の空売り・売買代金）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "日付 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "S33",
                field_type: "string",
                description: "33業種コード（例: 0050）",
            },
            FieldSchema {
                name: "SellExShortVa",
                field_type: "string",
                description: "空売り除き売買代金",
            },
            FieldSchema {
                name: "ShrtWithResVa",
                field_type: "string",
                description: "有報空売り売買代金",
            },
            FieldSchema {
                name: "ShrtNoResVa",
                field_type: "string",
                description: "無報空売り売買代金",
            },
        ]
    }
    fn field_count() -> usize {
        5
    }
}

impl SchemaInfo for ShortSaleReport {
    fn endpoint_key() -> &'static str {
        "mkt.short-sale-report"
    }
    fn endpoint_description() -> &'static str {
        "空売り残高報告（大量空売りポジションの開示情報）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "DiscDate",
                field_type: "string",
                description: "公表日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "CalcDate",
                field_type: "string",
                description: "計算日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            FieldSchema {
                name: "SSName",
                field_type: "string",
                description: "空売り主体名",
            },
            FieldSchema {
                name: "SSAddr",
                field_type: "string",
                description: "空売り主体所在地",
            },
            FieldSchema {
                name: "DICName",
                field_type: "string",
                description: "業務執行組合員名",
            },
            FieldSchema {
                name: "DICAddr",
                field_type: "string",
                description: "業務執行組合員所在地",
            },
            FieldSchema {
                name: "FundName",
                field_type: "string",
                description: "ファンド名",
            },
            FieldSchema {
                name: "ShrtPosToSO",
                field_type: "string",
                description: "空売りポジション（発行済株式比率）",
            },
            FieldSchema {
                name: "ShrtPosShares",
                field_type: "string",
                description: "空売りポジション（株数）",
            },
            FieldSchema {
                name: "ShrtPosUnits",
                field_type: "string",
                description: "空売りポジション（単位数）",
            },
            FieldSchema {
                name: "PrevRptDate",
                field_type: "string",
                description: "前回報告日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "PrevRptRatio",
                field_type: "string",
                description: "前回報告比率",
            },
            FieldSchema {
                name: "Notes",
                field_type: "string",
                description: "備考",
            },
        ]
    }
    fn field_count() -> usize {
        14
    }
}

impl SchemaInfo for Options225Bar {
    fn endpoint_key() -> &'static str {
        "deriv.options-225"
    }
    fn endpoint_description() -> &'static str {
        "日経225オプション四本値（立会別OHLC・建玉・理論価格・IV等）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "日付 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            // ── 全立会合算 ──
            FieldSchema {
                name: "O",
                field_type: "string",
                description: "始値",
            },
            FieldSchema {
                name: "H",
                field_type: "string",
                description: "高値",
            },
            FieldSchema {
                name: "L",
                field_type: "string",
                description: "安値",
            },
            FieldSchema {
                name: "C",
                field_type: "string",
                description: "終値",
            },
            // ── 夕場 ──
            FieldSchema {
                name: "EO",
                field_type: "string",
                description: "夕場始値",
            },
            FieldSchema {
                name: "EH",
                field_type: "string",
                description: "夕場高値",
            },
            FieldSchema {
                name: "EL",
                field_type: "string",
                description: "夕場安値",
            },
            FieldSchema {
                name: "EC",
                field_type: "string",
                description: "夕場終値",
            },
            // ── 後場 ──
            FieldSchema {
                name: "AO",
                field_type: "string",
                description: "後場始値",
            },
            FieldSchema {
                name: "AH",
                field_type: "string",
                description: "後場高値",
            },
            FieldSchema {
                name: "AL",
                field_type: "string",
                description: "後場安値",
            },
            FieldSchema {
                name: "AC",
                field_type: "string",
                description: "後場終値",
            },
            // ── その他 ──
            FieldSchema {
                name: "Vo",
                field_type: "string",
                description: "出来高",
            },
            FieldSchema {
                name: "OI",
                field_type: "string",
                description: "建玉（オープンインタレスト）",
            },
            FieldSchema {
                name: "Va",
                field_type: "string",
                description: "売買代金",
            },
            FieldSchema {
                name: "CM",
                field_type: "string",
                description: "限月 (YYYY-MM)",
            },
            FieldSchema {
                name: "Strike",
                field_type: "string",
                description: "行使価格",
            },
            FieldSchema {
                name: "VoOA",
                field_type: "string",
                description: "立会外出来高",
            },
            FieldSchema {
                name: "EmMrgnTrgDiv",
                field_type: "string",
                description: "緊急証拠金徴収区分",
            },
            FieldSchema {
                name: "PCDiv",
                field_type: "string",
                description: "プット・コール区分（P/C）",
            },
            FieldSchema {
                name: "LTD",
                field_type: "string",
                description: "最終取引日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "SQD",
                field_type: "string",
                description: "SQ日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Settle",
                field_type: "string",
                description: "清算価格",
            },
            FieldSchema {
                name: "Theo",
                field_type: "string",
                description: "理論価格",
            },
            FieldSchema {
                name: "BaseVol",
                field_type: "string",
                description: "基準ボラティリティ",
            },
            FieldSchema {
                name: "UnderPx",
                field_type: "string",
                description: "原資産価格",
            },
            FieldSchema {
                name: "IV",
                field_type: "string",
                description: "インプライドボラティリティ",
            },
            FieldSchema {
                name: "IR",
                field_type: "string",
                description: "金利（リスクフリーレート）",
            },
        ]
    }
    fn field_count() -> usize {
        30
    }
}

impl SchemaInfo for FuturesBar {
    fn endpoint_key() -> &'static str {
        "deriv.futures"
    }
    fn endpoint_description() -> &'static str {
        "先物四本値（立会別OHLC・建玉・清算価格等）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            FieldSchema {
                name: "ProdCat",
                field_type: "string",
                description: "商品区分（TOPIXF/NK225F等）",
            },
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "日付 (YYYY-MM-DD)",
            },
            // ── 全立会合算 ──
            FieldSchema {
                name: "O",
                field_type: "string",
                description: "始値",
            },
            FieldSchema {
                name: "H",
                field_type: "string",
                description: "高値",
            },
            FieldSchema {
                name: "L",
                field_type: "string",
                description: "安値",
            },
            FieldSchema {
                name: "C",
                field_type: "string",
                description: "終値",
            },
            // ── 前場 ──
            FieldSchema {
                name: "MO",
                field_type: "string",
                description: "前場始値",
            },
            FieldSchema {
                name: "MH",
                field_type: "string",
                description: "前場高値",
            },
            FieldSchema {
                name: "ML",
                field_type: "string",
                description: "前場安値",
            },
            FieldSchema {
                name: "MC",
                field_type: "string",
                description: "前場終値",
            },
            // ── 夕場 ──
            FieldSchema {
                name: "EO",
                field_type: "string",
                description: "夕場始値",
            },
            FieldSchema {
                name: "EH",
                field_type: "string",
                description: "夕場高値",
            },
            FieldSchema {
                name: "EL",
                field_type: "string",
                description: "夕場安値",
            },
            FieldSchema {
                name: "EC",
                field_type: "string",
                description: "夕場終値",
            },
            // ── 後場 ──
            FieldSchema {
                name: "AO",
                field_type: "string",
                description: "後場始値",
            },
            FieldSchema {
                name: "AH",
                field_type: "string",
                description: "後場高値",
            },
            FieldSchema {
                name: "AL",
                field_type: "string",
                description: "後場安値",
            },
            FieldSchema {
                name: "AC",
                field_type: "string",
                description: "後場終値",
            },
            // ── その他 ──
            FieldSchema {
                name: "Vo",
                field_type: "string",
                description: "出来高",
            },
            FieldSchema {
                name: "OI",
                field_type: "string",
                description: "建玉（オープンインタレスト）",
            },
            FieldSchema {
                name: "Va",
                field_type: "string",
                description: "売買代金",
            },
            FieldSchema {
                name: "CM",
                field_type: "string",
                description: "限月 (YYYY-MM)",
            },
            FieldSchema {
                name: "VoOA",
                field_type: "string",
                description: "立会外出来高",
            },
            FieldSchema {
                name: "EmMrgnTrgDiv",
                field_type: "string",
                description: "緊急証拠金徴収区分",
            },
            FieldSchema {
                name: "LTD",
                field_type: "string",
                description: "最終取引日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "SQD",
                field_type: "string",
                description: "SQ日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Settle",
                field_type: "string",
                description: "清算価格",
            },
            FieldSchema {
                name: "CCMFlag",
                field_type: "string",
                description: "中心限月フラグ（1=中心限月）",
            },
        ]
    }
    fn field_count() -> usize {
        29
    }
}

impl SchemaInfo for OptionsBar {
    fn endpoint_key() -> &'static str {
        "deriv.options"
    }
    fn endpoint_description() -> &'static str {
        "オプション四本値（有価証券オプション・立会別OHLC・建玉・理論価格・IV等）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            FieldSchema {
                name: "ProdCat",
                field_type: "string",
                description: "商品区分",
            },
            FieldSchema {
                name: "UndSSO",
                field_type: "string",
                description: "原資産の有価証券コード",
            },
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "日付 (YYYY-MM-DD)",
            },
            // ── 全立会合算 ──
            FieldSchema {
                name: "O",
                field_type: "string",
                description: "始値",
            },
            FieldSchema {
                name: "H",
                field_type: "string",
                description: "高値",
            },
            FieldSchema {
                name: "L",
                field_type: "string",
                description: "安値",
            },
            FieldSchema {
                name: "C",
                field_type: "string",
                description: "終値",
            },
            // ── 前場 ──
            FieldSchema {
                name: "MO",
                field_type: "string",
                description: "前場始値",
            },
            FieldSchema {
                name: "MH",
                field_type: "string",
                description: "前場高値",
            },
            FieldSchema {
                name: "ML",
                field_type: "string",
                description: "前場安値",
            },
            FieldSchema {
                name: "MC",
                field_type: "string",
                description: "前場終値",
            },
            // ── 夕場 ──
            FieldSchema {
                name: "EO",
                field_type: "string",
                description: "夕場始値",
            },
            FieldSchema {
                name: "EH",
                field_type: "string",
                description: "夕場高値",
            },
            FieldSchema {
                name: "EL",
                field_type: "string",
                description: "夕場安値",
            },
            FieldSchema {
                name: "EC",
                field_type: "string",
                description: "夕場終値",
            },
            // ── 後場 ──
            FieldSchema {
                name: "AO",
                field_type: "string",
                description: "後場始値",
            },
            FieldSchema {
                name: "AH",
                field_type: "string",
                description: "後場高値",
            },
            FieldSchema {
                name: "AL",
                field_type: "string",
                description: "後場安値",
            },
            FieldSchema {
                name: "AC",
                field_type: "string",
                description: "後場終値",
            },
            // ── その他 ──
            FieldSchema {
                name: "Vo",
                field_type: "string",
                description: "出来高",
            },
            FieldSchema {
                name: "OI",
                field_type: "string",
                description: "建玉（オープンインタレスト）",
            },
            FieldSchema {
                name: "Va",
                field_type: "string",
                description: "売買代金",
            },
            FieldSchema {
                name: "CM",
                field_type: "string",
                description: "限月 (YYYY-MM)",
            },
            FieldSchema {
                name: "Strike",
                field_type: "string",
                description: "行使価格",
            },
            FieldSchema {
                name: "VoOA",
                field_type: "string",
                description: "立会外出来高",
            },
            FieldSchema {
                name: "EmMrgnTrgDiv",
                field_type: "string",
                description: "緊急証拠金徴収区分",
            },
            FieldSchema {
                name: "PCDiv",
                field_type: "string",
                description: "プット・コール区分（P/C）",
            },
            FieldSchema {
                name: "LTD",
                field_type: "string",
                description: "最終取引日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "SQD",
                field_type: "string",
                description: "SQ日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Settle",
                field_type: "string",
                description: "清算価格",
            },
            FieldSchema {
                name: "Theo",
                field_type: "string",
                description: "理論価格",
            },
            FieldSchema {
                name: "BaseVol",
                field_type: "string",
                description: "基準ボラティリティ",
            },
            FieldSchema {
                name: "UnderPx",
                field_type: "string",
                description: "原資産価格",
            },
            FieldSchema {
                name: "IV",
                field_type: "string",
                description: "インプライドボラティリティ",
            },
            FieldSchema {
                name: "IR",
                field_type: "string",
                description: "金利（リスクフリーレート）",
            },
            FieldSchema {
                name: "CCMFlag",
                field_type: "string",
                description: "中心限月フラグ（1=中心限月）",
            },
        ]
    }
    fn field_count() -> usize {
        37
    }
}

impl SchemaInfo for FinsDetails {
    fn endpoint_key() -> &'static str {
        "fins.details"
    }
    fn endpoint_description() -> &'static str {
        "財務諸表（BS/PL/CF等の詳細財務データ。--output json で完全取得推奨）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema { name: "DiscDate", field_type: "string", description: "開示日 (YYYY-MM-DD)" },
            FieldSchema { name: "DiscTime", field_type: "string", description: "開示時刻 (HH:MM:SS)" },
            FieldSchema { name: "Code", field_type: "string", description: "銘柄コード" },
            FieldSchema { name: "DiscNo", field_type: "string", description: "開示番号" },
            FieldSchema { name: "DocType", field_type: "string", description: "書類種別（決算短信等）" },
            FieldSchema { name: "FS", field_type: "object", description: "財務諸表データ（BS/PL/CF等。キーは銘柄・決算種別により異なる。--output json で完全表示）" },
        ]
    }
    fn field_count() -> usize {
        6
    }
}

impl SchemaInfo for FinsDividend {
    fn endpoint_key() -> &'static str {
        "fins.dividend"
    }
    fn endpoint_description() -> &'static str {
        "配当金情報（配当率・権利確定日・支払日等）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "PubDate",
                field_type: "string",
                description: "公表日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "PubTime",
                field_type: "string",
                description: "公表時刻 (HH:MM:SS)",
            },
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            FieldSchema {
                name: "RefNo",
                field_type: "string",
                description: "参照番号",
            },
            FieldSchema {
                name: "StatCode",
                field_type: "string",
                description: "状態コード",
            },
            FieldSchema {
                name: "BoardDate",
                field_type: "string",
                description: "取締役会決議日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "IFCode",
                field_type: "string",
                description: "配当種類コード",
            },
            FieldSchema {
                name: "FRCode",
                field_type: "string",
                description: "決算期区分コード",
            },
            FieldSchema {
                name: "IFTerm",
                field_type: "string",
                description: "配当対象期間",
            },
            FieldSchema {
                name: "DivRate",
                field_type: "string",
                description: "1株当たり配当金額",
            },
            FieldSchema {
                name: "RecDate",
                field_type: "string",
                description: "権利確定日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "ExDate",
                field_type: "string",
                description: "権利落ち日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "ActRecDate",
                field_type: "string",
                description: "実際の権利確定日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "PayDate",
                field_type: "string",
                description: "配当支払日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "CARefNo",
                field_type: "string",
                description: "コーポレートアクション参照番号",
            },
            FieldSchema {
                name: "DistAmt",
                field_type: "string",
                description: "分配金額",
            },
            FieldSchema {
                name: "RetEarn",
                field_type: "string",
                description: "利益剰余金",
            },
            FieldSchema {
                name: "DeemDiv",
                field_type: "string",
                description: "みなし配当",
            },
            FieldSchema {
                name: "DeemCapGains",
                field_type: "string",
                description: "みなしキャピタルゲイン",
            },
            FieldSchema {
                name: "NetAssetDecRatio",
                field_type: "string",
                description: "純資産減少率",
            },
            FieldSchema {
                name: "CommSpecCode",
                field_type: "string",
                description: "普通配当・特別配当区分コード",
            },
            FieldSchema {
                name: "CommDivRate",
                field_type: "string",
                description: "普通配当額",
            },
            FieldSchema {
                name: "SpecDivRate",
                field_type: "string",
                description: "特別配当額",
            },
        ]
    }
    fn field_count() -> usize {
        23
    }
}

impl SchemaInfo for FinsSummary {
    fn endpoint_key() -> &'static str {
        "fins.summary"
    }
    fn endpoint_description() -> &'static str {
        "財務情報サマリー（実績・予想の売上・利益・配当・バランスシート等）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            // ── 基本情報 ──
            FieldSchema {
                name: "DiscDate",
                field_type: "string",
                description: "開示日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "DiscTime",
                field_type: "string",
                description: "開示時刻 (HH:MM:SS)",
            },
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "銘柄コード",
            },
            FieldSchema {
                name: "DiscNo",
                field_type: "string",
                description: "開示番号",
            },
            FieldSchema {
                name: "DocType",
                field_type: "string",
                description: "書類種別",
            },
            FieldSchema {
                name: "CurPerType",
                field_type: "string",
                description: "当期種別（通期/四半期）",
            },
            FieldSchema {
                name: "CurPerSt",
                field_type: "string",
                description: "当期開始日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "CurPerEn",
                field_type: "string",
                description: "当期終了日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "CurFYSt",
                field_type: "string",
                description: "当会計年度開始日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "CurFYEn",
                field_type: "string",
                description: "当会計年度終了日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "NxtFYSt",
                field_type: "string",
                description: "翌会計年度開始日 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "NxtFYEn",
                field_type: "string",
                description: "翌会計年度終了日 (YYYY-MM-DD)",
            },
            // ── 実績値 ──
            FieldSchema {
                name: "Sales",
                field_type: "string",
                description: "売上高（実績）",
            },
            FieldSchema {
                name: "OP",
                field_type: "string",
                description: "営業利益（実績）",
            },
            FieldSchema {
                name: "OdP",
                field_type: "string",
                description: "経常利益（実績）",
            },
            FieldSchema {
                name: "NP",
                field_type: "string",
                description: "純利益（実績）",
            },
            FieldSchema {
                name: "EPS",
                field_type: "string",
                description: "1株当たり純利益（実績）",
            },
            FieldSchema {
                name: "DEPS",
                field_type: "string",
                description: "希薄化後EPS（実績）",
            },
            FieldSchema {
                name: "TA",
                field_type: "string",
                description: "総資産（実績）",
            },
            FieldSchema {
                name: "Eq",
                field_type: "string",
                description: "純資産（実績）",
            },
            FieldSchema {
                name: "EqAR",
                field_type: "string",
                description: "自己資本比率（実績）",
            },
            FieldSchema {
                name: "BPS",
                field_type: "string",
                description: "1株当たり純資産（実績）",
            },
            FieldSchema {
                name: "CFO",
                field_type: "string",
                description: "営業CF（実績）",
            },
            FieldSchema {
                name: "CFI",
                field_type: "string",
                description: "投資CF（実績）",
            },
            FieldSchema {
                name: "CFF",
                field_type: "string",
                description: "財務CF（実績）",
            },
            FieldSchema {
                name: "CashEq",
                field_type: "string",
                description: "期末現金・現金同等物（実績）",
            },
            // ── 配当（当期・実績）──
            FieldSchema {
                name: "Div1Q",
                field_type: "string",
                description: "第1四半期配当（実績）",
            },
            FieldSchema {
                name: "Div2Q",
                field_type: "string",
                description: "第2四半期配当（実績）",
            },
            FieldSchema {
                name: "Div3Q",
                field_type: "string",
                description: "第3四半期配当（実績）",
            },
            FieldSchema {
                name: "DivFY",
                field_type: "string",
                description: "期末配当（実績）",
            },
            FieldSchema {
                name: "DivAnn",
                field_type: "string",
                description: "年間配当（実績）",
            },
            FieldSchema {
                name: "DivUnit",
                field_type: "string",
                description: "配当単位（実績）",
            },
            FieldSchema {
                name: "DivTotalAnn",
                field_type: "string",
                description: "年間配当総額（実績）",
            },
            FieldSchema {
                name: "PayoutRatioAnn",
                field_type: "string",
                description: "配当性向（実績）",
            },
            // ── 配当（当期・予想）──
            FieldSchema {
                name: "FDiv1Q",
                field_type: "string",
                description: "第1四半期配当（当期予想）",
            },
            FieldSchema {
                name: "FDiv2Q",
                field_type: "string",
                description: "第2四半期配当（当期予想）",
            },
            FieldSchema {
                name: "FDiv3Q",
                field_type: "string",
                description: "第3四半期配当（当期予想）",
            },
            FieldSchema {
                name: "FDivFY",
                field_type: "string",
                description: "期末配当（当期予想）",
            },
            FieldSchema {
                name: "FDivAnn",
                field_type: "string",
                description: "年間配当（当期予想）",
            },
            FieldSchema {
                name: "FDivUnit",
                field_type: "string",
                description: "配当単位（当期予想）",
            },
            FieldSchema {
                name: "FDivTotalAnn",
                field_type: "string",
                description: "年間配当総額（当期予想）",
            },
            FieldSchema {
                name: "FPayoutRatioAnn",
                field_type: "string",
                description: "配当性向（当期予想）",
            },
            // ── 配当（翌期・予想）──
            FieldSchema {
                name: "NxFDiv1Q",
                field_type: "string",
                description: "第1四半期配当（翌期予想）",
            },
            FieldSchema {
                name: "NxFDiv2Q",
                field_type: "string",
                description: "第2四半期配当（翌期予想）",
            },
            FieldSchema {
                name: "NxFDiv3Q",
                field_type: "string",
                description: "第3四半期配当（翌期予想）",
            },
            FieldSchema {
                name: "NxFDivFY",
                field_type: "string",
                description: "期末配当（翌期予想）",
            },
            FieldSchema {
                name: "NxFDivAnn",
                field_type: "string",
                description: "年間配当（翌期予想）",
            },
            FieldSchema {
                name: "NxFDivUnit",
                field_type: "string",
                description: "配当単位（翌期予想）",
            },
            FieldSchema {
                name: "NxFPayoutRatioAnn",
                field_type: "string",
                description: "配当性向（翌期予想）",
            },
            // ── 業績予想（当期・第2四半期）──
            FieldSchema {
                name: "FSales2Q",
                field_type: "string",
                description: "売上高第2四半期予想（当期）",
            },
            FieldSchema {
                name: "FOP2Q",
                field_type: "string",
                description: "営業利益第2四半期予想（当期）",
            },
            FieldSchema {
                name: "FOdP2Q",
                field_type: "string",
                description: "経常利益第2四半期予想（当期）",
            },
            FieldSchema {
                name: "FNP2Q",
                field_type: "string",
                description: "純利益第2四半期予想（当期）",
            },
            FieldSchema {
                name: "FEPS2Q",
                field_type: "string",
                description: "EPS第2四半期予想（当期）",
            },
            // ── 業績予想（翌期・第2四半期）──
            FieldSchema {
                name: "NxFSales2Q",
                field_type: "string",
                description: "売上高第2四半期予想（翌期）",
            },
            FieldSchema {
                name: "NxFOP2Q",
                field_type: "string",
                description: "営業利益第2四半期予想（翌期）",
            },
            FieldSchema {
                name: "NxFOdP2Q",
                field_type: "string",
                description: "経常利益第2四半期予想（翌期）",
            },
            FieldSchema {
                name: "NxFNp2Q",
                field_type: "string",
                description: "純利益第2四半期予想（翌期）",
            },
            FieldSchema {
                name: "NxFEPS2Q",
                field_type: "string",
                description: "EPS第2四半期予想（翌期）",
            },
            // ── 業績予想（当期・通期）──
            FieldSchema {
                name: "FSales",
                field_type: "string",
                description: "売上高通期予想（当期）",
            },
            FieldSchema {
                name: "FOP",
                field_type: "string",
                description: "営業利益通期予想（当期）",
            },
            FieldSchema {
                name: "FOdP",
                field_type: "string",
                description: "経常利益通期予想（当期）",
            },
            FieldSchema {
                name: "FNP",
                field_type: "string",
                description: "純利益通期予想（当期）",
            },
            FieldSchema {
                name: "FEPS",
                field_type: "string",
                description: "EPS通期予想（当期）",
            },
            // ── 業績予想（翌期・通期）──
            FieldSchema {
                name: "NxFSales",
                field_type: "string",
                description: "売上高通期予想（翌期）",
            },
            FieldSchema {
                name: "NxFOP",
                field_type: "string",
                description: "営業利益通期予想（翌期）",
            },
            FieldSchema {
                name: "NxFOdP",
                field_type: "string",
                description: "経常利益通期予想（翌期）",
            },
            FieldSchema {
                name: "NxFNp",
                field_type: "string",
                description: "純利益通期予想（翌期）",
            },
            FieldSchema {
                name: "NxFEPS",
                field_type: "string",
                description: "EPS通期予想（翌期）",
            },
            // ── フラグ・注記 ──
            FieldSchema {
                name: "MatChgSub",
                field_type: "string",
                description: "実質的な支配の変更フラグ",
            },
            FieldSchema {
                name: "SigChgInC",
                field_type: "string",
                description: "連結範囲重要変更フラグ",
            },
            FieldSchema {
                name: "ChgByASRev",
                field_type: "string",
                description: "会計基準変更フラグ",
            },
            FieldSchema {
                name: "ChgNoASRev",
                field_type: "string",
                description: "会計基準変更以外フラグ",
            },
            FieldSchema {
                name: "ChgAcEst",
                field_type: "string",
                description: "会計上の見積変更フラグ",
            },
            FieldSchema {
                name: "RetroRst",
                field_type: "string",
                description: "遡及修正フラグ",
            },
            // ── 株式数 ──
            FieldSchema {
                name: "ShOutFY",
                field_type: "string",
                description: "期末発行済株式数",
            },
            FieldSchema {
                name: "TrShFY",
                field_type: "string",
                description: "期末自己株式数",
            },
            FieldSchema {
                name: "AvgSh",
                field_type: "string",
                description: "期中平均株式数",
            },
            // ── 連結実績 ──
            FieldSchema {
                name: "NCSales",
                field_type: "string",
                description: "連結売上高（実績）",
            },
            FieldSchema {
                name: "NCOP",
                field_type: "string",
                description: "連結営業利益（実績）",
            },
            FieldSchema {
                name: "NCOdP",
                field_type: "string",
                description: "連結経常利益（実績）",
            },
            FieldSchema {
                name: "NCNP",
                field_type: "string",
                description: "連結純利益（実績）",
            },
            FieldSchema {
                name: "NCEPS",
                field_type: "string",
                description: "連結EPS（実績）",
            },
            FieldSchema {
                name: "NCTA",
                field_type: "string",
                description: "連結総資産（実績）",
            },
            FieldSchema {
                name: "NCEq",
                field_type: "string",
                description: "連結純資産（実績）",
            },
            FieldSchema {
                name: "NCEqAR",
                field_type: "string",
                description: "連結自己資本比率（実績）",
            },
            FieldSchema {
                name: "NCBPS",
                field_type: "string",
                description: "連結BPS（実績）",
            },
            // ── 連結予想（当期・第2四半期）──
            FieldSchema {
                name: "FNCSales2Q",
                field_type: "string",
                description: "連結売上高第2四半期予想（当期）",
            },
            FieldSchema {
                name: "FNCOP2Q",
                field_type: "string",
                description: "連結営業利益第2四半期予想（当期）",
            },
            FieldSchema {
                name: "FNCOdP2Q",
                field_type: "string",
                description: "連結経常利益第2四半期予想（当期）",
            },
            FieldSchema {
                name: "FNCNP2Q",
                field_type: "string",
                description: "連結純利益第2四半期予想（当期）",
            },
            FieldSchema {
                name: "FNCEPS2Q",
                field_type: "string",
                description: "連結EPS第2四半期予想（当期）",
            },
            // ── 連結予想（翌期・第2四半期）──
            FieldSchema {
                name: "NxFNCSales2Q",
                field_type: "string",
                description: "連結売上高第2四半期予想（翌期）",
            },
            FieldSchema {
                name: "NxFNCOP2Q",
                field_type: "string",
                description: "連結営業利益第2四半期予想（翌期）",
            },
            FieldSchema {
                name: "NxFNCOdP2Q",
                field_type: "string",
                description: "連結経常利益第2四半期予想（翌期）",
            },
            FieldSchema {
                name: "NxFNCNP2Q",
                field_type: "string",
                description: "連結純利益第2四半期予想（翌期）",
            },
            FieldSchema {
                name: "NxFNCEPS2Q",
                field_type: "string",
                description: "連結EPS第2四半期予想（翌期）",
            },
            // ── 連結予想（当期・通期）──
            FieldSchema {
                name: "FNCSales",
                field_type: "string",
                description: "連結売上高通期予想（当期）",
            },
            FieldSchema {
                name: "FNCOP",
                field_type: "string",
                description: "連結営業利益通期予想（当期）",
            },
            FieldSchema {
                name: "FNCOdP",
                field_type: "string",
                description: "連結経常利益通期予想（当期）",
            },
            FieldSchema {
                name: "FNCNP",
                field_type: "string",
                description: "連結純利益通期予想（当期）",
            },
            FieldSchema {
                name: "FNCEPS",
                field_type: "string",
                description: "連結EPS通期予想（当期）",
            },
            // ── 連結予想（翌期・通期）──
            FieldSchema {
                name: "NxFNCSales",
                field_type: "string",
                description: "連結売上高通期予想（翌期）",
            },
            FieldSchema {
                name: "NxFNCOP",
                field_type: "string",
                description: "連結営業利益通期予想（翌期）",
            },
            FieldSchema {
                name: "NxFNCOdP",
                field_type: "string",
                description: "連結経常利益通期予想（翌期）",
            },
            FieldSchema {
                name: "NxFNCNP",
                field_type: "string",
                description: "連結純利益通期予想（翌期）",
            },
            FieldSchema {
                name: "NxFNCEPS",
                field_type: "string",
                description: "連結EPS通期予想（翌期）",
            },
        ]
    }
    fn field_count() -> usize {
        107
    }
}

impl SchemaInfo for TopixDailyBar {
    fn endpoint_key() -> &'static str {
        "idx.daily-topix"
    }
    fn endpoint_description() -> &'static str {
        "TOPIX指数四本値日足（始値・高値・安値・終値）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "日付 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "O",
                field_type: "string",
                description: "始値",
            },
            FieldSchema {
                name: "H",
                field_type: "string",
                description: "高値",
            },
            FieldSchema {
                name: "L",
                field_type: "string",
                description: "安値",
            },
            FieldSchema {
                name: "C",
                field_type: "string",
                description: "終値",
            },
        ]
    }
    fn field_count() -> usize {
        5
    }
}

impl SchemaInfo for IndexDailyBar {
    fn endpoint_key() -> &'static str {
        "idx.daily"
    }
    fn endpoint_description() -> &'static str {
        "各種指数四本値日足（指数コード別の始値・高値・安値・終値）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Date",
                field_type: "string",
                description: "日付 (YYYY-MM-DD)",
            },
            FieldSchema {
                name: "Code",
                field_type: "string",
                description: "指数コード（例: 0000=TOPIX, 0028=東証マザーズ指数）",
            },
            FieldSchema {
                name: "O",
                field_type: "string",
                description: "始値",
            },
            FieldSchema {
                name: "H",
                field_type: "string",
                description: "高値",
            },
            FieldSchema {
                name: "L",
                field_type: "string",
                description: "安値",
            },
            FieldSchema {
                name: "C",
                field_type: "string",
                description: "終値",
            },
        ]
    }
    fn field_count() -> usize {
        6
    }
}

impl SchemaInfo for BulkListItem {
    fn endpoint_key() -> &'static str {
        "bulk.list"
    }
    fn endpoint_description() -> &'static str {
        "バルクダウンロード可能ファイル一覧（キー名・更新日時・ファイルサイズ）"
    }
    fn field_schemas() -> Vec<FieldSchema> {
        vec![
            FieldSchema {
                name: "Key",
                field_type: "string",
                description: "ファイルキー（bulk get で使用）",
            },
            FieldSchema {
                name: "LastModified",
                field_type: "string",
                description: "最終更新日時",
            },
            FieldSchema {
                name: "Size",
                field_type: "integer",
                description: "ファイルサイズ（バイト）",
            },
        ]
    }
    fn field_count() -> usize {
        3
    }
}

// ── レジストリ関数 ────────────────────────────────────────────────────────────

fn make_summary<T: SchemaInfo>() -> EndpointSchema {
    EndpointSchema {
        key: T::endpoint_key(),
        description: T::endpoint_description(),
        field_count: T::field_count(),
    }
}

/// 全エンドポイントを単一の型リストから登録するマクロ。
/// `all_endpoint_schemas`・`lookup_endpoint`・`all_endpoint_keys` の3関数を生成する。
macro_rules! register_endpoints {
    ( $( $( $ty:ty ),+ );+ $(;)? ) => {
        /// 全エンドポイントの一覧（論理的なグループ順）
        pub fn all_endpoint_schemas() -> Vec<EndpointSchema> {
            vec![ $( $( make_summary::<$ty>(), )+ )+ ]
        }

        /// エンドポイントキーからフィールドスキーマを検索
        pub fn lookup_endpoint(key: &str) -> Option<Vec<FieldSchema>> {
            $(
                $(
                    if key == <$ty as SchemaInfo>::endpoint_key() {
                        return Some(<$ty>::field_schemas());
                    }
                )+
            )+
            None
        }

        /// エラーメッセージ用: 全有効キーの一覧
        pub fn all_endpoint_keys() -> Vec<&'static str> {
            vec![ $( $( <$ty as SchemaInfo>::endpoint_key(), )+ )+ ]
        }
    };
}

register_endpoints! {
    // eq グループ
    StockMaster, AmBar, MinuteBar, DailyBar, EarningsCalendar, InvestorType;
    // mkt グループ
    Breakdown, MarginAlert, MarginInterest, Calendar, ShortRatio, ShortSaleReport;
    // deriv グループ
    Options225Bar, FuturesBar, OptionsBar;
    // fins グループ
    FinsDetails, FinsDividend, FinsSummary;
    // idx グループ
    TopixDailyBar, IndexDailyBar;
    // bulk グループ
    BulkListItem;
}

// ── テスト ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_endpoints_count() {
        assert_eq!(all_endpoint_schemas().len(), 21);
    }

    #[test]
    fn test_lookup_known_endpoint() {
        assert!(lookup_endpoint("eq.daily").is_some());
        assert!(lookup_endpoint("fins.summary").is_some());
        assert!(lookup_endpoint("bulk.list").is_some());
    }

    #[test]
    fn test_lookup_unknown_endpoint() {
        assert!(lookup_endpoint("nonexistent").is_none());
        assert!(lookup_endpoint("eq.trades").is_none());
        assert!(lookup_endpoint("").is_none());
    }

    #[test]
    fn test_field_count_consistency() {
        for schema in all_endpoint_schemas() {
            let actual = lookup_endpoint(schema.key).unwrap().len();
            assert_eq!(
                schema.field_count, actual,
                "field_count mismatch for {}: expected {}, got {}",
                schema.key, schema.field_count, actual
            );
        }
    }

    #[test]
    fn test_no_duplicate_keys() {
        let schemas = all_endpoint_schemas();
        let mut keys: Vec<&str> = schemas.iter().map(|s| s.key).collect();
        keys.sort_unstable();
        let len_before = keys.len();
        keys.dedup();
        assert_eq!(len_before, keys.len(), "Duplicate endpoint keys found");
    }

    #[test]
    fn test_no_duplicate_field_names() {
        for schema in all_endpoint_schemas() {
            let fields = lookup_endpoint(schema.key).unwrap();
            let mut names: Vec<&str> = fields.iter().map(|f| f.name).collect();
            names.sort_unstable();
            let len_before = names.len();
            names.dedup();
            assert_eq!(
                len_before,
                names.len(),
                "Duplicate field names in {}",
                schema.key
            );
        }
    }
}

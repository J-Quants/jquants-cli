use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt;
use std::ops::Deref;

// ── FlexString: JSON の string/number/null を文字列として受け取る newtype ──

#[derive(Debug, Clone)]
pub struct FlexString(pub String);

impl<'de> Deserialize<'de> for FlexString {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let s = match value {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
            Value::Null => String::new(),
            other => other.to_string(),
        };
        Ok(FlexString(s))
    }
}

impl Serialize for FlexString {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl fmt::Display for FlexString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Deref for FlexString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

// ── ジェネリック ApiResponse<T> ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Vec<T>,
    pub pagination_key: Option<String>,
}

// ── 個別データ型 ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StockMaster {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "CoName")]
    pub co_name: String,
    #[serde(rename = "CoNameEn")]
    pub co_name_en: String,
    #[serde(rename = "S17")]
    pub sector17_code: String,
    #[serde(rename = "S17Nm")]
    pub sector17_code_name: String,
    #[serde(rename = "S33")]
    pub sector33_code: String,
    #[serde(rename = "S33Nm")]
    pub sector33_code_name: String,
    #[serde(rename = "ScaleCat")]
    pub scale_category: String,
    #[serde(rename = "Mkt")]
    pub market_code: String,
    #[serde(rename = "MktNm")]
    pub market_code_name: String,
    #[serde(rename = "Mrgn")]
    pub margin_code: String,
    #[serde(rename = "MrgnNm")]
    pub margin_code_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AmBar {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "MO")]
    pub morning_open: Option<f64>,
    #[serde(rename = "MH")]
    pub morning_high: Option<f64>,
    #[serde(rename = "ML")]
    pub morning_low: Option<f64>,
    #[serde(rename = "MC")]
    pub morning_close: Option<f64>,
    #[serde(rename = "MVo")]
    pub morning_volume: Option<f64>,
    #[serde(rename = "MVa")]
    pub morning_turnover: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Breakdown {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "LongSellVa")]
    pub long_sell_va: f64,
    #[serde(rename = "ShrtNoMrgnVa")]
    pub shrt_no_mrgn_va: f64,
    #[serde(rename = "MrgnSellNewVa")]
    pub mrgn_sell_new_va: f64,
    #[serde(rename = "MrgnSellCloseVa")]
    pub mrgn_sell_close_va: f64,
    #[serde(rename = "LongBuyVa")]
    pub long_buy_va: f64,
    #[serde(rename = "MrgnBuyNewVa")]
    pub mrgn_buy_new_va: f64,
    #[serde(rename = "MrgnBuyCloseVa")]
    pub mrgn_buy_close_va: f64,
    #[serde(rename = "LongSellVo")]
    pub long_sell_vo: f64,
    #[serde(rename = "ShrtNoMrgnVo")]
    pub shrt_no_mrgn_vo: f64,
    #[serde(rename = "MrgnSellNewVo")]
    pub mrgn_sell_new_vo: f64,
    #[serde(rename = "MrgnSellCloseVo")]
    pub mrgn_sell_close_vo: f64,
    #[serde(rename = "LongBuyVo")]
    pub long_buy_vo: f64,
    #[serde(rename = "MrgnBuyNewVo")]
    pub mrgn_buy_new_vo: f64,
    #[serde(rename = "MrgnBuyCloseVo")]
    pub mrgn_buy_close_vo: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MinuteBar {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Time")]
    pub time: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "O")]
    pub open: f64,
    #[serde(rename = "H")]
    pub high: f64,
    #[serde(rename = "L")]
    pub low: f64,
    #[serde(rename = "C")]
    pub close: f64,
    #[serde(rename = "Vo")]
    pub volume: f64,
    #[serde(rename = "Va")]
    pub turnover: f64,
}

// DailyBar: 35 フィールドのため Clone 削除
#[derive(Debug, Deserialize, Serialize)]
pub struct DailyBar {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "O")]
    pub open: Option<f64>,
    #[serde(rename = "H")]
    pub high: Option<f64>,
    #[serde(rename = "L")]
    pub low: Option<f64>,
    #[serde(rename = "C")]
    pub close: Option<f64>,
    #[serde(rename = "UL")]
    pub upper_limit: Option<String>,
    #[serde(rename = "LL")]
    pub lower_limit: Option<String>,
    #[serde(rename = "Vo")]
    pub volume: Option<f64>,
    #[serde(rename = "Va")]
    pub turnover: Option<f64>,
    #[serde(rename = "AdjFactor")]
    pub adj_factor: Option<f64>,
    #[serde(rename = "AdjO")]
    pub adj_open: Option<f64>,
    #[serde(rename = "AdjH")]
    pub adj_high: Option<f64>,
    #[serde(rename = "AdjL")]
    pub adj_low: Option<f64>,
    #[serde(rename = "AdjC")]
    pub adj_close: Option<f64>,
    #[serde(rename = "AdjVo")]
    pub adj_volume: Option<f64>,
    #[serde(rename = "MO")]
    pub morning_open: Option<f64>,
    #[serde(rename = "MH")]
    pub morning_high: Option<f64>,
    #[serde(rename = "ML")]
    pub morning_low: Option<f64>,
    #[serde(rename = "MC")]
    pub morning_close: Option<f64>,
    #[serde(rename = "MUL")]
    pub morning_upper_limit: Option<String>,
    #[serde(rename = "MLL")]
    pub morning_lower_limit: Option<String>,
    #[serde(rename = "MVo")]
    pub morning_volume: Option<f64>,
    #[serde(rename = "MVa")]
    pub morning_turnover: Option<f64>,
    #[serde(rename = "MAdjO")]
    pub morning_adj_open: Option<f64>,
    #[serde(rename = "MAdjH")]
    pub morning_adj_high: Option<f64>,
    #[serde(rename = "MAdjL")]
    pub morning_adj_low: Option<f64>,
    #[serde(rename = "MAdjC")]
    pub morning_adj_close: Option<f64>,
    #[serde(rename = "MAdjVo")]
    pub morning_adj_volume: Option<f64>,
    #[serde(rename = "AO")]
    pub afternoon_open: Option<f64>,
    #[serde(rename = "AH")]
    pub afternoon_high: Option<f64>,
    #[serde(rename = "AL")]
    pub afternoon_low: Option<f64>,
    #[serde(rename = "AC")]
    pub afternoon_close: Option<f64>,
    #[serde(rename = "AUL")]
    pub afternoon_upper_limit: Option<String>,
    #[serde(rename = "ALL")]
    pub afternoon_lower_limit: Option<String>,
    #[serde(rename = "AVo")]
    pub afternoon_volume: Option<f64>,
    #[serde(rename = "AVa")]
    pub afternoon_turnover: Option<f64>,
    #[serde(rename = "AAdjO")]
    pub afternoon_adj_open: Option<f64>,
    #[serde(rename = "AAdjH")]
    pub afternoon_adj_high: Option<f64>,
    #[serde(rename = "AAdjL")]
    pub afternoon_adj_low: Option<f64>,
    #[serde(rename = "AAdjC")]
    pub afternoon_adj_close: Option<f64>,
    #[serde(rename = "AAdjVo")]
    pub afternoon_adj_volume: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct BulkGetResponse {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BulkListItem {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "Size")]
    pub size: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Calendar {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "HolDiv")]
    pub hol_div: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EarningsCalendar {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "CoName")]
    pub co_name: String,
    #[serde(rename = "FY")]
    pub fy: String,
    #[serde(rename = "SectorNm")]
    pub sector_nm: String,
    #[serde(rename = "FQ")]
    pub fq: String,
    #[serde(rename = "Section")]
    pub section: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Options225Bar {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "O")]
    pub open: FlexString,
    #[serde(rename = "H")]
    pub high: FlexString,
    #[serde(rename = "L")]
    pub low: FlexString,
    #[serde(rename = "C")]
    pub close: FlexString,
    #[serde(rename = "EO")]
    pub evening_open: FlexString,
    #[serde(rename = "EH")]
    pub evening_high: FlexString,
    #[serde(rename = "EL")]
    pub evening_low: FlexString,
    #[serde(rename = "EC")]
    pub evening_close: FlexString,
    #[serde(rename = "AO")]
    pub afternoon_open: FlexString,
    #[serde(rename = "AH")]
    pub afternoon_high: FlexString,
    #[serde(rename = "AL")]
    pub afternoon_low: FlexString,
    #[serde(rename = "AC")]
    pub afternoon_close: FlexString,
    #[serde(rename = "Vo")]
    pub volume: FlexString,
    #[serde(rename = "OI")]
    pub open_interest: FlexString,
    #[serde(rename = "Va")]
    pub turnover: FlexString,
    #[serde(rename = "CM")]
    pub contract_month: String,
    #[serde(rename = "Strike")]
    pub strike: FlexString,
    #[serde(rename = "VoOA")]
    pub volume_on_auction: FlexString,
    #[serde(rename = "EmMrgnTrgDiv")]
    pub em_mrgn_trg_div: String,
    #[serde(rename = "PCDiv")]
    pub pc_div: String,
    #[serde(rename = "LTD")]
    pub last_trading_date: String,
    #[serde(rename = "SQD")]
    pub sq_date: String,
    #[serde(rename = "Settle")]
    pub settle: FlexString,
    #[serde(rename = "Theo")]
    pub theo: FlexString,
    #[serde(rename = "BaseVol")]
    pub base_vol: FlexString,
    #[serde(rename = "UnderPx")]
    pub under_px: FlexString,
    #[serde(rename = "IV")]
    pub iv: FlexString,
    #[serde(rename = "IR")]
    pub ir: FlexString,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FuturesBar {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "ProdCat")]
    pub prod_cat: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "O")]
    pub open: FlexString,
    #[serde(rename = "H")]
    pub high: FlexString,
    #[serde(rename = "L")]
    pub low: FlexString,
    #[serde(rename = "C")]
    pub close: FlexString,
    #[serde(rename = "MO")]
    pub morning_open: FlexString,
    #[serde(rename = "MH")]
    pub morning_high: FlexString,
    #[serde(rename = "ML")]
    pub morning_low: FlexString,
    #[serde(rename = "MC")]
    pub morning_close: FlexString,
    #[serde(rename = "EO")]
    pub evening_open: FlexString,
    #[serde(rename = "EH")]
    pub evening_high: FlexString,
    #[serde(rename = "EL")]
    pub evening_low: FlexString,
    #[serde(rename = "EC")]
    pub evening_close: FlexString,
    #[serde(rename = "AO")]
    pub afternoon_open: FlexString,
    #[serde(rename = "AH")]
    pub afternoon_high: FlexString,
    #[serde(rename = "AL")]
    pub afternoon_low: FlexString,
    #[serde(rename = "AC")]
    pub afternoon_close: FlexString,
    #[serde(rename = "Vo")]
    pub volume: FlexString,
    #[serde(rename = "OI")]
    pub open_interest: FlexString,
    #[serde(rename = "Va")]
    pub turnover: FlexString,
    #[serde(rename = "CM")]
    pub contract_month: String,
    #[serde(rename = "VoOA")]
    pub volume_on_auction: FlexString,
    #[serde(rename = "EmMrgnTrgDiv")]
    pub em_mrgn_trg_div: String,
    #[serde(rename = "LTD")]
    pub last_trading_date: String,
    #[serde(rename = "SQD")]
    pub sq_date: String,
    #[serde(rename = "Settle")]
    pub settle: FlexString,
    #[serde(rename = "CCMFlag")]
    pub ccm_flag: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OptionsBar {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "ProdCat")]
    pub prod_cat: String,
    #[serde(rename = "UndSSO")]
    pub und_sso: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "O")]
    pub open: FlexString,
    #[serde(rename = "H")]
    pub high: FlexString,
    #[serde(rename = "L")]
    pub low: FlexString,
    #[serde(rename = "C")]
    pub close: FlexString,
    #[serde(rename = "MO")]
    pub morning_open: FlexString,
    #[serde(rename = "MH")]
    pub morning_high: FlexString,
    #[serde(rename = "ML")]
    pub morning_low: FlexString,
    #[serde(rename = "MC")]
    pub morning_close: FlexString,
    #[serde(rename = "EO")]
    pub evening_open: FlexString,
    #[serde(rename = "EH")]
    pub evening_high: FlexString,
    #[serde(rename = "EL")]
    pub evening_low: FlexString,
    #[serde(rename = "EC")]
    pub evening_close: FlexString,
    #[serde(rename = "AO")]
    pub afternoon_open: FlexString,
    #[serde(rename = "AH")]
    pub afternoon_high: FlexString,
    #[serde(rename = "AL")]
    pub afternoon_low: FlexString,
    #[serde(rename = "AC")]
    pub afternoon_close: FlexString,
    #[serde(rename = "Vo")]
    pub volume: FlexString,
    #[serde(rename = "OI")]
    pub open_interest: FlexString,
    #[serde(rename = "Va")]
    pub turnover: FlexString,
    #[serde(rename = "CM")]
    pub contract_month: String,
    #[serde(rename = "Strike")]
    pub strike: FlexString,
    #[serde(rename = "VoOA")]
    pub volume_on_auction: FlexString,
    #[serde(rename = "EmMrgnTrgDiv")]
    pub em_mrgn_trg_div: String,
    #[serde(rename = "PCDiv")]
    pub pc_div: String,
    #[serde(rename = "LTD")]
    pub last_trading_date: String,
    #[serde(rename = "SQD")]
    pub sq_date: String,
    #[serde(rename = "Settle")]
    pub settle: FlexString,
    #[serde(rename = "Theo")]
    pub theo: FlexString,
    #[serde(rename = "BaseVol")]
    pub base_vol: FlexString,
    #[serde(rename = "UnderPx")]
    pub under_px: FlexString,
    #[serde(rename = "IV")]
    pub iv: FlexString,
    #[serde(rename = "IR")]
    pub ir: FlexString,
    #[serde(rename = "CCMFlag")]
    pub ccm_flag: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FinsDetails {
    #[serde(rename = "DiscDate")]
    pub disc_date: String,
    #[serde(rename = "DiscTime")]
    pub disc_time: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "DiscNo")]
    pub disc_no: String,
    #[serde(rename = "DocType")]
    pub doc_type: String,
    #[serde(rename = "FS")]
    pub fs: Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FinsDividend {
    #[serde(rename = "PubDate")]
    pub pub_date: String,
    #[serde(rename = "PubTime")]
    pub pub_time: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "RefNo")]
    pub ref_no: String,
    #[serde(rename = "StatCode")]
    pub stat_code: String,
    #[serde(rename = "BoardDate")]
    pub board_date: String,
    #[serde(rename = "IFCode")]
    pub if_code: String,
    #[serde(rename = "FRCode")]
    pub fr_code: String,
    #[serde(rename = "IFTerm")]
    pub if_term: String,
    #[serde(rename = "DivRate")]
    pub div_rate: FlexString,
    #[serde(rename = "RecDate")]
    pub rec_date: String,
    #[serde(rename = "ExDate")]
    pub ex_date: String,
    #[serde(rename = "ActRecDate")]
    pub act_rec_date: String,
    #[serde(rename = "PayDate")]
    pub pay_date: String,
    #[serde(rename = "CARefNo")]
    pub ca_ref_no: String,
    #[serde(rename = "DistAmt")]
    pub dist_amt: FlexString,
    #[serde(rename = "RetEarn")]
    pub ret_earn: FlexString,
    #[serde(rename = "DeemDiv")]
    pub deem_div: FlexString,
    #[serde(rename = "DeemCapGains")]
    pub deem_cap_gains: FlexString,
    #[serde(rename = "NetAssetDecRatio")]
    pub net_asset_dec_ratio: FlexString,
    #[serde(rename = "CommSpecCode")]
    pub comm_spec_code: String,
    #[serde(rename = "CommDivRate")]
    pub comm_div_rate: FlexString,
    #[serde(rename = "SpecDivRate")]
    pub spec_div_rate: FlexString,
}

// FinsSummary: 80+ フィールドのため Clone 削除
#[derive(Debug, Deserialize, Serialize)]
pub struct FinsSummary {
    #[serde(rename = "DiscDate")]
    pub disc_date: String,
    #[serde(rename = "DiscTime")]
    pub disc_time: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "DiscNo")]
    pub disc_no: String,
    #[serde(rename = "DocType")]
    pub doc_type: String,
    #[serde(rename = "CurPerType")]
    pub cur_per_type: String,
    #[serde(rename = "CurPerSt")]
    pub cur_per_st: String,
    #[serde(rename = "CurPerEn")]
    pub cur_per_en: String,
    #[serde(rename = "CurFYSt")]
    pub cur_fy_st: String,
    #[serde(rename = "CurFYEn")]
    pub cur_fy_en: String,
    #[serde(rename = "NxtFYSt")]
    pub nxt_fy_st: String,
    #[serde(rename = "NxtFYEn")]
    pub nxt_fy_en: String,
    #[serde(rename = "Sales")]
    pub sales: FlexString,
    #[serde(rename = "OP")]
    pub op: FlexString,
    #[serde(rename = "OdP")]
    pub od_p: FlexString,
    #[serde(rename = "NP")]
    pub np: FlexString,
    #[serde(rename = "EPS")]
    pub eps: FlexString,
    #[serde(rename = "DEPS")]
    pub deps: FlexString,
    #[serde(rename = "TA")]
    pub ta: FlexString,
    #[serde(rename = "Eq")]
    pub eq: FlexString,
    #[serde(rename = "EqAR")]
    pub eq_ar: FlexString,
    #[serde(rename = "BPS")]
    pub bps: FlexString,
    #[serde(rename = "CFO")]
    pub cfo: FlexString,
    #[serde(rename = "CFI")]
    pub cfi: FlexString,
    #[serde(rename = "CFF")]
    pub cff: FlexString,
    #[serde(rename = "CashEq")]
    pub cash_eq: FlexString,
    #[serde(rename = "Div1Q")]
    pub div_1q: FlexString,
    #[serde(rename = "Div2Q")]
    pub div_2q: FlexString,
    #[serde(rename = "Div3Q")]
    pub div_3q: FlexString,
    #[serde(rename = "DivFY")]
    pub div_fy: FlexString,
    #[serde(rename = "DivAnn")]
    pub div_ann: FlexString,
    #[serde(rename = "DivUnit")]
    pub div_unit: FlexString,
    #[serde(rename = "DivTotalAnn")]
    pub div_total_ann: FlexString,
    #[serde(rename = "PayoutRatioAnn")]
    pub payout_ratio_ann: FlexString,
    #[serde(rename = "FDiv1Q")]
    pub f_div_1q: FlexString,
    #[serde(rename = "FDiv2Q")]
    pub f_div_2q: FlexString,
    #[serde(rename = "FDiv3Q")]
    pub f_div_3q: FlexString,
    #[serde(rename = "FDivFY")]
    pub f_div_fy: FlexString,
    #[serde(rename = "FDivAnn")]
    pub f_div_ann: FlexString,
    #[serde(rename = "FDivUnit")]
    pub f_div_unit: FlexString,
    #[serde(rename = "FDivTotalAnn")]
    pub f_div_total_ann: FlexString,
    #[serde(rename = "FPayoutRatioAnn")]
    pub f_payout_ratio_ann: FlexString,
    #[serde(rename = "NxFDiv1Q")]
    pub nx_f_div_1q: FlexString,
    #[serde(rename = "NxFDiv2Q")]
    pub nx_f_div_2q: FlexString,
    #[serde(rename = "NxFDiv3Q")]
    pub nx_f_div_3q: FlexString,
    #[serde(rename = "NxFDivFY")]
    pub nx_f_div_fy: FlexString,
    #[serde(rename = "NxFDivAnn")]
    pub nx_f_div_ann: FlexString,
    #[serde(rename = "NxFDivUnit")]
    pub nx_f_div_unit: FlexString,
    #[serde(rename = "NxFPayoutRatioAnn")]
    pub nx_f_payout_ratio_ann: FlexString,
    #[serde(rename = "FSales2Q")]
    pub f_sales_2q: FlexString,
    #[serde(rename = "FOP2Q")]
    pub f_op_2q: FlexString,
    #[serde(rename = "FOdP2Q")]
    pub f_od_p_2q: FlexString,
    #[serde(rename = "FNP2Q")]
    pub f_np_2q: FlexString,
    #[serde(rename = "FEPS2Q")]
    pub f_eps_2q: FlexString,
    #[serde(rename = "NxFSales2Q")]
    pub nx_f_sales_2q: FlexString,
    #[serde(rename = "NxFOP2Q")]
    pub nx_f_op_2q: FlexString,
    #[serde(rename = "NxFOdP2Q")]
    pub nx_f_od_p_2q: FlexString,
    #[serde(rename = "NxFNp2Q")]
    pub nx_f_np_2q: FlexString,
    #[serde(rename = "NxFEPS2Q")]
    pub nx_f_eps_2q: FlexString,
    #[serde(rename = "FSales")]
    pub f_sales: FlexString,
    #[serde(rename = "FOP")]
    pub f_op: FlexString,
    #[serde(rename = "FOdP")]
    pub f_od_p: FlexString,
    #[serde(rename = "FNP")]
    pub f_np: FlexString,
    #[serde(rename = "FEPS")]
    pub f_eps: FlexString,
    #[serde(rename = "NxFSales")]
    pub nx_f_sales: FlexString,
    #[serde(rename = "NxFOP")]
    pub nx_f_op: FlexString,
    #[serde(rename = "NxFOdP")]
    pub nx_f_od_p: FlexString,
    #[serde(rename = "NxFNp")]
    pub nx_f_np: FlexString,
    #[serde(rename = "NxFEPS")]
    pub nx_f_eps: FlexString,
    #[serde(rename = "MatChgSub")]
    pub mat_chg_sub: String,
    #[serde(rename = "SigChgInC")]
    pub sig_chg_in_c: String,
    #[serde(rename = "ChgByASRev")]
    pub chg_by_as_rev: String,
    #[serde(rename = "ChgNoASRev")]
    pub chg_no_as_rev: String,
    #[serde(rename = "ChgAcEst")]
    pub chg_ac_est: String,
    #[serde(rename = "RetroRst")]
    pub retro_rst: String,
    #[serde(rename = "ShOutFY")]
    pub sh_out_fy: FlexString,
    #[serde(rename = "TrShFY")]
    pub tr_sh_fy: FlexString,
    #[serde(rename = "AvgSh")]
    pub avg_sh: FlexString,
    #[serde(rename = "NCSales")]
    pub nc_sales: FlexString,
    #[serde(rename = "NCOP")]
    pub nc_op: FlexString,
    #[serde(rename = "NCOdP")]
    pub nc_od_p: FlexString,
    #[serde(rename = "NCNP")]
    pub nc_np: FlexString,
    #[serde(rename = "NCEPS")]
    pub nc_eps: FlexString,
    #[serde(rename = "NCTA")]
    pub nc_ta: FlexString,
    #[serde(rename = "NCEq")]
    pub nc_eq: FlexString,
    #[serde(rename = "NCEqAR")]
    pub nc_eq_ar: FlexString,
    #[serde(rename = "NCBPS")]
    pub nc_bps: FlexString,
    #[serde(rename = "FNCSales2Q")]
    pub f_nc_sales_2q: FlexString,
    #[serde(rename = "FNCOP2Q")]
    pub f_nc_op_2q: FlexString,
    #[serde(rename = "FNCOdP2Q")]
    pub f_nc_od_p_2q: FlexString,
    #[serde(rename = "FNCNP2Q")]
    pub f_nc_np_2q: FlexString,
    #[serde(rename = "FNCEPS2Q")]
    pub f_nc_eps_2q: FlexString,
    #[serde(rename = "NxFNCSales2Q")]
    pub nx_f_nc_sales_2q: FlexString,
    #[serde(rename = "NxFNCOP2Q")]
    pub nx_f_nc_op_2q: FlexString,
    #[serde(rename = "NxFNCOdP2Q")]
    pub nx_f_nc_od_p_2q: FlexString,
    #[serde(rename = "NxFNCNP2Q")]
    pub nx_f_nc_np_2q: FlexString,
    #[serde(rename = "NxFNCEPS2Q")]
    pub nx_f_nc_eps_2q: FlexString,
    #[serde(rename = "FNCSales")]
    pub f_nc_sales: FlexString,
    #[serde(rename = "FNCOP")]
    pub f_nc_op: FlexString,
    #[serde(rename = "FNCOdP")]
    pub f_nc_od_p: FlexString,
    #[serde(rename = "FNCNP")]
    pub f_nc_np: FlexString,
    #[serde(rename = "FNCEPS")]
    pub f_nc_eps: FlexString,
    #[serde(rename = "NxFNCSales")]
    pub nx_f_nc_sales: FlexString,
    #[serde(rename = "NxFNCOP")]
    pub nx_f_nc_op: FlexString,
    #[serde(rename = "NxFNCOdP")]
    pub nx_f_nc_od_p: FlexString,
    #[serde(rename = "NxFNCNP")]
    pub nx_f_nc_np: FlexString,
    #[serde(rename = "NxFNCEPS")]
    pub nx_f_nc_eps: FlexString,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TopixDailyBar {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "O")]
    pub open: FlexString,
    #[serde(rename = "H")]
    pub high: FlexString,
    #[serde(rename = "L")]
    pub low: FlexString,
    #[serde(rename = "C")]
    pub close: FlexString,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IndexDailyBar {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "O")]
    pub open: FlexString,
    #[serde(rename = "H")]
    pub high: FlexString,
    #[serde(rename = "L")]
    pub low: FlexString,
    #[serde(rename = "C")]
    pub close: FlexString,
}

// InvestorType: 非常に多くのフィールドのため Clone 削除
#[derive(Debug, Deserialize, Serialize)]
pub struct InvestorType {
    #[serde(rename = "PubDate")]
    pub pub_date: String,
    #[serde(rename = "StDate")]
    pub st_date: String,
    #[serde(rename = "EnDate")]
    pub en_date: String,
    #[serde(rename = "Section")]
    pub section: String,
    #[serde(rename = "PropSell")]
    pub prop_sell: FlexString,
    #[serde(rename = "PropBuy")]
    pub prop_buy: FlexString,
    #[serde(rename = "PropTot")]
    pub prop_tot: FlexString,
    #[serde(rename = "PropBal")]
    pub prop_bal: FlexString,
    #[serde(rename = "BrkSell")]
    pub brk_sell: FlexString,
    #[serde(rename = "BrkBuy")]
    pub brk_buy: FlexString,
    #[serde(rename = "BrkTot")]
    pub brk_tot: FlexString,
    #[serde(rename = "BrkBal")]
    pub brk_bal: FlexString,
    #[serde(rename = "TotSell")]
    pub tot_sell: FlexString,
    #[serde(rename = "TotBuy")]
    pub tot_buy: FlexString,
    #[serde(rename = "TotTot")]
    pub tot_tot: FlexString,
    #[serde(rename = "TotBal")]
    pub tot_bal: FlexString,
    #[serde(rename = "IndSell")]
    pub ind_sell: FlexString,
    #[serde(rename = "IndBuy")]
    pub ind_buy: FlexString,
    #[serde(rename = "IndTot")]
    pub ind_tot: FlexString,
    #[serde(rename = "IndBal")]
    pub ind_bal: FlexString,
    #[serde(rename = "FrgnSell")]
    pub frgn_sell: FlexString,
    #[serde(rename = "FrgnBuy")]
    pub frgn_buy: FlexString,
    #[serde(rename = "FrgnTot")]
    pub frgn_tot: FlexString,
    #[serde(rename = "FrgnBal")]
    pub frgn_bal: FlexString,
    #[serde(rename = "SecCoSell")]
    pub sec_co_sell: FlexString,
    #[serde(rename = "SecCoBuy")]
    pub sec_co_buy: FlexString,
    #[serde(rename = "SecCoTot")]
    pub sec_co_tot: FlexString,
    #[serde(rename = "SecCoBal")]
    pub sec_co_bal: FlexString,
    #[serde(rename = "InvTrSell")]
    pub inv_tr_sell: FlexString,
    #[serde(rename = "InvTrBuy")]
    pub inv_tr_buy: FlexString,
    #[serde(rename = "InvTrTot")]
    pub inv_tr_tot: FlexString,
    #[serde(rename = "InvTrBal")]
    pub inv_tr_bal: FlexString,
    #[serde(rename = "BusCoSell")]
    pub bus_co_sell: FlexString,
    #[serde(rename = "BusCoBuy")]
    pub bus_co_buy: FlexString,
    #[serde(rename = "BusCoTot")]
    pub bus_co_tot: FlexString,
    #[serde(rename = "BusCoBal")]
    pub bus_co_bal: FlexString,
    #[serde(rename = "OthCoSell")]
    pub oth_co_sell: FlexString,
    #[serde(rename = "OthCoBuy")]
    pub oth_co_buy: FlexString,
    #[serde(rename = "OthCoTot")]
    pub oth_co_tot: FlexString,
    #[serde(rename = "OthCoBal")]
    pub oth_co_bal: FlexString,
    #[serde(rename = "InsCoSell")]
    pub ins_co_sell: FlexString,
    #[serde(rename = "InsCoBuy")]
    pub ins_co_buy: FlexString,
    #[serde(rename = "InsCoTot")]
    pub ins_co_tot: FlexString,
    #[serde(rename = "InsCoBal")]
    pub ins_co_bal: FlexString,
    #[serde(rename = "BankSell")]
    pub bank_sell: FlexString,
    #[serde(rename = "BankBuy")]
    pub bank_buy: FlexString,
    #[serde(rename = "BankTot")]
    pub bank_tot: FlexString,
    #[serde(rename = "BankBal")]
    pub bank_bal: FlexString,
    #[serde(rename = "TrstBnkSell")]
    pub trst_bnk_sell: FlexString,
    #[serde(rename = "TrstBnkBuy")]
    pub trst_bnk_buy: FlexString,
    #[serde(rename = "TrstBnkTot")]
    pub trst_bnk_tot: FlexString,
    #[serde(rename = "TrstBnkBal")]
    pub trst_bnk_bal: FlexString,
    #[serde(rename = "OthFinSell")]
    pub oth_fin_sell: FlexString,
    #[serde(rename = "OthFinBuy")]
    pub oth_fin_buy: FlexString,
    #[serde(rename = "OthFinTot")]
    pub oth_fin_tot: FlexString,
    #[serde(rename = "OthFinBal")]
    pub oth_fin_bal: FlexString,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PubReason {
    #[serde(rename = "Restricted")]
    pub restricted: String,
    #[serde(rename = "DailyPublication")]
    pub daily_publication: String,
    #[serde(rename = "Monitoring")]
    pub monitoring: String,
    #[serde(rename = "RestrictedByJSF")]
    pub restricted_by_jsf: String,
    #[serde(rename = "PrecautionByJSF")]
    pub precaution_by_jsf: String,
    #[serde(rename = "UnclearOrSecOnAlert")]
    pub unclear_or_sec_on_alert: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarginAlert {
    #[serde(rename = "PubDate")]
    pub pub_date: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "AppDate")]
    pub app_date: String,
    #[serde(rename = "PubReason")]
    pub pub_reason: PubReason,
    #[serde(rename = "ShrtOut")]
    pub shrt_out: FlexString,
    #[serde(rename = "ShrtOutChg")]
    pub shrt_out_chg: FlexString,
    #[serde(rename = "ShrtOutRatio")]
    pub shrt_out_ratio: FlexString,
    #[serde(rename = "LongOut")]
    pub long_out: FlexString,
    #[serde(rename = "LongOutChg")]
    pub long_out_chg: FlexString,
    #[serde(rename = "LongOutRatio")]
    pub long_out_ratio: FlexString,
    #[serde(rename = "SLRatio")]
    pub sl_ratio: FlexString,
    #[serde(rename = "ShrtNegOut")]
    pub shrt_neg_out: FlexString,
    #[serde(rename = "ShrtNegOutChg")]
    pub shrt_neg_out_chg: FlexString,
    #[serde(rename = "ShrtStdOut")]
    pub shrt_std_out: FlexString,
    #[serde(rename = "ShrtStdOutChg")]
    pub shrt_std_out_chg: FlexString,
    #[serde(rename = "LongNegOut")]
    pub long_neg_out: FlexString,
    #[serde(rename = "LongNegOutChg")]
    pub long_neg_out_chg: FlexString,
    #[serde(rename = "LongStdOut")]
    pub long_std_out: FlexString,
    #[serde(rename = "LongStdOutChg")]
    pub long_std_out_chg: FlexString,
    #[serde(rename = "TSEMrgnRegCls")]
    pub tse_mrgn_reg_cls: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarginInterest {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "ShrtVol")]
    pub shrt_vol: FlexString,
    #[serde(rename = "LongVol")]
    pub long_vol: FlexString,
    #[serde(rename = "ShrtNegVol")]
    pub shrt_neg_vol: FlexString,
    #[serde(rename = "LongNegVol")]
    pub long_neg_vol: FlexString,
    #[serde(rename = "ShrtStdVol")]
    pub shrt_std_vol: FlexString,
    #[serde(rename = "LongStdVol")]
    pub long_std_vol: FlexString,
    #[serde(rename = "IssType")]
    pub iss_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShortRatio {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "S33")]
    pub s33: String,
    #[serde(rename = "SellExShortVa")]
    pub sell_ex_short_va: FlexString,
    #[serde(rename = "ShrtWithResVa")]
    pub shrt_with_res_va: FlexString,
    #[serde(rename = "ShrtNoResVa")]
    pub shrt_no_res_va: FlexString,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShortSaleReport {
    #[serde(rename = "DiscDate")]
    pub disc_date: String,
    #[serde(rename = "CalcDate")]
    pub calc_date: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "SSName")]
    pub ss_name: String,
    #[serde(rename = "SSAddr")]
    pub ss_addr: String,
    #[serde(rename = "DICName")]
    pub dic_name: String,
    #[serde(rename = "DICAddr")]
    pub dic_addr: String,
    #[serde(rename = "FundName")]
    pub fund_name: String,
    #[serde(rename = "ShrtPosToSO")]
    pub shrt_pos_to_so: FlexString,
    #[serde(rename = "ShrtPosShares")]
    pub shrt_pos_shares: FlexString,
    #[serde(rename = "ShrtPosUnits")]
    pub shrt_pos_units: FlexString,
    #[serde(rename = "PrevRptDate")]
    pub prev_rpt_date: String,
    #[serde(rename = "PrevRptRatio")]
    pub prev_rpt_ratio: FlexString,
    #[serde(rename = "Notes")]
    pub notes: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_stock_master_response() {
        let json = r#"{
            "data": [
                {
                    "Date": "2026-03-14",
                    "Code": "86970",
                    "CoName": "日本取引所グループ",
                    "CoNameEn": "Japan Exchange Group",
                    "S17": "16",
                    "S17Nm": "金融（除く銀行）",
                    "S33": "7200",
                    "S33Nm": "その他金融業",
                    "ScaleCat": "TOPIX Large70",
                    "Mkt": "0111",
                    "MktNm": "プライム",
                    "Mrgn": "1",
                    "MrgnNm": "信用"
                }
            ],
            "pagination_key": null
        }"#;

        let response: ApiResponse<StockMaster> = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].code, "86970");
        assert_eq!(response.data[0].co_name, "日本取引所グループ");
        assert!(response.pagination_key.is_none());
    }

    #[test]
    fn test_deserialize_with_pagination_key() {
        let json = r#"{
            "data": [],
            "pagination_key": "next_page_token_abc123"
        }"#;

        let response: ApiResponse<StockMaster> = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 0);
        assert_eq!(
            response.pagination_key.as_deref(),
            Some("next_page_token_abc123")
        );
    }

    #[test]
    fn test_deserialize_api_error_response() {
        let json = r#"{"message": "Unauthorized"}"#;
        let error: ApiErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(error.message, "Unauthorized");
    }

    #[test]
    fn test_deserialize_am_bar_response() {
        let json = r#"{
            "data": [
                {
                    "Date": "2026-03-14",
                    "Code": "27800",
                    "MO": 1000.0,
                    "MH": 1050.0,
                    "ML": 990.0,
                    "MC": 1030.0,
                    "MVo": 500000,
                    "MVa": 515000000
                }
            ],
            "pagination_key": null
        }"#;

        let response: ApiResponse<AmBar> = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].code, "27800");
        assert_eq!(response.data[0].morning_open, Some(1000.0));
        assert_eq!(response.data[0].morning_close, Some(1030.0));
        assert!(response.pagination_key.is_none());
    }

    #[test]
    fn test_deserialize_am_bar_with_pagination_key() {
        let json = r#"{
            "data": [],
            "pagination_key": "am_next_page_token_xyz"
        }"#;

        let response: ApiResponse<AmBar> = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 0);
        assert_eq!(
            response.pagination_key.as_deref(),
            Some("am_next_page_token_xyz")
        );
    }

    #[test]
    fn test_deserialize_breakdown_response() {
        let json = r#"{
            "data": [
                {
                    "Date": "2021-09-01",
                    "Code": "27800",
                    "LongSellVa": 1000000,
                    "ShrtNoMrgnVa": 200000,
                    "MrgnSellNewVa": 300000,
                    "MrgnSellCloseVa": 400000,
                    "LongBuyVa": 500000,
                    "MrgnBuyNewVa": 600000,
                    "MrgnBuyCloseVa": 700000,
                    "LongSellVo": 1000,
                    "ShrtNoMrgnVo": 200,
                    "MrgnSellNewVo": 300,
                    "MrgnSellCloseVo": 400,
                    "LongBuyVo": 500,
                    "MrgnBuyNewVo": 600,
                    "MrgnBuyCloseVo": 700
                }
            ],
            "pagination_key": null
        }"#;

        let response: ApiResponse<Breakdown> = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].code, "27800");
        assert_eq!(response.data[0].long_sell_va, 1000000.0);
        assert_eq!(response.data[0].long_buy_vo, 500.0);
        assert!(response.pagination_key.is_none());
    }

    #[test]
    fn test_deserialize_bulk_get_response() {
        let json = r#"{"url": "https://example.com/bulk/file.gz"}"#;
        let response: BulkGetResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.url, "https://example.com/bulk/file.gz");
    }

    #[test]
    fn test_deserialize_bulk_get_response_presigned_url() {
        let json = r#"{"url": "https://s3.amazonaws.com/bucket/path/to/file.gz?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAIOSFODNN7EXAMPLE%2F20260317%2Fap-northeast-1%2Fs3%2Faws4_request&X-Amz-Date=20260317T000000Z&X-Amz-Expires=3600&X-Amz-Signature=abc123def456"}"#;
        let response: BulkGetResponse = serde_json::from_str(json).unwrap();
        assert!(response.url.starts_with("https://s3.amazonaws.com/"));
        assert!(response.url.contains("X-Amz-Signature="));
    }

    #[test]
    fn test_deserialize_bulk_list_response() {
        let json = r#"{
            "data": [
                {
                    "Key": "indices/bars/daily/2026/03/bars_daily_20260301.csv.gz",
                    "LastModified": "2026-03-02T00:00:00Z",
                    "Size": 12345
                }
            ]
        }"#;
        let response: ApiResponse<BulkListItem> = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(
            response.data[0].key,
            "indices/bars/daily/2026/03/bars_daily_20260301.csv.gz"
        );
        assert_eq!(response.data[0].last_modified, "2026-03-02T00:00:00Z");
        assert_eq!(response.data[0].size, 12345.0);
        assert!(response.pagination_key.is_none());
    }

    #[test]
    fn test_deserialize_minute_bar_response() {
        let json = r#"{
            "data": [
                {
                    "Date": "2026-03-14",
                    "Time": "09:00:00",
                    "Code": "27800",
                    "O": 1000.0,
                    "H": 1050.0,
                    "L": 990.0,
                    "C": 1030.0,
                    "Vo": 5000.0,
                    "Va": 5150000.0
                }
            ],
            "pagination_key": null
        }"#;

        let response: ApiResponse<MinuteBar> = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].code, "27800");
        assert_eq!(response.data[0].open, 1000.0);
        assert!(response.pagination_key.is_none());
    }

    #[test]
    fn test_flex_string_from_number() {
        let json = r#"{"data": [{"Date": "2021-09-01", "S33": "0050", "SellExShortVa": 1234567, "ShrtWithResVa": 0, "ShrtNoResVa": null}], "pagination_key": null}"#;
        let response: ApiResponse<ShortRatio> = serde_json::from_str(json).unwrap();
        assert_eq!(response.data[0].sell_ex_short_va.0, "1234567");
        assert_eq!(response.data[0].shrt_no_res_va.0, "");
    }
}

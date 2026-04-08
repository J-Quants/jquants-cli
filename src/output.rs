use crate::cli::OutputFormat;
use crate::error::AppError;
use crate::models::{
    AmBar, Breakdown, BulkListItem, Calendar, DailyBar, EarningsCalendar, FinsDetails,
    FinsDividend, FinsSummary, FuturesBar, IndexDailyBar, InvestorType, MarginAlert,
    MarginInterest, MinuteBar, Options225Bar, OptionsBar, ShortRatio, ShortSaleReport, StockMaster,
    TopixDailyBar,
};
use arrow_json::reader::infer_json_schema_from_seekable;
use arrow_schema::Schema;
use comfy_table::{presets::UTF8_FULL, Table};
use parquet::arrow::ArrowWriter;
use serde::Serialize;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Cursor, IsTerminal, Write};
use std::sync::Arc;

// ── TableDisplay トレイト ─────────────────────────────────────────────────────

pub trait TableDisplay {
    fn table_headers() -> Vec<&'static str>;
    fn table_row(&self) -> Vec<String>;
}

// ── TableDisplay impls ────────────────────────────────────────────────────────

impl TableDisplay for StockMaster {
    fn table_headers() -> Vec<&'static str> {
        vec!["Date", "Code", "CoName", "Market", "Sector17", "Scale"]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.code.clone(),
            self.co_name.clone(),
            self.market_code_name.clone(),
            self.sector17_code_name.clone(),
            self.scale_category.clone(),
        ]
    }
}

impl TableDisplay for AmBar {
    fn table_headers() -> Vec<&'static str> {
        vec![
            "Date", "Code", "Open", "High", "Low", "Close", "Volume", "Turnover",
        ]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.code.clone(),
            opt_display(self.morning_open),
            opt_display(self.morning_high),
            opt_display(self.morning_low),
            opt_display(self.morning_close),
            opt_display(self.morning_volume),
            opt_display(self.morning_turnover),
        ]
    }
}

impl TableDisplay for Breakdown {
    fn table_headers() -> Vec<&'static str> {
        vec![
            "Date",
            "Code",
            "LongSellVa",
            "LongBuyVa",
            "MrgnSellNewVa",
            "MrgnBuyNewVa",
            "LongSellVo",
            "LongBuyVo",
        ]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.code.clone(),
            self.long_sell_va.to_string(),
            self.long_buy_va.to_string(),
            self.mrgn_sell_new_va.to_string(),
            self.mrgn_buy_new_va.to_string(),
            self.long_sell_vo.to_string(),
            self.long_buy_vo.to_string(),
        ]
    }
}

impl TableDisplay for MinuteBar {
    fn table_headers() -> Vec<&'static str> {
        vec![
            "Date", "Time", "Code", "Open", "High", "Low", "Close", "Volume",
        ]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.time.clone(),
            self.code.clone(),
            self.open.to_string(),
            self.high.to_string(),
            self.low.to_string(),
            self.close.to_string(),
            self.volume.to_string(),
        ]
    }
}

impl TableDisplay for DailyBar {
    fn table_headers() -> Vec<&'static str> {
        vec![
            "Date", "Code", "AdjO", "AdjH", "AdjL", "AdjC", "AdjVo", "Va",
        ]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.code.clone(),
            opt_display(self.adj_open),
            opt_display(self.adj_high),
            opt_display(self.adj_low),
            opt_display(self.adj_close),
            opt_display(self.adj_volume),
            opt_display(self.turnover),
        ]
    }
}

impl TableDisplay for BulkListItem {
    fn table_headers() -> Vec<&'static str> {
        vec!["Key", "LastModified", "Size"]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.key.clone(),
            self.last_modified.clone(),
            self.size.to_string(),
        ]
    }
}

impl TableDisplay for Calendar {
    fn table_headers() -> Vec<&'static str> {
        vec!["Date", "HolDiv"]
    }
    fn table_row(&self) -> Vec<String> {
        vec![self.date.clone(), self.hol_div.clone()]
    }
}

impl TableDisplay for EarningsCalendar {
    fn table_headers() -> Vec<&'static str> {
        vec!["Date", "Code", "CoName", "FY", "SectorNm", "FQ", "Section"]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.code.clone(),
            self.co_name.clone(),
            self.fy.clone(),
            self.sector_nm.clone(),
            self.fq.clone(),
            self.section.clone(),
        ]
    }
}

impl TableDisplay for Options225Bar {
    fn table_headers() -> Vec<&'static str> {
        vec!["Date", "Code", "Strike", "PCDiv", "O", "H", "L", "C"]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.code.clone(),
            self.strike.to_string(),
            self.pc_div.clone(),
            self.open.to_string(),
            self.high.to_string(),
            self.low.to_string(),
            self.close.to_string(),
        ]
    }
}

impl TableDisplay for FuturesBar {
    fn table_headers() -> Vec<&'static str> {
        vec!["Date", "Code", "ProdCat", "O", "H", "L", "C", "Settle"]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.code.clone(),
            self.prod_cat.clone(),
            self.open.to_string(),
            self.high.to_string(),
            self.low.to_string(),
            self.close.to_string(),
            self.settle.to_string(),
        ]
    }
}

impl TableDisplay for OptionsBar {
    fn table_headers() -> Vec<&'static str> {
        vec!["Date", "Code", "ProdCat", "Strike", "PCDiv", "O", "H", "C"]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.code.clone(),
            self.prod_cat.clone(),
            self.strike.to_string(),
            self.pc_div.clone(),
            self.open.to_string(),
            self.high.to_string(),
            self.close.to_string(),
        ]
    }
}

impl TableDisplay for FinsDividend {
    fn table_headers() -> Vec<&'static str> {
        vec![
            "PubDate", "Code", "DivRate", "IFCode", "FRCode", "RecDate", "ExDate", "PayDate",
        ]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.pub_date.clone(),
            self.code.clone(),
            self.div_rate.to_string(),
            self.if_code.clone(),
            self.fr_code.clone(),
            self.rec_date.clone(),
            self.ex_date.clone(),
            self.pay_date.clone(),
        ]
    }
}

impl TableDisplay for FinsSummary {
    fn table_headers() -> Vec<&'static str> {
        vec![
            "DiscDate", "Code", "DocType", "PerType", "Sales", "OP", "NP", "EPS",
        ]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.disc_date.clone(),
            self.code.clone(),
            self.doc_type.clone(),
            self.cur_per_type.clone(),
            self.sales.to_string(),
            self.op.to_string(),
            self.np.to_string(),
            self.eps.to_string(),
        ]
    }
}

impl TableDisplay for TopixDailyBar {
    fn table_headers() -> Vec<&'static str> {
        vec!["Date", "Open", "High", "Low", "Close"]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.open.to_string(),
            self.high.to_string(),
            self.low.to_string(),
            self.close.to_string(),
        ]
    }
}

impl TableDisplay for IndexDailyBar {
    fn table_headers() -> Vec<&'static str> {
        vec!["Date", "Code", "Open", "High", "Low", "Close"]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.code.clone(),
            self.open.to_string(),
            self.high.to_string(),
            self.low.to_string(),
            self.close.to_string(),
        ]
    }
}

impl TableDisplay for InvestorType {
    fn table_headers() -> Vec<&'static str> {
        vec![
            "PubDate", "StDate", "EnDate", "Section", "TotSell", "TotBuy", "TotBal",
        ]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.pub_date.clone(),
            self.st_date.clone(),
            self.en_date.clone(),
            self.section.clone(),
            self.tot_sell.to_string(),
            self.tot_buy.to_string(),
            self.tot_bal.to_string(),
        ]
    }
}

impl TableDisplay for MarginInterest {
    fn table_headers() -> Vec<&'static str> {
        vec![
            "Date",
            "Code",
            "ShrtVol",
            "LongVol",
            "ShrtNegVol",
            "LongNegVol",
            "IssType",
        ]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.code.clone(),
            self.shrt_vol.to_string(),
            self.long_vol.to_string(),
            self.shrt_neg_vol.to_string(),
            self.long_neg_vol.to_string(),
            self.iss_type.clone(),
        ]
    }
}

impl TableDisplay for ShortRatio {
    fn table_headers() -> Vec<&'static str> {
        vec![
            "Date",
            "S33",
            "SellExShortVa",
            "ShrtWithResVa",
            "ShrtNoResVa",
        ]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.s33.clone(),
            self.sell_ex_short_va.to_string(),
            self.shrt_with_res_va.to_string(),
            self.shrt_no_res_va.to_string(),
        ]
    }
}

impl TableDisplay for ShortSaleReport {
    fn table_headers() -> Vec<&'static str> {
        vec![
            "DiscDate",
            "CalcDate",
            "Code",
            "SSName",
            "ShrtPosToSO",
            "ShrtPosShares",
            "PrevRptRatio",
        ]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.disc_date.clone(),
            self.calc_date.clone(),
            self.code.clone(),
            self.ss_name.clone(),
            self.shrt_pos_to_so.to_string(),
            self.shrt_pos_shares.to_string(),
            self.prev_rpt_ratio.to_string(),
        ]
    }
}

impl TableDisplay for MarginAlert {
    fn table_headers() -> Vec<&'static str> {
        vec![
            "PubDate",
            "Code",
            "AppDate",
            "ShrtOut",
            "LongOut",
            "SLRatio",
            "TSEMrgnRegCls",
        ]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.pub_date.clone(),
            self.code.clone(),
            self.app_date.clone(),
            self.shrt_out.to_string(),
            self.long_out.to_string(),
            self.sl_ratio.to_string(),
            self.tse_mrgn_reg_cls.clone(),
        ]
    }
}

impl TableDisplay for FinsDetails {
    fn table_headers() -> Vec<&'static str> {
        vec![
            "DiscDate", "DiscTime", "Code", "DiscNo", "DocType", "FS(keys)",
        ]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.disc_date.clone(),
            self.disc_time.clone(),
            self.code.clone(),
            self.disc_no.clone(),
            self.doc_type.clone(),
            fins_fs_keys_summary(&self.fs),
        ]
    }
}

// ── writer ヘルパー ───────────────────────────────────────────────────────────

pub fn make_writer(save: &Option<String>) -> Result<Box<dyn Write>, AppError> {
    match save {
        Some(path) => Ok(Box::new(BufWriter::new(File::create(path)?))),
        None => Ok(Box::new(BufWriter::new(io::stdout().lock()))),
    }
}

fn saved_notice(save: &Option<String>) {
    if let Some(path) = save {
        eprintln!("Saved: {}", path);
    }
}

fn write_json_pretty<T: Serialize>(results: &[T], save: &Option<String>) -> Result<(), AppError> {
    let mut writer = make_writer(save)?;
    serde_json::to_writer_pretty(&mut writer, results)?;
    writeln!(writer)?;
    Ok(())
}

fn write_parquet<T: Serialize>(results: &[T], path: &str) -> Result<(), AppError> {
    if results.is_empty() {
        eprintln!("No data to write.");
        return Ok(());
    }

    // 1. NDJSON に変換（1行1レコード）
    let mut buf: Vec<u8> = Vec::with_capacity(results.len() * 256);
    for item in results {
        serde_json::to_writer(&mut buf, item)?;
        buf.push(b'\n');
    }

    write_parquet_from_ndjson(&buf, path)
}

/// NDJSON バイト列から Parquet ファイルを書き出す共通処理
fn write_parquet_from_ndjson(buf: &[u8], path: &str) -> Result<(), AppError> {
    // スキーマ推論（戻り値は (Schema, 読み込み行数) のタプル）
    let (inferred_schema, _) =
        infer_json_schema_from_seekable(&mut BufReader::new(Cursor::new(buf)), None)?;

    // 最初の NDJSON 行からフィールド順序を取得し、スキーマを並べ替え
    // （infer_json_schema_from_seekable は内部でアルファベット順になるため）
    let schema = if let Some(first_line) = buf.split(|&b| b == b'\n').next() {
        if let Ok(serde_json::Value::Object(map)) =
            serde_json::from_slice::<serde_json::Value>(first_line)
        {
            let ordered_fields: Vec<_> = map
                .keys()
                .filter_map(|name| inferred_schema.field_with_name(name).ok().cloned())
                .collect();
            if ordered_fields.len() == inferred_schema.fields().len() {
                Arc::new(Schema::new(ordered_fields))
            } else {
                Arc::new(inferred_schema)
            }
        } else {
            Arc::new(inferred_schema)
        }
    } else {
        Arc::new(inferred_schema)
    };

    // JSON → RecordBatch（複数バッチ対応）
    let reader =
        arrow_json::ReaderBuilder::new(schema.clone()).build(BufReader::new(Cursor::new(buf)))?;

    // Parquet ファイルに書き出し
    let file = File::create(path)?;
    let mut writer = ArrowWriter::try_new(file, schema, None)?;
    for batch in reader {
        writer.write(&batch?)?;
    }
    writer.close()?;
    eprintln!("Saved: {}", path);
    Ok(())
}

fn fins_fs_keys_summary(fs: &serde_json::Value) -> String {
    match fs {
        serde_json::Value::Object(map) => format!("{} items", map.len()),
        _ => "-".to_string(),
    }
}

fn opt_display<T: ToString>(o: Option<T>) -> String {
    o.map(|v| v.to_string()).unwrap_or_default()
}

// ── フィールド選択 ────────────────────────────────────────────────────────────

/// `--fields` でパース済みのフィールド選択（None は全フィールド出力）
pub type FieldSelection = Option<Vec<String>>;

/// serde_json::Value を CSV/テーブルセル用の文字列に変換
fn value_to_cell(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => String::new(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
            serde_json::to_string(v).unwrap_or_default()
        }
    }
}

/// Object から指定フィールドだけを抽出した新しい Value を返す
fn filter_object(
    map: &serde_json::Map<String, serde_json::Value>,
    fields: &[String],
) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    for field in fields {
        if let Some(val) = map.get(field) {
            obj.insert(field.clone(), val.clone());
        }
    }
    serde_json::Value::Object(obj)
}

/// Object から指定フィールドの値を文字列行として返す
fn row_from_fields(
    map: &serde_json::Map<String, serde_json::Value>,
    fields: &[String],
) -> Vec<String> {
    fields
        .iter()
        .map(|f| map.get(f).map(value_to_cell).unwrap_or_default())
        .collect()
}

/// CSV ライターにヘッダーと全行を書き出す
fn write_csv_filtered<W: io::Write>(
    wtr: &mut csv::Writer<W>,
    fields: &[String],
    values: &[serde_json::Value],
) -> Result<(), AppError> {
    wtr.write_record(fields)?;
    for v in values {
        if let serde_json::Value::Object(map) = v {
            wtr.write_record(row_from_fields(map, fields))?;
        }
    }
    wtr.flush()?;
    Ok(())
}

/// `--fields` 指定時の出力。全フォーマット共通のフィルタリングパス。
fn output_filtered<T: Serialize>(
    results: &[T],
    fields: &[String],
    format: &OutputFormat,
    save: &Option<String>,
) -> Result<(), AppError> {
    // 全レコードを Value に変換
    let values: Vec<serde_json::Value> = results
        .iter()
        .map(serde_json::to_value)
        .collect::<Result<_, _>>()?;

    // 先頭レコードのキーでフィールド名を検証（0件時はスキップ）
    if let Some(serde_json::Value::Object(ref map)) = values.first() {
        let invalid: Vec<&str> = fields
            .iter()
            .filter(|f| !map.contains_key(f.as_str()))
            .map(|f| f.as_str())
            .collect();
        if !invalid.is_empty() {
            let available: Vec<&str> = map.keys().map(|s| s.as_str()).collect();
            return Err(AppError::Usage(format!(
                "不明なフィールド: {}.\n  利用可能フィールド: {}",
                invalid.join(", "),
                available.join(", "),
            )));
        }
    }

    match format {
        OutputFormat::Json => {
            let filtered: Vec<serde_json::Value> = values
                .iter()
                .map(|v| {
                    if let serde_json::Value::Object(map) = v {
                        filter_object(map, fields)
                    } else {
                        v.clone()
                    }
                })
                .collect();
            let mut writer = make_writer(save)?;
            serde_json::to_writer_pretty(&mut writer, &filtered)?;
            writeln!(writer)?;
        }
        OutputFormat::Csv => {
            let mut wtr = csv::Writer::from_writer(make_writer(save)?);
            write_csv_filtered(&mut wtr, fields, &values)?;
        }
        OutputFormat::Table => {
            if save.is_none() && !io::stdout().is_terminal() {
                // パイプ時: CSV フォールバック
                let mut wtr = csv::Writer::from_writer(io::stdout());
                write_csv_filtered(&mut wtr, fields, &values)?;
            } else {
                let mut table = Table::new();
                table.load_preset(UTF8_FULL);
                table.set_header(fields.iter().map(|s| s.as_str()));
                for v in &values {
                    if let serde_json::Value::Object(map) = v {
                        table.add_row(row_from_fields(map, fields));
                    }
                }
                println!("{table}");
            }
        }
        OutputFormat::Parquet => {
            let path = save
                .as_deref()
                .ok_or_else(|| AppError::Usage("--output parquet requires --save <path>".into()))?;
            if values.is_empty() {
                eprintln!("No data to write.");
                return Ok(());
            }
            // フィルタ済み NDJSON を構築
            let mut buf: Vec<u8> = Vec::with_capacity(values.len() * 256);
            for v in &values {
                if let serde_json::Value::Object(map) = v {
                    serde_json::to_writer(&mut buf, &filter_object(map, fields))?;
                    buf.push(b'\n');
                }
            }
            return write_parquet_from_ndjson(&buf, path);
        }
    }
    saved_notice(save);
    Ok(())
}

// ── schema モジュール向け TableDisplay impls ─────────────────────────────────

impl TableDisplay for crate::schema::EndpointSchema {
    fn table_headers() -> Vec<&'static str> {
        vec!["Endpoint", "Description", "Fields"]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.key.to_string(),
            self.description.to_string(),
            self.field_count.to_string(),
        ]
    }
}

impl TableDisplay for crate::schema::FieldSchema {
    fn table_headers() -> Vec<&'static str> {
        vec!["Field", "Type", "Description"]
    }
    fn table_row(&self) -> Vec<String> {
        vec![
            self.name.to_string(),
            self.field_type.to_string(),
            self.description.to_string(),
        ]
    }
}

// ── ジェネリック output 関数 ──────────────────────────────────────────────────

pub fn output<T: Serialize + TableDisplay>(
    results: &[T],
    format: &OutputFormat,
    save: &Option<String>,
    fields: &FieldSelection,
) -> Result<(), AppError> {
    if let Some(ref field_list) = fields {
        return output_filtered(results, field_list, format, save);
    }
    match format {
        OutputFormat::Json => {
            write_json_pretty(results, save)?;
        }
        OutputFormat::Csv => {
            let mut wtr = csv::Writer::from_writer(make_writer(save)?);
            for item in results {
                wtr.serialize(item)?;
            }
            wtr.flush()?;
        }
        OutputFormat::Table => {
            if save.is_none() && !io::stdout().is_terminal() {
                // パイプ時: CSV出力（ヘッダー付き）
                let mut wtr = csv::Writer::from_writer(io::stdout());
                wtr.write_record(T::table_headers())?;
                for item in results {
                    wtr.write_record(item.table_row())?;
                }
                wtr.flush()?;
            } else {
                let mut table = Table::new();
                table.load_preset(UTF8_FULL);
                table.set_header(T::table_headers());
                for item in results {
                    table.add_row(item.table_row());
                }
                println!("{table}");
            }
        }
        OutputFormat::Parquet => {
            // write_parquet 内で saved_notice を呼ぶため、ここで return
            let path = save
                .as_deref()
                .ok_or_else(|| AppError::Usage("--output parquet requires --save <path>".into()))?;
            return write_parquet(results, path);
        }
    }
    saved_notice(save);
    Ok(())
}

// ── 特殊出力関数 ──────────────────────────────────────────────────────────────

pub fn output_margin_alert(
    results: &[MarginAlert],
    format: &OutputFormat,
    save: &Option<String>,
    fields: &FieldSelection,
) -> Result<(), AppError> {
    if let Some(ref field_list) = fields {
        return output_filtered(results, field_list, format, save);
    }
    if let OutputFormat::Csv = format {
        let mut wtr = csv::Writer::from_writer(make_writer(save)?);
        wtr.write_record([
            "PubDate",
            "Code",
            "AppDate",
            "PubReason",
            "ShrtOut",
            "ShrtOutChg",
            "ShrtOutRatio",
            "LongOut",
            "LongOutChg",
            "LongOutRatio",
            "SLRatio",
            "ShrtNegOut",
            "ShrtNegOutChg",
            "ShrtStdOut",
            "ShrtStdOutChg",
            "LongNegOut",
            "LongNegOutChg",
            "LongStdOut",
            "LongStdOutChg",
            "TSEMrgnRegCls",
        ])?;
        for item in results {
            let pub_reason_str = serde_json::to_string(&item.pub_reason)?;
            let shrt_out = item.shrt_out.to_string();
            let shrt_out_chg = item.shrt_out_chg.to_string();
            let shrt_out_ratio = item.shrt_out_ratio.to_string();
            let long_out = item.long_out.to_string();
            let long_out_chg = item.long_out_chg.to_string();
            let long_out_ratio = item.long_out_ratio.to_string();
            let sl_ratio = item.sl_ratio.to_string();
            let shrt_neg_out = item.shrt_neg_out.to_string();
            let shrt_neg_out_chg = item.shrt_neg_out_chg.to_string();
            let shrt_std_out = item.shrt_std_out.to_string();
            let shrt_std_out_chg = item.shrt_std_out_chg.to_string();
            let long_neg_out = item.long_neg_out.to_string();
            let long_neg_out_chg = item.long_neg_out_chg.to_string();
            let long_std_out = item.long_std_out.to_string();
            let long_std_out_chg = item.long_std_out_chg.to_string();
            wtr.write_record([
                item.pub_date.as_str(),
                item.code.as_str(),
                item.app_date.as_str(),
                pub_reason_str.as_str(),
                shrt_out.as_str(),
                shrt_out_chg.as_str(),
                shrt_out_ratio.as_str(),
                long_out.as_str(),
                long_out_chg.as_str(),
                long_out_ratio.as_str(),
                sl_ratio.as_str(),
                shrt_neg_out.as_str(),
                shrt_neg_out_chg.as_str(),
                shrt_std_out.as_str(),
                shrt_std_out_chg.as_str(),
                long_neg_out.as_str(),
                long_neg_out_chg.as_str(),
                long_std_out.as_str(),
                long_std_out_chg.as_str(),
                item.tse_mrgn_reg_cls.as_str(),
            ])?;
        }
        wtr.flush()?;
        saved_notice(save);
        Ok(())
    } else {
        output(results, format, save, fields)
    }
}

pub fn output_fins_details(
    results: &[FinsDetails],
    format: &OutputFormat,
    save: &Option<String>,
    fields: &FieldSelection,
) -> Result<(), AppError> {
    if let Some(ref field_list) = fields {
        return output_filtered(results, field_list, format, save);
    }
    if let OutputFormat::Csv = format {
        let mut wtr = csv::Writer::from_writer(make_writer(save)?);
        wtr.write_record(["DiscDate", "DiscTime", "Code", "DiscNo", "DocType", "FS"])?;
        for item in results {
            let fs_str = serde_json::to_string(&item.fs)?;
            wtr.write_record([
                item.disc_date.as_str(),
                item.disc_time.as_str(),
                item.code.as_str(),
                item.disc_no.as_str(),
                item.doc_type.as_str(),
                fs_str.as_str(),
            ])?;
        }
        wtr.flush()?;
        saved_notice(save);
        Ok(())
    } else {
        output(results, format, save, fields)
    }
}

pub fn output_bulk_get(
    url: &str,
    format: &OutputFormat,
    save: &Option<String>,
) -> Result<(), AppError> {
    match format {
        OutputFormat::Json => {
            let mut writer = make_writer(save)?;
            writeln!(
                writer,
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({"url": url}))?
            )?;
        }
        OutputFormat::Csv | OutputFormat::Table => {
            let mut writer = make_writer(save)?;
            writeln!(writer, "{}", url)?;
        }
        OutputFormat::Parquet => {
            return Err(AppError::Usage(
                "--output parquet is not supported for bulk get URL output".into(),
            ));
        }
    }
    saved_notice(save);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::StockMaster;

    fn make_stock_master() -> StockMaster {
        StockMaster {
            date: "2026-03-19".into(),
            code: "86970".into(),
            co_name: "テスト会社".into(),
            co_name_en: "Test Co".into(),
            sector17_code: "17".into(),
            sector17_code_name: "IT".into(),
            sector33_code: "33".into(),
            sector33_code_name: "IT33".into(),
            scale_category: "Large".into(),
            market_code: "0101".into(),
            market_code_name: "Prime".into(),
            margin_code: "1".into(),
            margin_code_name: "MarginOK".into(),
        }
    }

    #[test]
    fn test_output_csv_writes_header_and_row() {
        let items = vec![make_stock_master()];
        let mut buf: Vec<u8> = Vec::new();
        let mut wtr = csv::Writer::from_writer(&mut buf);
        wtr.write_record(StockMaster::table_headers()).unwrap();
        for item in &items {
            wtr.write_record(item.table_row()).unwrap();
        }
        wtr.flush().unwrap();
        drop(wtr);
        let csv_str = String::from_utf8(buf).unwrap();
        assert!(csv_str.contains("Date,Code,CoName"));
        assert!(csv_str.contains("86970"));
    }

    #[test]
    fn test_write_parquet_basic() {
        let items = vec![make_stock_master()];
        let tmp = std::env::temp_dir().join("test_parquet_basic.parquet");
        let path = tmp.to_str().unwrap();
        write_parquet(&items, path).unwrap();

        // ファイルが存在し、Parquet マジックバイト (PAR1) を持つことを確認
        let bytes = std::fs::read(path).unwrap();
        assert!(bytes.len() > 8);
        assert_eq!(&bytes[..4], b"PAR1");
        assert_eq!(&bytes[bytes.len() - 4..], b"PAR1");

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_write_parquet_column_order() {
        use parquet::file::reader::{FileReader, SerializedFileReader};

        let items = vec![make_stock_master()];
        let tmp = std::env::temp_dir().join("test_parquet_col_order.parquet");
        let path = tmp.to_str().unwrap();
        write_parquet(&items, path).unwrap();

        // Parquet メタデータからカラム名の順序を検証
        let file = File::open(path).unwrap();
        let reader = SerializedFileReader::new(file).unwrap();
        let col_names: Vec<&str> = reader
            .metadata()
            .file_metadata()
            .schema_descr()
            .columns()
            .iter()
            .map(|c| c.name())
            .collect();

        // StockMaster のフィールド宣言順（serde rename 適用後）
        assert_eq!(
            col_names,
            vec![
                "Date", "Code", "CoName", "CoNameEn", "S17", "S17Nm", "S33", "S33Nm", "ScaleCat",
                "Mkt", "MktNm", "Mrgn", "MrgnNm"
            ]
        );

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_write_parquet_empty() {
        let items: Vec<StockMaster> = vec![];
        let tmp = std::env::temp_dir().join("test_parquet_empty.parquet");
        let path = tmp.to_str().unwrap();
        // 空データは早期リターン（エラーなし、ファイルは作成されない）
        write_parquet(&items, path).unwrap();
        assert!(!tmp.exists());
    }

    #[test]
    fn test_value_to_cell() {
        use serde_json::json;
        assert_eq!(value_to_cell(&json!("hello")), "hello");
        assert_eq!(value_to_cell(&json!(null)), "");
        assert_eq!(value_to_cell(&json!(42)), "42");
        assert_eq!(value_to_cell(&json!(3.14)), "3.14");
        assert_eq!(value_to_cell(&json!(true)), "true");
        assert_eq!(value_to_cell(&json!(false)), "false");
        // ネストされた値は JSON 文字列化
        let obj_str = value_to_cell(&json!({"a": 1}));
        assert!(obj_str.contains("\"a\""));
        let arr_str = value_to_cell(&json!([1, 2]));
        assert!(arr_str.contains('1'));
    }

    #[test]
    fn test_output_filtered_csv() {
        let items = vec![make_stock_master()];
        let tmp = std::env::temp_dir().join("test_filtered_csv.csv");
        let save = Some(tmp.to_str().unwrap().to_string());
        let fields = vec!["Date".to_string(), "Code".to_string()];
        output_filtered(&items, &fields, &OutputFormat::Csv, &save).unwrap();
        let content = std::fs::read_to_string(tmp.as_path()).unwrap();
        assert!(content.contains("Date,Code"));
        assert!(content.contains("2026-03-19,86970"));
        // 他のフィールドは含まれない
        assert!(!content.contains("CoName"));
        std::fs::remove_file(tmp).ok();
    }

    #[test]
    fn test_output_filtered_json() {
        let items = vec![make_stock_master()];
        let tmp = std::env::temp_dir().join("test_filtered_json.json");
        let save = Some(tmp.to_str().unwrap().to_string());
        let fields = vec!["Date".to_string(), "Code".to_string()];
        output_filtered(&items, &fields, &OutputFormat::Json, &save).unwrap();
        let content = std::fs::read_to_string(tmp.as_path()).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.len(), 1);
        let obj = parsed[0].as_object().unwrap();
        assert!(obj.contains_key("Date"));
        assert!(obj.contains_key("Code"));
        assert!(!obj.contains_key("CoName"));
        std::fs::remove_file(tmp).ok();
    }

    #[test]
    fn test_output_filtered_field_order() {
        let items = vec![make_stock_master()];
        let tmp = std::env::temp_dir().join("test_filtered_field_order.csv");
        let save = Some(tmp.to_str().unwrap().to_string());
        // ユーザーが Code,Date の順で指定
        let fields = vec!["Code".to_string(), "Date".to_string()];
        output_filtered(&items, &fields, &OutputFormat::Csv, &save).unwrap();
        let content = std::fs::read_to_string(tmp.as_path()).unwrap();
        // ヘッダーが Code,Date の順になっていること
        let first_line = content.lines().next().unwrap();
        assert_eq!(first_line, "Code,Date");
        std::fs::remove_file(tmp).ok();
    }

    #[test]
    fn test_output_filtered_invalid_field() {
        let items = vec![make_stock_master()];
        let fields = vec!["Date".to_string(), "InvalidField".to_string()];
        let result = output_filtered(&items, &fields, &OutputFormat::Json, &None);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("InvalidField"));
        assert!(err_msg.contains("Date")); // 利用可能フィールドが含まれる
    }

    #[test]
    fn test_output_filtered_empty_results() {
        let items: Vec<StockMaster> = vec![];
        let tmp = std::env::temp_dir().join("test_filtered_empty.csv");
        let save = Some(tmp.to_str().unwrap().to_string());
        let fields = vec!["Date".to_string(), "Code".to_string()];
        // 0件でもエラーにならず、ヘッダーのみ出力
        output_filtered(&items, &fields, &OutputFormat::Csv, &save).unwrap();
        let content = std::fs::read_to_string(tmp.as_path()).unwrap();
        assert!(content.contains("Date,Code"));
        std::fs::remove_file(tmp).ok();
    }

    #[test]
    fn test_output_filtered_parquet() {
        use parquet::file::reader::{FileReader, SerializedFileReader};
        let items = vec![make_stock_master()];
        let tmp = std::env::temp_dir().join("test_filtered_parquet.parquet");
        let save = Some(tmp.to_str().unwrap().to_string());
        let fields = vec!["Date".to_string(), "Code".to_string()];
        output_filtered(&items, &fields, &OutputFormat::Parquet, &save).unwrap();
        let file = File::open(tmp.as_path()).unwrap();
        let reader = SerializedFileReader::new(file).unwrap();
        let col_names: Vec<String> = reader
            .metadata()
            .file_metadata()
            .schema_descr()
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect();
        assert_eq!(col_names, vec!["Date", "Code"]);
        std::fs::remove_file(tmp).ok();
    }
}

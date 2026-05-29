use crate::cli::OutputFormat;
use crate::error::AppError;
use crate::models::TdFilesInner;
use crate::output::output_bulk_get;
use tokio::io::AsyncWriteExt;

async fn download_to_file(
    http_client: &reqwest::Client,
    url: &str,
    filename: &str,
) -> Result<(), AppError> {
    let mut response = http_client.get(url).send().await?;
    let mut file = tokio::fs::File::create(filename).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    Ok(())
}

pub async fn download_bulk_file(
    http_client: &reqwest::Client,
    url: &str,
) -> Result<String, AppError> {
    let filename = url
        .split('?')
        .next()
        .and_then(|path| path.rsplit('/').next())
        .filter(|name| !name.is_empty())
        .unwrap_or("bulk_data.gz")
        .to_string();

    download_to_file(http_client, url, &filename).await?;
    Ok(filename)
}

pub async fn download_td_files(
    http_client: &reqwest::Client,
    files: &TdFilesInner,
    disc_no: &str,
) -> Result<(), AppError> {
    let targets: &[(&Option<String>, &str, &str)] = &[
        (&files.pdf, "pdf", "pdf"),
        (&files.summary_pdf, "summary", "pdf"),
        (&files.xbrl, "xbrl", "zip"),
    ];
    for (url_opt, prefix, ext) in targets {
        if let Some(url) = url_opt {
            let filename = format!("{prefix}_{disc_no}.{ext}");
            download_to_file(http_client, url, &filename).await?;
            eprintln!("Downloaded: {}", filename);
        }
    }
    Ok(())
}

pub async fn handle_bulk_download(
    http_client: &reqwest::Client,
    url: &str,
    download: bool,
    format: &OutputFormat,
    save: &Option<String>,
) -> Result<(), AppError> {
    if download {
        let filename = download_bulk_file(http_client, url).await?;
        eprintln!("Downloaded: {}", filename);
    } else {
        output_bulk_get(url, format, save)?;
    }
    Ok(())
}

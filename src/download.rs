use crate::cli::OutputFormat;
use crate::error::AppError;
use crate::output::output_bulk_get;
use tokio::io::AsyncWriteExt;

pub async fn download_bulk_file(
    http_client: &reqwest::Client,
    url: &str,
) -> Result<String, AppError> {
    let mut response = http_client.get(url).send().await?;

    let filename = url
        .split('?')
        .next()
        .and_then(|path| path.rsplit('/').next())
        .filter(|name| !name.is_empty())
        .unwrap_or("bulk_data.gz")
        .to_string();

    let mut file = tokio::fs::File::create(&filename).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    file.flush().await?;

    Ok(filename)
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

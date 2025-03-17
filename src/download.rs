use crate::client::{Client, ClientConfig};
use crate::{NekosBestError, NekosBestResponseSingle};
use image::AnimationDecoder;
#[cfg(feature = "blocking")]
use nb_blocking_util::blocking;
use reqwest::IntoUrl;
use tokio::io::AsyncWriteExt;

#[derive(Clone)]
pub enum DownloadResult {
    Image(image::DynamicImage),
    Gif(GifDownloadResult),
}

impl DownloadResult {
    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<(), NekosBestError> {
        match self {
            DownloadResult::Image(img) => img.save(path)?,
            DownloadResult::Gif(gif) => gif.save(path)?,
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct GifDownloadResult {
    frames: Vec<image::Frame>,
}

impl GifDownloadResult {
    pub fn get_frames(&self) -> &Vec<image::Frame> {
        &self.frames
    }

    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<(), NekosBestError> {
        self.save_with_extra(path, |_| Ok(()))
    }

    pub fn save_with_repeat(
        &self,
        path: impl AsRef<std::path::Path>,
        repeat: image::codecs::gif::Repeat,
    ) -> Result<(), NekosBestError> {
        self.save_with_extra(path, |encoder| {
            encoder.set_repeat(repeat)?;
            Ok(())
        })
    }

    fn save_with_extra(
        &self,
        path: impl AsRef<std::path::Path>,
        extra: impl FnOnce(
            &mut image::codecs::gif::GifEncoder<std::fs::File>,
        ) -> Result<(), NekosBestError>,
    ) -> Result<(), NekosBestError> {
        let mut encoder = image::codecs::gif::GifEncoder::new(std::fs::File::create(path)?);
        for frame in &self.frames {
            encoder.encode_frame(frame.clone())?;
        }
        extra(&mut encoder)?;
        Ok(())
    }
}

/// Downloads the image from the given response.
#[cfg_attr(feature = "blocking", blocking)]
pub async fn download(
    response: &NekosBestResponseSingle,
) -> Result<DownloadResult, NekosBestError> {
    download_with_client(&Client::new(ClientConfig::default()), response).await
}

/// Downloads the image from the given response using the given client.
#[cfg_attr(feature = "blocking", blocking)]
pub async fn download_with_client(
    client: &Client,
    response: &NekosBestResponseSingle,
) -> Result<DownloadResult, NekosBestError> {
    download_from_url_with_client(client, &response.url).await
}

/// Downloads the image from the given url.
#[cfg_attr(feature = "blocking", blocking)]
pub async fn download_from_url(url: impl IntoUrl) -> Result<DownloadResult, NekosBestError> {
    download_from_url_with_client(&Client::new(ClientConfig::default()), url).await
}

/// Downloads the image from the given url using the given client.
#[cfg_attr(feature = "blocking", blocking)]
pub async fn download_from_url_with_client(
    client: &Client,
    url: impl IntoUrl,
) -> Result<DownloadResult, NekosBestError> {
    let resp = crate::prepare_request(client.client.get(url)).send().await?.error_for_status()?;
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .ok_or(NekosBestError::MissingContentType)?
        .to_str()
        .unwrap()
        .to_owned();
    let bytes = resp.bytes().await?;
    match content_type.as_str() {
        "image/png" => Ok(DownloadResult::Image(image::load_from_memory(&bytes)?)),
        "image/gif" => {
            let decoder = image::codecs::gif::GifDecoder::new(std::io::Cursor::new(bytes))?;
            let frames = decoder.into_frames().collect_frames()?;
            Ok(DownloadResult::Gif(GifDownloadResult { frames }))
        }
        _ => Err(NekosBestError::MissingContentType),
    }
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn download_to_file(
    response: &NekosBestResponseSingle,
    file: impl AsRef<std::path::Path>,
) -> Result<(), NekosBestError> {
    download_to_file_with_client(
        &Client::new(ClientConfig::default()),
        response,
        file,
    ).await
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn download_to_file_with_client(
    client: &Client,
    response: &NekosBestResponseSingle,
    file: impl AsRef<std::path::Path>,
) -> Result<(), NekosBestError> {
    download_from_url_to_file_with_client(
        client,
        &response.url,
        file,
    ).await
}

#[cfg_attr(feature = "blocking", blocking)]
pub async fn download_from_url_to_file(
    url: impl IntoUrl,
    file: impl AsRef<std::path::Path>,
) -> Result<(), NekosBestError> {
    download_from_url_to_file_with_client(
        &Client::new(ClientConfig::default()),
        url,
        file,
    ).await
}

#[cfg(not(feature = "blocking"))]
pub async fn download_from_url_to_file_with_client(
    client: &Client,
    url: impl IntoUrl,
    file: impl AsRef<std::path::Path>,
) -> Result<(), NekosBestError> {
    use futures::StreamExt;

    let resp = crate::prepare_request(client.client.get(url)).send().await?.error_for_status()?;
    let mut stream = resp.bytes_stream();
    let mut f = tokio::fs::File::create(file).await?;

    while let Some(item) = stream.next().await {
        f.write_all(&item?).await?;
    }

    Ok(())
}


#[cfg(feature = "blocking")]
pub fn download_from_url_to_file_with_client(
    client: &Client,
    url: impl IntoUrl,
    file: impl AsRef<std::path::Path>,
) -> Result<(), NekosBestError> {
    use std::io::Write;

    let mut resp = crate::prepare_request(client.client.get(url)).send()?.error_for_status()?;
    let mut f = std::fs::File::create(file)?;

    std::io::copy(&mut resp, &mut f)?;

    Ok(())
}

//! D3 Fetch module
//! Provides synchronous HTTP fetch using ureq (native only).

#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_text(url: &str) -> Result<String, String> {
    match ureq::get(url).call() {
        Ok(resp) => resp.into_string().map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn fetch_text(_url: &str) -> Result<String, String> {
    Err("fetch not supported on wasm32".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_json(url: &str) -> Result<serde_json::Value, String> {
    let text = fetch_text(url)?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

#[cfg(target_arch = "wasm32")]
pub fn fetch_json(_url: &str) -> Result<serde_json::Value, String> {
    Err("fetch not supported on wasm32".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_csv(url: &str) -> Result<Vec<csv::StringRecord>, String> {
    let text = fetch_text(url)?;
    let mut rdr = csv::Reader::from_reader(text.as_bytes());
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result.map_err(|e| e.to_string())?;
        records.push(record);
    }
    Ok(records)
}

#[cfg(target_arch = "wasm32")]
pub fn fetch_csv(_url: &str) -> Result<Vec<csv::StringRecord>, String> {
    Err("fetch not supported on wasm32".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_tsv(url: &str) -> Result<Vec<csv::StringRecord>, String> {
    let text = fetch_text(url)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(text.as_bytes());
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result.map_err(|e| e.to_string())?;
        records.push(record);
    }
    Ok(records)
}

#[cfg(target_arch = "wasm32")]
pub fn fetch_tsv(_url: &str) -> Result<Vec<csv::StringRecord>, String> {
    Err("fetch not supported on wasm32".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_xml(url: &str) -> Result<String, String> {
    fetch_text(url) // Just return the XML as string; parsing can be added if needed
}

#[cfg(target_arch = "wasm32")]
pub fn fetch_xml(_url: &str) -> Result<String, String> {
    Err("fetch not supported on wasm32".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_html(url: &str) -> Result<String, String> {
    fetch_text(url) // Just return the HTML as string
}

#[cfg(target_arch = "wasm32")]
pub fn fetch_html(_url: &str) -> Result<String, String> {
    Err("fetch not supported on wasm32".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_blob(url: &str) -> Result<Vec<u8>, String> {
    use std::io::Read;
    match ureq::get(url).call() {
        Ok(mut resp) => {
            let mut buf = Vec::new();
            resp.into_reader().read_to_end(&mut buf).map_err(|e| e.to_string())?;
            Ok(buf)
        },
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn fetch_blob(_url: &str) -> Result<Vec<u8>, String> {
    Err("fetch not supported on wasm32".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_image(url: &str) -> Result<Vec<u8>, String> {
    fetch_blob(url)
}

#[cfg(target_arch = "wasm32")]
pub fn fetch_image(_url: &str) -> Result<Vec<u8>, String> {
    Err("fetch not supported on wasm32".to_string())
}

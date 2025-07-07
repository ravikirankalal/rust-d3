use ureq;
use serde_json::Value;
use csv::ReaderBuilder;
use std::io::Read;

pub fn fetch_text(url: &str) -> Result<String, String> {
    ureq::get(url).call().map_err(|e| e.to_string())?.into_string().map_err(|e| e.to_string())
}

pub fn fetch_json(url: &str) -> Result<Value, String> {
    let resp = ureq::get(url).call().map_err(|e| e.to_string())?;
    let text = resp.into_string().map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

pub fn fetch_csv(url: &str) -> Result<Vec<std::collections::HashMap<String, String>>, String> {
    let resp = ureq::get(url).call().map_err(|e| e.to_string())?;
    let mut reader = ReaderBuilder::new().from_reader(resp.into_reader());
    let headers = reader.headers().map_err(|e| e.to_string())?.clone();
    let mut records = Vec::new();
    for result in reader.into_records() {
        let record = result.map_err(|e| e.to_string())?;
        let mut map = std::collections::HashMap::new();
        for (i, value) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                map.insert(header.to_string(), value.to_string());
            }
        }
        records.push(map);
    }
    Ok(records)
}

pub fn fetch_tsv(url: &str) -> Result<Vec<std::collections::HashMap<String, String>>, String> {
    let resp = ureq::get(url).call().map_err(|e| e.to_string())?;
    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(resp.into_reader());
    let headers = reader.headers().map_err(|e| e.to_string())?.clone();
    let mut records = Vec::new();
    for result in reader.into_records() {
        let record = result.map_err(|e| e.to_string())?;
        let mut map = std::collections::HashMap::new();
        for (i, value) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                map.insert(header.to_string(), value.to_string());
            }
        }
        records.push(map);
    }
    Ok(records)
}

pub fn fetch_xml(url: &str) -> Result<String, String> {
    ureq::get(url).call().map_err(|e| e.to_string())?.into_string().map_err(|e| e.to_string())
}

pub fn fetch_blob(url: &str) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    ureq::get(url).call().map_err(|e| e.to_string())?.into_reader().read_to_end(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf)
}

pub fn fetch_buffer(url: &str) -> Result<Vec<u8>, String> {
    fetch_blob(url) // In Rust, blob and buffer are often handled similarly as Vec<u8>
}

pub fn fetch_svg(url: &str) -> Result<String, String> {
    ureq::get(url).call().map_err(|e| e.to_string())?.into_string().map_err(|e| e.to_string())
}

pub fn fetch_html(url: &str) -> Result<String, String> {
    fetch_text(url)
}

pub fn fetch_image(url: &str) -> Result<Vec<u8>, String> {
    fetch_blob(url)
}
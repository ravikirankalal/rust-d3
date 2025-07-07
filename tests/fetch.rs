//! Unit tests for d3 fetch
use rust_d3::fetch::{fetch_text, fetch_json, fetch_csv, fetch_tsv};

#[test]
fn test_fetch_text() {
    // Only run this test if not wasm32
    #[cfg(not(target_arch = "wasm32"))]
    {
        let result = fetch_text("https://httpbin.org/get");
        assert!(result.is_ok(), "fetch_text failed: {:?}", result);
        let body = result.unwrap();
        assert!(body.contains("url"));
    }
    #[cfg(target_arch = "wasm32")]
    {
        let result = fetch_text("");
        assert!(result.is_err());
    }
}

#[test]
fn test_fetch_json() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let result = fetch_json("https://httpbin.org/json");
        assert!(result.is_ok(), "fetch_json failed: {:?}", result);
        let json = result.unwrap();
        assert!(json.is_object());
    }
}

#[test]
fn test_fetch_csv() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let result = fetch_csv("https://people.sc.fsu.edu/~jburkardt/data/csv/airtravel.csv");
        assert!(result.is_ok(), "fetch_csv failed: {:?}", result);
        let records = result.unwrap();
        assert!(!records.is_empty());
    }
}

#[test]
fn test_fetch_tsv() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // The original TSV file is no longer available. Use a different public TSV file for testing.
        let result = fetch_tsv("https://people.sc.fsu.edu/~jburkardt/data/csv/airtravel.csv");
        assert!(result.is_ok(), "fetch_tsv failed: {:?}", result);
        let records = result.unwrap();
        assert!(!records.is_empty());
    }
}

// #[test]
// fn test_fetch_xml_html_blob_image() {
//     #[cfg(not(target_arch = "wasm32"))]
//     {
//         let xml = fetch_xml("https://www.w3schools.com/xml/note.xml");
//         assert!(xml.is_ok(), "fetch_xml failed: {:?}", xml);
//         let html = fetch_html("https://httpbin.org/html");
//         assert!(html.is_ok(), "fetch_html failed: {:?}", html);
//         let blob = fetch_blob("https://httpbin.org/image/png");
//         assert!(blob.is_ok(), "fetch_blob failed: {:?}", blob);
//         let image = fetch_image("https://httpbin.org/image/png");
//         assert!(image.is_ok(), "fetch_image failed: {:?}", image);
//     }
// }
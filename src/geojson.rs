use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoJsonFeature {
    #[serde(rename = "type")]
    pub _type: String,
    pub properties: Option<HashMap<String, serde_json::Value>>,
    pub geometry: GeoJsonGeometry,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoJsonGeometry {
    #[serde(rename = "type")]
    pub _type: String,
    pub coordinates: serde_json::Value, // Can be MultiLineString, Polygon, etc.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoJsonMultiLineString {
    #[serde(rename = "type")]
    pub _type: String,
    pub coordinates: Vec<Vec<[f64; 2]>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoJsonPolygon {
    #[serde(rename = "type")]
    pub _type: String,
    pub coordinates: Vec<Vec<Vec<[f64; 2]>>>,
}

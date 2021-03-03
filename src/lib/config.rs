use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub source_url: String,
    pub check_interval: u32
}

#[derive(Debug, Deserialize)]
pub struct ScalerConfig {
    #[serde(rename = "scalingEnabled")]
    pub scaling_enabled: bool,
    pub objects: Vec<ScalerResource>
}

#[derive(Debug, Deserialize)]
pub struct ScalerResource {
    pub kind: String,
    pub namespace: String,
    pub name: String
}
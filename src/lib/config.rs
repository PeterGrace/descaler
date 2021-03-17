use serde::Deserialize;
use anyhow::{Result, bail};
use std::convert::From;


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub source_url: String,
    pub check_interval: u32
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct ScalerConfig {
    #[serde(rename = "scalingEnabled")]
    pub scaling_enabled: bool,
    pub objects: Vec<ScalerResource>
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct ScalerResource {
    pub kind: String,
    pub namespace: String,
    pub name: String
}

impl ScalerConfig {
    pub(crate) fn from_string(input_string: String) -> Result<Self> {

        match serde_yaml::from_str(input_string.as_str()) {
            Ok(c) => Ok(c),
            Err(e) => bail!("Couldn't parse yaml from the remote site: {:#?}", e)
        }
    }

}
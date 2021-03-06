/*
 * scaledobjects.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledObjectScaleTargetRef : ScaleTarget holds the a reference to the scale target Object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledObjectScaleTargetRef {
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(rename = "envSourceContainerName", skip_serializing_if = "Option::is_none")]
    pub env_source_container_name: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
}

impl ScaledObjectScaleTargetRef {
    /// ScaleTarget holds the a reference to the scale target Object
    pub fn new(name: String) -> ScaledObjectScaleTargetRef {
        ScaledObjectScaleTargetRef {
            api_version: None,
            env_source_container_name: None,
            kind: None,
            name,
        }
    }
}



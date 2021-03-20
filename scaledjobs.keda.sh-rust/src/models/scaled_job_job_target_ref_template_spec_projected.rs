/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecProjected : Items for all in one resources secrets, configmaps, and downward API



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecProjected {
    /// Mode bits to use on created files by default. Must be a value between 0 and 0777. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(rename = "defaultMode", skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
    /// list of volume projections
    #[serde(rename = "sources")]
    pub sources: Vec<crate::models::ScaledJobJobTargetRefTemplateSpecProjectedSources>,
}

impl ScaledJobJobTargetRefTemplateSpecProjected {
    /// Items for all in one resources secrets, configmaps, and downward API
    pub fn new(sources: Vec<crate::models::ScaledJobJobTargetRefTemplateSpecProjectedSources>) -> ScaledJobJobTargetRefTemplateSpecProjected {
        ScaledJobJobTargetRefTemplateSpecProjected {
            default_mode: None,
            sources,
        }
    }
}


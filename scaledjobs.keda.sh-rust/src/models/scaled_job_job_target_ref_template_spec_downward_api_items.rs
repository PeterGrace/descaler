/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecDownwardApiItems : DownwardAPIVolumeFile represents information to create the file containing the pod field



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecDownwardApiItems {
    #[serde(rename = "fieldRef", skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<crate::models::ScaledJobJobTargetRefTemplateSpecDownwardApiFieldRef>,
    /// Optional: mode bits to use on this file, must be a value between 0 and 0777. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "resourceFieldRef", skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<crate::models::ScaledJobJobTargetRefTemplateSpecDownwardApiResourceFieldRef>,
}

impl ScaledJobJobTargetRefTemplateSpecDownwardApiItems {
    /// DownwardAPIVolumeFile represents information to create the file containing the pod field
    pub fn new(path: String) -> ScaledJobJobTargetRefTemplateSpecDownwardApiItems {
        ScaledJobJobTargetRefTemplateSpecDownwardApiItems {
            field_ref: None,
            mode: None,
            path,
            resource_field_ref: None,
        }
    }
}


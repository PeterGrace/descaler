/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecEmptyDir : EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecEmptyDir {
    /// What type of storage medium should back this directory. The default is \"\" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    #[serde(rename = "medium", skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    /// Total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: http://kubernetes.io/docs/user-guide/volumes#emptydir
    #[serde(rename = "sizeLimit", skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<String>,
}

impl ScaledJobJobTargetRefTemplateSpecEmptyDir {
    /// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    pub fn new() -> ScaledJobJobTargetRefTemplateSpecEmptyDir {
        ScaledJobJobTargetRefTemplateSpecEmptyDir {
            medium: None,
            size_limit: None,
        }
    }
}



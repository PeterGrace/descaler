/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecFc : FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecFc {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. TODO: how do we prevent errors in the filesystem from compromising the machine
    #[serde(rename = "fsType", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// Optional: FC target lun number
    #[serde(rename = "lun", skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Optional: FC target worldwide names (WWNs)
    #[serde(rename = "targetWWNs", skip_serializing_if = "Option::is_none")]
    pub target_wwns: Option<Vec<String>>,
    /// Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
    #[serde(rename = "wwids", skip_serializing_if = "Option::is_none")]
    pub wwids: Option<Vec<String>>,
}

impl ScaledJobJobTargetRefTemplateSpecFc {
    /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    pub fn new() -> ScaledJobJobTargetRefTemplateSpecFc {
        ScaledJobJobTargetRefTemplateSpecFc {
            fs_type: None,
            lun: None,
            read_only: None,
            target_wwns: None,
            wwids: None,
        }
    }
}


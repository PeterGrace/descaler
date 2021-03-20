/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecStorageos : StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecStorageos {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.
    #[serde(rename = "fsType", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "secretRef", skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<crate::models::ScaledJobJobTargetRefTemplateSpecStorageosSecretRef>,
    /// VolumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
    #[serde(rename = "volumeName", skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
    /// VolumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to \"default\" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
    #[serde(rename = "volumeNamespace", skip_serializing_if = "Option::is_none")]
    pub volume_namespace: Option<String>,
}

impl ScaledJobJobTargetRefTemplateSpecStorageos {
    /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
    pub fn new() -> ScaledJobJobTargetRefTemplateSpecStorageos {
        ScaledJobJobTargetRefTemplateSpecStorageos {
            fs_type: None,
            read_only: None,
            secret_ref: None,
            volume_name: None,
            volume_namespace: None,
        }
    }
}



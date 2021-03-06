/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecVolumeDevices : volumeDevice describes a mapping of a raw block device within a container.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecVolumeDevices {
    /// devicePath is the path inside of the container that the device will be mapped to.
    #[serde(rename = "devicePath")]
    pub device_path: String,
    /// name must match the name of a persistentVolumeClaim in the pod
    #[serde(rename = "name")]
    pub name: String,
}

impl ScaledJobJobTargetRefTemplateSpecVolumeDevices {
    /// volumeDevice describes a mapping of a raw block device within a container.
    pub fn new(device_path: String, name: String) -> ScaledJobJobTargetRefTemplateSpecVolumeDevices {
        ScaledJobJobTargetRefTemplateSpecVolumeDevices {
            device_path,
            name,
        }
    }
}



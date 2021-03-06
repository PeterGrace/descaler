/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecVolumes : Volume represents a named volume in a pod that may be accessed by any container in the pod.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecVolumes {
    #[serde(rename = "awsElasticBlockStore", skip_serializing_if = "Option::is_none")]
    pub aws_elastic_block_store: Option<crate::models::ScaledJobJobTargetRefTemplateSpecAwsElasticBlockStore>,
    #[serde(rename = "azureDisk", skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<crate::models::ScaledJobJobTargetRefTemplateSpecAzureDisk>,
    #[serde(rename = "azureFile", skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<crate::models::ScaledJobJobTargetRefTemplateSpecAzureFile>,
    #[serde(rename = "cephfs", skip_serializing_if = "Option::is_none")]
    pub cephfs: Option<crate::models::ScaledJobJobTargetRefTemplateSpecCephfs>,
    #[serde(rename = "cinder", skip_serializing_if = "Option::is_none")]
    pub cinder: Option<crate::models::ScaledJobJobTargetRefTemplateSpecCinder>,
    #[serde(rename = "configMap", skip_serializing_if = "Option::is_none")]
    pub config_map: Option<crate::models::ScaledJobJobTargetRefTemplateSpecConfigMap>,
    #[serde(rename = "csi", skip_serializing_if = "Option::is_none")]
    pub csi: Option<crate::models::ScaledJobJobTargetRefTemplateSpecCsi>,
    #[serde(rename = "downwardAPI", skip_serializing_if = "Option::is_none")]
    pub downward_api: Option<crate::models::ScaledJobJobTargetRefTemplateSpecDownwardApi>,
    #[serde(rename = "emptyDir", skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<crate::models::ScaledJobJobTargetRefTemplateSpecEmptyDir>,
    #[serde(rename = "fc", skip_serializing_if = "Option::is_none")]
    pub fc: Option<crate::models::ScaledJobJobTargetRefTemplateSpecFc>,
    #[serde(rename = "flexVolume", skip_serializing_if = "Option::is_none")]
    pub flex_volume: Option<crate::models::ScaledJobJobTargetRefTemplateSpecFlexVolume>,
    #[serde(rename = "flocker", skip_serializing_if = "Option::is_none")]
    pub flocker: Option<crate::models::ScaledJobJobTargetRefTemplateSpecFlocker>,
    #[serde(rename = "gcePersistentDisk", skip_serializing_if = "Option::is_none")]
    pub gce_persistent_disk: Option<crate::models::ScaledJobJobTargetRefTemplateSpecGcePersistentDisk>,
    #[serde(rename = "gitRepo", skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<crate::models::ScaledJobJobTargetRefTemplateSpecGitRepo>,
    #[serde(rename = "glusterfs", skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<crate::models::ScaledJobJobTargetRefTemplateSpecGlusterfs>,
    #[serde(rename = "hostPath", skip_serializing_if = "Option::is_none")]
    pub host_path: Option<crate::models::ScaledJobJobTargetRefTemplateSpecHostPath>,
    #[serde(rename = "iscsi", skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<crate::models::ScaledJobJobTargetRefTemplateSpecIscsi>,
    /// Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "nfs", skip_serializing_if = "Option::is_none")]
    pub nfs: Option<crate::models::ScaledJobJobTargetRefTemplateSpecNfs>,
    #[serde(rename = "persistentVolumeClaim", skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim: Option<crate::models::ScaledJobJobTargetRefTemplateSpecPersistentVolumeClaim>,
    #[serde(rename = "photonPersistentDisk", skip_serializing_if = "Option::is_none")]
    pub photon_persistent_disk: Option<crate::models::ScaledJobJobTargetRefTemplateSpecPhotonPersistentDisk>,
    #[serde(rename = "portworxVolume", skip_serializing_if = "Option::is_none")]
    pub portworx_volume: Option<crate::models::ScaledJobJobTargetRefTemplateSpecPortworxVolume>,
    #[serde(rename = "projected", skip_serializing_if = "Option::is_none")]
    pub projected: Option<crate::models::ScaledJobJobTargetRefTemplateSpecProjected>,
    #[serde(rename = "quobyte", skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<crate::models::ScaledJobJobTargetRefTemplateSpecQuobyte>,
    #[serde(rename = "rbd", skip_serializing_if = "Option::is_none")]
    pub rbd: Option<crate::models::ScaledJobJobTargetRefTemplateSpecRbd>,
    #[serde(rename = "scaleIO", skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<crate::models::ScaledJobJobTargetRefTemplateSpecScaleIo>,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<crate::models::ScaledJobJobTargetRefTemplateSpecSecret>,
    #[serde(rename = "storageos", skip_serializing_if = "Option::is_none")]
    pub storageos: Option<crate::models::ScaledJobJobTargetRefTemplateSpecStorageos>,
    #[serde(rename = "vsphereVolume", skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: Option<crate::models::ScaledJobJobTargetRefTemplateSpecVsphereVolume>,
}

impl ScaledJobJobTargetRefTemplateSpecVolumes {
    /// Volume represents a named volume in a pod that may be accessed by any container in the pod.
    pub fn new(name: String) -> ScaledJobJobTargetRefTemplateSpecVolumes {
        ScaledJobJobTargetRefTemplateSpecVolumes {
            aws_elastic_block_store: None,
            azure_disk: None,
            azure_file: None,
            cephfs: None,
            cinder: None,
            config_map: None,
            csi: None,
            downward_api: None,
            empty_dir: None,
            fc: None,
            flex_volume: None,
            flocker: None,
            gce_persistent_disk: None,
            git_repo: None,
            glusterfs: None,
            host_path: None,
            iscsi: None,
            name,
            nfs: None,
            persistent_volume_claim: None,
            photon_persistent_disk: None,
            portworx_volume: None,
            projected: None,
            quobyte: None,
            rbd: None,
            scale_io: None,
            secret: None,
            storageos: None,
            vsphere_volume: None,
        }
    }
}



/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecAffinity : If specified, the pod's scheduling constraints



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecAffinity {
    #[serde(rename = "nodeAffinity", skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<crate::models::ScaledJobJobTargetRefTemplateSpecAffinityNodeAffinity>,
    #[serde(rename = "podAffinity", skip_serializing_if = "Option::is_none")]
    pub pod_affinity: Option<crate::models::ScaledJobJobTargetRefTemplateSpecAffinityPodAffinity>,
    #[serde(rename = "podAntiAffinity", skip_serializing_if = "Option::is_none")]
    pub pod_anti_affinity: Option<crate::models::ScaledJobJobTargetRefTemplateSpecAffinityPodAntiAffinity>,
}

impl ScaledJobJobTargetRefTemplateSpecAffinity {
    /// If specified, the pod's scheduling constraints
    pub fn new() -> ScaledJobJobTargetRefTemplateSpecAffinity {
        ScaledJobJobTargetRefTemplateSpecAffinity {
            node_affinity: None,
            pod_affinity: None,
            pod_anti_affinity: None,
        }
    }
}



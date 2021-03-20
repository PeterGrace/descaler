/*
 * scaledobjects.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Status : ScaledObjectStatus is the status for a ScaledObject resource



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
    /// Conditions an array representation to store multiple Conditions
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::StatusConditions>>,
    #[serde(rename = "externalMetricNames", skip_serializing_if = "Option::is_none")]
    pub external_metric_names: Option<Vec<String>>,
    #[serde(rename = "lastActiveTime", skip_serializing_if = "Option::is_none")]
    pub last_active_time: Option<String>,
    #[serde(rename = "originalReplicaCount", skip_serializing_if = "Option::is_none")]
    pub original_replica_count: Option<i32>,
    #[serde(rename = "scaleTargetGVKR", skip_serializing_if = "Option::is_none")]
    pub scale_target_gvkr: Option<crate::models::StatusScaleTargetGvkr>,
    #[serde(rename = "scaleTargetKind", skip_serializing_if = "Option::is_none")]
    pub scale_target_kind: Option<String>,
}

impl Status {
    /// ScaledObjectStatus is the status for a ScaledObject resource
    pub fn new() -> Status {
        Status {
            conditions: None,
            external_metric_names: None,
            last_active_time: None,
            original_replica_count: None,
            scale_target_gvkr: None,
            scale_target_kind: None,
        }
    }
}



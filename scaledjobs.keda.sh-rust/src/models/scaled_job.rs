/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJob : ScaledJobSpec defines the desired state of ScaledJob

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(CustomResource, Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
#[kube(
    group = "keda.sh",
    kind = "ScaledJob",
    version = "v1alpha1",
    namespaced,
    derive="PartialEq",
    derive="Default"
)]
pub struct ScaledJobSpec {
    #[serde(rename = "envSourceContainerName", skip_serializing_if = "Option::is_none")]
    pub env_source_container_name: Option<String>,
    #[serde(rename = "failedJobsHistoryLimit", skip_serializing_if = "Option::is_none")]
    pub failed_jobs_history_limit: Option<i32>,
    #[serde(rename = "jobTargetRef")]
    pub job_target_ref: crate::models::ScaledJobJobTargetRef,
    #[serde(rename = "maxReplicaCount", skip_serializing_if = "Option::is_none")]
    pub max_replica_count: Option<i32>,
    #[serde(rename = "pollingInterval", skip_serializing_if = "Option::is_none")]
    pub polling_interval: Option<i32>,
    #[serde(rename = "successfulJobsHistoryLimit", skip_serializing_if = "Option::is_none")]
    pub successful_jobs_history_limit: Option<i32>,
    #[serde(rename = "triggers")]
    pub triggers: Vec<crate::models::ScaledJobTriggers>,
}

impl ScaledJobSpec {
    /// ScaledJobSpec defines the desired state of ScaledJob
    pub fn new(job_target_ref: crate::models::ScaledJobJobTargetRef, triggers: Vec<crate::models::ScaledJobTriggers>) -> ScaledJobSpec {
        ScaledJobSpec {
            env_source_container_name: None,
            failed_jobs_history_limit: None,
            job_target_ref,
            max_replica_count: None,
            polling_interval: None,
            successful_jobs_history_limit: None,
            triggers,
        }
    }
}


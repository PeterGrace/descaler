/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecReadinessProbe : Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecReadinessProbe {
    #[serde(rename = "exec", skip_serializing_if = "Option::is_none")]
    pub exec: Option<crate::models::ScaledJobJobTargetRefTemplateSpecLifecyclePostStartExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(rename = "failureThreshold", skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[serde(rename = "httpGet", skip_serializing_if = "Option::is_none")]
    pub http_get: Option<crate::models::ScaledJobJobTargetRefTemplateSpecLifecyclePostStartHttpGet>,
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(rename = "initialDelaySeconds", skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(rename = "periodSeconds", skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    #[serde(rename = "successThreshold", skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    #[serde(rename = "tcpSocket", skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<crate::models::ScaledJobJobTargetRefTemplateSpecLifecyclePostStartTcpSocket>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(rename = "timeoutSeconds", skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}

impl ScaledJobJobTargetRefTemplateSpecReadinessProbe {
    /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    pub fn new() -> ScaledJobJobTargetRefTemplateSpecReadinessProbe {
        ScaledJobJobTargetRefTemplateSpecReadinessProbe {
            exec: None,
            failure_threshold: None,
            http_get: None,
            initial_delay_seconds: None,
            period_seconds: None,
            success_threshold: None,
            tcp_socket: None,
            timeout_seconds: None,
        }
    }
}



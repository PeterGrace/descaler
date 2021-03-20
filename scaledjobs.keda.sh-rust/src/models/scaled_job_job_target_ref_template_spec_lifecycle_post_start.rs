/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecLifecyclePostStart : PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecLifecyclePostStart {
    #[serde(rename = "exec", skip_serializing_if = "Option::is_none")]
    pub exec: Option<crate::models::ScaledJobJobTargetRefTemplateSpecLifecyclePostStartExec>,
    #[serde(rename = "httpGet", skip_serializing_if = "Option::is_none")]
    pub http_get: Option<crate::models::ScaledJobJobTargetRefTemplateSpecLifecyclePostStartHttpGet>,
    #[serde(rename = "tcpSocket", skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<crate::models::ScaledJobJobTargetRefTemplateSpecLifecyclePostStartTcpSocket>,
}

impl ScaledJobJobTargetRefTemplateSpecLifecyclePostStart {
    /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
    pub fn new() -> ScaledJobJobTargetRefTemplateSpecLifecyclePostStart {
        ScaledJobJobTargetRefTemplateSpecLifecyclePostStart {
            exec: None,
            http_get: None,
            tcp_socket: None,
        }
    }
}


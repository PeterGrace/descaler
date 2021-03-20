/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecEnvFrom : EnvFromSource represents the source of a set of ConfigMaps



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecEnvFrom {
    #[serde(rename = "configMapRef", skip_serializing_if = "Option::is_none")]
    pub config_map_ref: Option<crate::models::ScaledJobJobTargetRefTemplateSpecConfigMapRef>,
    /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "secretRef", skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<crate::models::ScaledJobJobTargetRefTemplateSpecSecretRef>,
}

impl ScaledJobJobTargetRefTemplateSpecEnvFrom {
    /// EnvFromSource represents the source of a set of ConfigMaps
    pub fn new() -> ScaledJobJobTargetRefTemplateSpecEnvFrom {
        ScaledJobJobTargetRefTemplateSpecEnvFrom {
            config_map_ref: None,
            prefix: None,
            secret_ref: None,
        }
    }
}


/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecValueFromConfigMapKeyRef : Selects a key of a ConfigMap.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecValueFromConfigMapKeyRef {
    /// The key to select.
    #[serde(rename = "key")]
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(rename = "optional", skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

impl ScaledJobJobTargetRefTemplateSpecValueFromConfigMapKeyRef {
    /// Selects a key of a ConfigMap.
    pub fn new(key: String) -> ScaledJobJobTargetRefTemplateSpecValueFromConfigMapKeyRef {
        ScaledJobJobTargetRefTemplateSpecValueFromConfigMapKeyRef {
            key,
            name: None,
            optional: None,
        }
    }
}



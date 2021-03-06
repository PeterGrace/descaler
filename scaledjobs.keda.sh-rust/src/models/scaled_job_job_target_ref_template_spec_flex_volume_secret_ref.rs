/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecFlexVolumeSecretRef : Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecFlexVolumeSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ScaledJobJobTargetRefTemplateSpecFlexVolumeSecretRef {
    /// Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
    pub fn new() -> ScaledJobJobTargetRefTemplateSpecFlexVolumeSecretRef {
        ScaledJobJobTargetRefTemplateSpecFlexVolumeSecretRef {
            name: None,
        }
    }
}



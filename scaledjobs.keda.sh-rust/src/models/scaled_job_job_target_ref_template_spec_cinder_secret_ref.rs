/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecCinderSecretRef : Optional: points to a secret object containing parameters used to connect to OpenStack.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecCinderSecretRef {
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ScaledJobJobTargetRefTemplateSpecCinderSecretRef {
    /// Optional: points to a secret object containing parameters used to connect to OpenStack.
    pub fn new() -> ScaledJobJobTargetRefTemplateSpecCinderSecretRef {
        ScaledJobJobTargetRefTemplateSpecCinderSecretRef {
            name: None,
        }
    }
}



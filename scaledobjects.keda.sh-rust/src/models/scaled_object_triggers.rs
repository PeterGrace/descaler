/*
 * scaledobjects.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledObjectTriggers : ScaleTriggers reference the scaler that will be used



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ScaledObjectTriggers {
    #[serde(rename = "authenticationRef", skip_serializing_if = "Option::is_none")]
    pub authentication_ref: Option<crate::models::ScaledObjectAuthenticationRef>,
    #[serde(rename = "metadata")]
    pub metadata: ::std::collections::HashMap<String, String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
}

impl ScaledObjectTriggers {
    /// ScaleTriggers reference the scaler that will be used
    pub fn new(metadata: ::std::collections::HashMap<String, String>, _type: String) -> ScaledObjectTriggers {
        ScaledObjectTriggers {
            authentication_ref: None,
            metadata,
            name: None,
            _type,
        }
    }
}



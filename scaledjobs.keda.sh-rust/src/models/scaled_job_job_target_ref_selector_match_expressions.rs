/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefSelectorMatchExpressions : A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    #[serde(rename = "key")]
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    #[serde(rename = "operator")]
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl ScaledJobJobTargetRefSelectorMatchExpressions {
    /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
    pub fn new(key: String, operator: String) -> ScaledJobJobTargetRefSelectorMatchExpressions {
        ScaledJobJobTargetRefSelectorMatchExpressions {
            key,
            operator,
            values: None,
        }
    }
}



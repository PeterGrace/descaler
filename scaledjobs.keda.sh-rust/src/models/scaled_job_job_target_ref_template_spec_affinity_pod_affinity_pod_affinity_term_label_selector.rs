/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecAffinityPodAffinityPodAffinityTermLabelSelector : A label query over a set of resources, in this case pods.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecAffinityPodAffinityPodAffinityTermLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(rename = "matchExpressions", skip_serializing_if = "Option::is_none")]
    pub match_expressions: Option<Vec<crate::models::ScaledJobJobTargetRefSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is \"key\", the operator is \"In\", and the values array contains only \"value\". The requirements are ANDed.
    #[serde(rename = "matchLabels", skip_serializing_if = "Option::is_none")]
    pub match_labels: Option<::std::collections::HashMap<String, String>>,
}

impl ScaledJobJobTargetRefTemplateSpecAffinityPodAffinityPodAffinityTermLabelSelector {
    /// A label query over a set of resources, in this case pods.
    pub fn new() -> ScaledJobJobTargetRefTemplateSpecAffinityPodAffinityPodAffinityTermLabelSelector {
        ScaledJobJobTargetRefTemplateSpecAffinityPodAffinityPodAffinityTermLabelSelector {
            match_expressions: None,
            match_labels: None,
        }
    }
}


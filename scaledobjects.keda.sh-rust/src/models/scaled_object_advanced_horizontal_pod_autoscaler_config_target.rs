/*
 * scaledobjects.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledObjectAdvancedHorizontalPodAutoscalerConfigTarget : target specifies the target value for the given metric



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ScaledObjectAdvancedHorizontalPodAutoscalerConfigTarget {
    /// averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type
    #[serde(rename = "averageUtilization", skip_serializing_if = "Option::is_none")]
    pub average_utilization: Option<i32>,
    /// averageValue is the target value of the average of the metric across all relevant pods (as a quantity)
    #[serde(rename = "averageValue", skip_serializing_if = "Option::is_none")]
    pub average_value: Option<String>,
    /// type represents whether the metric type is Utilization, Value, or AverageValue
    #[serde(rename = "type")]
    pub _type: String,
    /// value is the target value of the metric (as a quantity).
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ScaledObjectAdvancedHorizontalPodAutoscalerConfigTarget {
    /// target specifies the target value for the given metric
    pub fn new(_type: String) -> ScaledObjectAdvancedHorizontalPodAutoscalerConfigTarget {
        ScaledObjectAdvancedHorizontalPodAutoscalerConfigTarget {
            average_utilization: None,
            average_value: None,
            _type,
            value: None,
        }
    }
}



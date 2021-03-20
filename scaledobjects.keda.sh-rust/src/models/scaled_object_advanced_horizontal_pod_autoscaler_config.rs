/*
 * scaledobjects.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledObjectAdvancedHorizontalPodAutoscalerConfig : HorizontalPodAutoscalerConfig specifies horizontal scale config



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ScaledObjectAdvancedHorizontalPodAutoscalerConfig {
    #[serde(rename = "behavior", skip_serializing_if = "Option::is_none")]
    pub behavior: Option<crate::models::ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehavior>,
    #[serde(rename = "resourceMetrics", skip_serializing_if = "Option::is_none")]
    pub resource_metrics: Option<Vec<crate::models::ScaledObjectAdvancedHorizontalPodAutoscalerConfigResourceMetrics>>,
}

impl ScaledObjectAdvancedHorizontalPodAutoscalerConfig {
    /// HorizontalPodAutoscalerConfig specifies horizontal scale config
    pub fn new() -> ScaledObjectAdvancedHorizontalPodAutoscalerConfig {
        ScaledObjectAdvancedHorizontalPodAutoscalerConfig {
            behavior: None,
            resource_metrics: None,
        }
    }
}


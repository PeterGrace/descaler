/*
 * scaledobjects.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleDown : scaleDown is scaling policy for scaling Down. If not set, the default value is to allow to scale down to minReplicas pods, with a 300 second stabilization window (i.e., the highest recommendation for the last 300sec is used).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleDown {
    /// policies is a list of potential scaling polices which can be used during scaling. At least one policy must be specified, otherwise the HPAScalingRules will be discarded as invalid
    #[serde(rename = "policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<crate::models::ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleDownPolicies>>,
    /// selectPolicy is used to specify which policy should be used. If not set, the default value MaxPolicySelect is used.
    #[serde(rename = "selectPolicy", skip_serializing_if = "Option::is_none")]
    pub select_policy: Option<String>,
    /// StabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).
    #[serde(rename = "stabilizationWindowSeconds", skip_serializing_if = "Option::is_none")]
    pub stabilization_window_seconds: Option<i32>,
}

impl ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleDown {
    /// scaleDown is scaling policy for scaling Down. If not set, the default value is to allow to scale down to minReplicas pods, with a 300 second stabilization window (i.e., the highest recommendation for the last 300sec is used).
    pub fn new() -> ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleDown {
        ScaledObjectAdvancedHorizontalPodAutoscalerConfigBehaviorScaleDown {
            policies: None,
            select_policy: None,
            stabilization_window_seconds: None,
        }
    }
}


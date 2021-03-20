/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecLifecyclePostStartExec : One and only one of the following should be specified. Exec specifies the action to take.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecLifecyclePostStartExec {
    /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
}

impl ScaledJobJobTargetRefTemplateSpecLifecyclePostStartExec {
    /// One and only one of the following should be specified. Exec specifies the action to take.
    pub fn new() -> ScaledJobJobTargetRefTemplateSpecLifecyclePostStartExec {
        ScaledJobJobTargetRefTemplateSpecLifecyclePostStartExec {
            command: None,
        }
    }
}



/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecSecurityContext1 : SecurityContext is not allowed for ephemeral containers.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecSecurityContext1 {
    /// AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN
    #[serde(rename = "allowPrivilegeEscalation", skip_serializing_if = "Option::is_none")]
    pub allow_privilege_escalation: Option<bool>,
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<crate::models::ScaledJobJobTargetRefTemplateSpecSecurityContextCapabilities>,
    /// Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false.
    #[serde(rename = "privileged", skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled.
    #[serde(rename = "procMount", skip_serializing_if = "Option::is_none")]
    pub proc_mount: Option<String>,
    /// Whether this container has a read-only root filesystem. Default is false.
    #[serde(rename = "readOnlyRootFilesystem", skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,
    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(rename = "runAsGroup", skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,
    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(rename = "runAsNonRoot", skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,
    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(rename = "runAsUser", skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    #[serde(rename = "seLinuxOptions", skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<crate::models::ScaledJobJobTargetRefTemplateSpecSecurityContextSeLinuxOptions>,
    #[serde(rename = "windowsOptions", skip_serializing_if = "Option::is_none")]
    pub windows_options: Option<crate::models::ScaledJobJobTargetRefTemplateSpecSecurityContextWindowsOptions>,
}

impl ScaledJobJobTargetRefTemplateSpecSecurityContext1 {
    /// SecurityContext is not allowed for ephemeral containers.
    pub fn new() -> ScaledJobJobTargetRefTemplateSpecSecurityContext1 {
        ScaledJobJobTargetRefTemplateSpecSecurityContext1 {
            allow_privilege_escalation: None,
            capabilities: None,
            privileged: None,
            proc_mount: None,
            read_only_root_filesystem: None,
            run_as_group: None,
            run_as_non_root: None,
            run_as_user: None,
            se_linux_options: None,
            windows_options: None,
        }
    }
}



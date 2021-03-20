/*
 * scaledjobs.keda.sh
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScaledJobJobTargetRefTemplateSpecContainers : A single application container that you want to run within a pod.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct ScaledJobJobTargetRefTemplateSpecContainers {
    /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(rename = "args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// List of environment variables to set in the container. Cannot be updated.
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<crate::models::ScaledJobJobTargetRefTemplateSpecEnv>>,
    /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
    #[serde(rename = "envFrom", skip_serializing_if = "Option::is_none")]
    pub env_from: Option<Vec<crate::models::ScaledJobJobTargetRefTemplateSpecEnvFrom>>,
    /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    #[serde(rename = "imagePullPolicy", skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,
    #[serde(rename = "lifecycle", skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<crate::models::ScaledJobJobTargetRefTemplateSpecLifecycle>,
    #[serde(rename = "livenessProbe", skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<crate::models::ScaledJobJobTargetRefTemplateSpecLivenessProbe>,
    /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
    #[serde(rename = "name")]
    pub name: String,
    /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default \"0.0.0.0\" address inside a container will be accessible from the network. Cannot be updated.
    #[serde(rename = "ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<crate::models::ScaledJobJobTargetRefTemplateSpecPorts>>,
    #[serde(rename = "readinessProbe", skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<crate::models::ScaledJobJobTargetRefTemplateSpecReadinessProbe>,
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<crate::models::ScaledJobJobTargetRefTemplateSpecResources>,
    #[serde(rename = "securityContext", skip_serializing_if = "Option::is_none")]
    pub security_context: Option<crate::models::ScaledJobJobTargetRefTemplateSpecSecurityContext>,
    #[serde(rename = "startupProbe", skip_serializing_if = "Option::is_none")]
    pub startup_probe: Option<crate::models::ScaledJobJobTargetRefTemplateSpecStartupProbe>,
    /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
    #[serde(rename = "stdin", skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
    #[serde(rename = "stdinOnce", skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
    #[serde(rename = "terminationMessagePath", skip_serializing_if = "Option::is_none")]
    pub termination_message_path: Option<String>,
    /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
    #[serde(rename = "terminationMessagePolicy", skip_serializing_if = "Option::is_none")]
    pub termination_message_policy: Option<String>,
    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
    #[serde(rename = "tty", skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// volumeDevices is the list of block devices to be used by the container.
    #[serde(rename = "volumeDevices", skip_serializing_if = "Option::is_none")]
    pub volume_devices: Option<Vec<crate::models::ScaledJobJobTargetRefTemplateSpecVolumeDevices>>,
    /// Pod volumes to mount into the container's filesystem. Cannot be updated.
    #[serde(rename = "volumeMounts", skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<crate::models::ScaledJobJobTargetRefTemplateSpecVolumeMounts>>,
    /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
    #[serde(rename = "workingDir", skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

impl ScaledJobJobTargetRefTemplateSpecContainers {
    /// A single application container that you want to run within a pod.
    pub fn new(name: String) -> ScaledJobJobTargetRefTemplateSpecContainers {
        ScaledJobJobTargetRefTemplateSpecContainers {
            args: None,
            command: None,
            env: None,
            env_from: None,
            image: None,
            image_pull_policy: None,
            lifecycle: None,
            liveness_probe: None,
            name,
            ports: None,
            readiness_probe: None,
            resources: None,
            security_context: None,
            startup_probe: None,
            stdin: None,
            stdin_once: None,
            termination_message_path: None,
            termination_message_policy: None,
            tty: None,
            volume_devices: None,
            volume_mounts: None,
            working_dir: None,
        }
    }
}


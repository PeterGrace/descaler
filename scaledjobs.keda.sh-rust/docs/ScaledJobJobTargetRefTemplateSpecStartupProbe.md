# ScaledJobJobTargetRefTemplateSpecStartupProbe

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exec** | Option<[**crate::models::ScaledJobJobTargetRefTemplateSpecLifecyclePostStartExec**](ScaledJob_jobTargetRef_template_spec_lifecycle_postStart_exec.md)> |  | [optional]
**failure_threshold** | Option<**i32**> | Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1. | [optional]
**http_get** | Option<[**crate::models::ScaledJobJobTargetRefTemplateSpecLifecyclePostStartHttpGet**](ScaledJob_jobTargetRef_template_spec_lifecycle_postStart_httpGet.md)> |  | [optional]
**initial_delay_seconds** | Option<**i32**> | Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes | [optional]
**period_seconds** | Option<**i32**> | How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1. | [optional]
**success_threshold** | Option<**i32**> | Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1. | [optional]
**tcp_socket** | Option<[**crate::models::ScaledJobJobTargetRefTemplateSpecLifecyclePostStartTcpSocket**](ScaledJob_jobTargetRef_template_spec_lifecycle_postStart_tcpSocket.md)> |  | [optional]
**timeout_seconds** | Option<**i32**> | Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



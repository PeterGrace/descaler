# ScaledJobJobTargetRefTemplateSpecLifecyclePostStartHttpGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | Option<**String**> | Host name to connect to, defaults to the pod IP. You probably want to set \"Host\" in httpHeaders instead. | [optional]
**http_headers** | Option<[**Vec<crate::models::ScaledJobJobTargetRefTemplateSpecLifecyclePostStartHttpGetHttpHeaders>**](ScaledJob_jobTargetRef_template_spec_lifecycle_postStart_httpGet_httpHeaders.md)> | Custom headers to set in the request. HTTP allows repeated headers. | [optional]
**path** | Option<**String**> | Path to access on the HTTP server. | [optional]
**port** | [**crate::models::AnyOfintegerstring**](anyOf<integer,string>.md) | Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. | 
**scheme** | Option<**String**> | Scheme to use for connecting to the host. Defaults to HTTP. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



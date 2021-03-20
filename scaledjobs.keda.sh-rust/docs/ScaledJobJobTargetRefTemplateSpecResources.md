# ScaledJobJobTargetRefTemplateSpecResources

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**limits** | Option<[**::std::collections::HashMap<String, crate::models::AnyOfintegerstring>**](anyOf<integer,string>.md)> | Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/ | [optional]
**requests** | Option<[**::std::collections::HashMap<String, crate::models::AnyOfintegerstring>**](anyOf<integer,string>.md)> | Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/ | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



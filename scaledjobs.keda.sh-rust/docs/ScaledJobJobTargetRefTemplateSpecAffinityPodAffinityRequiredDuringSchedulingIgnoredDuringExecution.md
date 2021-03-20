# ScaledJobJobTargetRefTemplateSpecAffinityPodAffinityRequiredDuringSchedulingIgnoredDuringExecution

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label_selector** | Option<[**crate::models::ScaledJobJobTargetRefTemplateSpecAffinityPodAffinityPodAffinityTermLabelSelector**](ScaledJob_jobTargetRef_template_spec_affinity_podAffinity_podAffinityTerm_labelSelector.md)> |  | [optional]
**namespaces** | Option<**Vec<String>**> | namespaces specifies which namespaces the labelSelector applies to (matches against); null or empty list means \"this pod's namespace\" | [optional]
**topology_key** | **String** | This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



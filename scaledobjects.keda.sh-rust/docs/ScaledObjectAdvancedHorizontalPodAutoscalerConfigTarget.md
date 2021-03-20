# ScaledObjectAdvancedHorizontalPodAutoscalerConfigTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**average_utilization** | Option<**i32**> | averageUtilization is the target value of the average of the resource metric across all relevant pods, represented as a percentage of the requested value of the resource for the pods. Currently only valid for Resource metric source type | [optional]
**average_value** | Option<[**crate::models::AnyOfintegerstring**](anyOf<integer,string>.md)> | averageValue is the target value of the average of the metric across all relevant pods (as a quantity) | [optional]
**_type** | **String** | type represents whether the metric type is Utilization, Value, or AverageValue | 
**value** | Option<[**crate::models::AnyOfintegerstring**](anyOf<integer,string>.md)> | value is the target value of the metric (as a quantity). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



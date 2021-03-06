# ScaledJobJobTargetRefTemplateSpecConfigMap

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_mode** | Option<**i32**> | Optional: mode bits to use on created files by default. Must be a value between 0 and 0777. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set. | [optional]
**items** | Option<[**Vec<crate::models::ScaledJobJobTargetRefTemplateSpecConfigMapItems>**](ScaledJob_jobTargetRef_template_spec_configMap_items.md)> | If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'. | [optional]
**name** | Option<**String**> | Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid? | [optional]
**optional** | Option<**bool**> | Specify whether the ConfigMap or its keys must be defined | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



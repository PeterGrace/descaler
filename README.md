# descaler
## What it is
descaler is a Kubernetes controller which, based on a signal that it pulls from a specific location, will signal to cluster-autoscaler, as well as any horizontal pod autoscalers, that downscaling is prohibited.

## How does it work
The descaler container takes an input configuration file, expected to be named `config.yaml`:
```
---
source_url: 'file:///home/pgrace/repos/rust/descaler/scaler.yaml'
check_interval: 10
enumerate_nodes_secs: 30
enumerate_scalers_secs: 30
```
### Config file 
- Given a specified `source_url`, descaler will check that url every `check_interval` secs and consider the contents therein:
```
---
scalingEnabled: true
ignore: []
```
**NOTE**: ignore is currently not implemented, but is planned.

### Node Management
- Every `enumerate_nodes_secs` it will evaluate what the current status is of the nodes, based on the info gathered from `source_url`.
  - if scalingEnabled is false
    - if scaling is already disabled, the node is left alone
    - if scaling was not previously disabled, it adds the scaling-disabled annotation as well as a marker that descaler set said annotation.
  - if scalingEnabled is true
    - if the node has the descaler annotation that says it previously disabled scaling, it will remove the scaling annotation,
    - it will also clean up the previous annotation it left.

### Scaler Management
- Additionally, every `enumerate_scalers_secs`, descaler will enumerate all hpa objects on the cluster.  Based on the status of scaling, it will:
  - if scalingEnabled is false
    - if scaler is an hpa object without an ownerRef, it will:
       - annotate the hpa object with the "original" minreplicas value,
       - set minReplicas to currentReplicas
       - patch the HPA object.
    - if scaler is an hpa that has a ownerRef of a KEDA ScaledObject, it will instead:
       - annotate the ScaledObject with the origiinal minreplicas value,
       - query the HPA object for its currentReplica number,
       - amend the minReplicaCount for the ScaledObject to match the current replica count.
  - if scalingEnabled is true
    - it will revert the above settings, if they are found.



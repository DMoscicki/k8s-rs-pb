This package is experimental and try to convert answer from k8s-openapi to rust-protobuf in kube-rs.

```
use k8s-rs-pb::api::core::v1::{Node, Pod, PodList, Event};
use k8s_openapi::api::core::v1::{Event as OtherEvent, Node as OtherNode, Pod as OtherPod};

let pod_openapi = OtherPod::default();

let pod_pb: Pod = converter::from_openapi(pod_openapi).unwrap();

assert_eq!(pod_pb.has_metadata(), true);
```
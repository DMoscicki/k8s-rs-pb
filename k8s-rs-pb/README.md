This package is experimental and I tried to convert answer from k8s-openapi (kube-rs) to rust-protobuf.

In the future I will add Serialize to custom_date.
At this time i only need the Deserialize (i don't like to work with JSON).

```
use k8s-rs-pb::api::core::v1::{Node, Pod, PodList, Event};
use k8s_openapi::api::core::v1::{Event as OtherEvent, Node as OtherNode, Pod as OtherPod};

let pod_openapi = OtherPod::default();

let pod_pb: Pod = converter::from_openapi(pod_openapi).unwrap();

assert_eq!(pod_pb.has_metadata(), true);
```
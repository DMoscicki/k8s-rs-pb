# Inspired by [k8s-pb](https://github.com/kube-rs/k8s-pb)

This package is experimental and I tried to convert answer from k8s-openapi (kube-rs) to rust-protobuf.

In the future I will add Serialize to custom_date.
At this time i only need the Deserialize (i don't like to work with JSON).

## Example

```
    use k8s_rs_pb::api::core::v1::Pod;
    use k8s_openapi::api::core::v1::Pod as OtherPod;

    let pod_openapi = OtherPod::default();
    let pod_pb: Pod = k8s_rs_pb::converter::from_openapi(pod_openapi).unwrap();

    assert_eq!(pod_pb.has_metadata(), true);
```

### Build Dependencies

- [fd](https://github.com/sharkdp/fd)
- [jq](https://stedolan.github.io/jq/)
- [just](https://github.com/casey/just)
- [sd](https://github.com/chmln/sd)
- [rust](https://www.rust-lang.org/)

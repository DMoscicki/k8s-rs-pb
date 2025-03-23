# k8s-rs-pb

**Inspired by [k8s-pb](https://github.com/kube-rs/k8s-pb)**

This experimental package aims to bridge the gap between Kubernetes OpenAPI objects and Rust's Protobuf ecosystem. It provides seamless conversion utilities to transform Kubernetes objects from `k8s-openapi` (used in [kube-rs](https://github.com/kube-rs/kube-rs)) into Rust-Protobuf representations, and vice versa.

This tool was born out of necessity for [kubesman](https://github.com/DMoscicki/kubesman), where describing each Kubernetes object in JSON for frontend consumption became a cumbersome task. With `k8s-rs-pb`, you can generate Protobuf definitions tailored to your specific Kubernetes version, making it easier to work with Kubernetes objects in a type-safe and efficient manner.

---

## Key Features

- **Seamless Conversion**: Convert Kubernetes objects between `k8s-openapi` and Protobuf formats effortlessly.
- **Customizable Protobuf Generation**: Generate Protobuf definitions for your specific Kubernetes version by tweaking the `KUBERNETES_VERSION` variable in the `justfile`.
- **Lightweight and Experimental**: Designed as a lightweight utility to simplify Kubernetes object handling in Rust projects.

---

## Usage Examples

### Convert from `k8s-openapi` to Protobuf

```rust
use k8s_rs_pb::api::core::v1::Pod;
use k8s_openapi::api::core::v1::Pod as OtherPod;

let pod_openapi = OtherPod::default();
let pod_pb: Pod = k8s_rs_pb::converter::from_openapi(pod_openapi).unwrap();

assert_eq!(pod_pb.has_metadata(), true);
```

### Convert from Protobuf to `k8s-openapi`

```rust
use k8s_rs_pb::api::core::v1::Pod;
use k8s_openapi::api::core::v1::Pod as OtherPod;

let pod_pb = Pod::default();
let pod_openapi: OtherPod = k8s_rs_pb::converter::to_openapi(pod_pb).unwrap();
```

## Build Dependencies

To use this package, ensure you have the following tools installed:

- **[fd](https://github.com/sharkdp/fd)**: A simple, fast, and user-friendly alternative to `find`.
- **[jq](https://stedolan.github.io/jq/)**: A lightweight and flexible command-line JSON processor.
- **[just](https://github.com/casey/just)**: A handy command runner for project-specific tasks.
- **[sd](https://github.com/chmln/sd)**: An intuitive find-and-replace CLI tool.
- **[Rust](https://www.rust-lang.org/)**: The Rust programming language and its ecosystem.

---

## Why Use This Package?

If you're working on a project that involves Kubernetes and requires efficient serialization/deserialization of Kubernetes objects, `k8s-rs-pb` can significantly streamline your workflow. By leveraging Protobuf, you can achieve faster serialization, smaller payloads, and better compatibility with frontend or cross-language systems.

---

## Contributing

This project is experimental and open to contributions! If you encounter any issues or have ideas for improvements, feel free to open an issue or submit a pull request. Your feedback is invaluable in shaping the future of this tool.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Made with ❤️ for Kubernetes enthusiasts and Rustaceans alike!

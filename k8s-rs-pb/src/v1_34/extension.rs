use protobuf::{Message, Result};

use super::apimachinery::pkg::runtime::Unknown;

static MAGIC_BYTES: [u8;4] = [0x6b, 0x38, 0x73, 0x00];

/*
  A four byte magic number prefix:
  Bytes 0-3: "k8s\x00" [0x6b, 0x38, 0x73, 0x00]

  An encoded Protobuf message with the following IDL:
  message Unknown {
    // typeMeta should have the string values for "kind" and "apiVersion" as set on the JSON object
    optional TypeMeta typeMeta = 1;

    // raw will hold the complete serialized object in protobuf. See the protobuf definitions in the client libraries for a given kind.
    optional bytes raw = 2;

    // contentEncoding is encoding used for the raw data. Unspecified means no encoding.
    optional string contentEncoding = 3;

    // contentType is the serialization method used to serialize 'raw'. Unspecified means application/vnd.kubernetes.protobuf and is usually
    // omitted.
    optional string contentType = 4;
  }

  message TypeMeta {
    // apiVersion is the group/version for this type
    optional string apiVersion = 1;
    // kind is the name of the object schema. A protobuf definition should exist for this object.
    optional string kind = 2;
  }
*/

/* 
### Parse Protobuf from kube-apiserver
```rust
    use k8s_rs_pb::extension::KubeAPIServerMessage;
    use k8s_rs_pb::api::core::v1::Pod;

    // Handles the "k8s\x00" magic prefix and Unknown wrapper
    let pod = Pod::parse_from_apiserver_bytes(&raw_apiserver_response).unwrap();
```
*/
pub trait KubeAPIServerMessage: Message {
    fn parse_from_apiserver_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes[0..4] == MAGIC_BYTES {
            let unk = Unknown::parse_from_bytes(&bytes[4..])?;
            Ok(Message::parse_from_bytes(unk.raw()).unwrap())
        } else {
            let e = std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid tag in proto");
            Err(e.into())
        }
    }
}

impl <T: Message> KubeAPIServerMessage for T {}
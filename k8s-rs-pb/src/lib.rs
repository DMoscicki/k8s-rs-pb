pub mod api;
pub mod apiextensions_apiserver;
pub mod apimachinery;
pub mod kube_aggregator;
pub mod metrics;

use serde::{Deserialize, Serialize};
use protobuf::MessageField;

#[derive(Serialize, Deserialize)]
#[serde(remote = "MessageField")]
pub struct MessageFieldDef<T>(pub Option<Box<T>>); // dont touch


pub mod custom_date {
    use serde::{Deserialize, Deserializer};
    use super::apimachinery::pkg::apis::meta::v1::Time as TimePb;
    use chrono::DateTime;
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<::protobuf::MessageField<TimePb>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let time_as_str = String::deserialize(deserializer).unwrap_or_default();
        match DateTime::parse_from_rfc3339(&time_as_str) {
            Ok(val) => {
                let secs = val.timestamp();
                let nanos = val.timestamp_subsec_nanos();

                let mut new_time = TimePb::new();

                new_time.set_seconds(secs);
                new_time.set_nanos(nanos as i32);
            
                let fiex = ::protobuf::MessageField::some(new_time);
                Ok(fiex)
            },
            Err(err) => {
                eprintln!("Parse error: {}", err);

                let def_time = TimePb::default();

                Ok(protobuf::MessageField::some(def_time))
            },
        }
    }
}

pub mod intorstr {
    use std::fmt;
    use protobuf::MessageField;
    use serde::{de::Visitor, Deserializer};
    use super::apimachinery::pkg::util::intstr::IntOrString;
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<::protobuf::MessageField<IntOrString>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntOrStringVisitor;

        impl <'de> Visitor<'de> for IntOrStringVisitor {

            type Value = MessageField<IntOrString>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("int or string")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
            {
                let mut int_or_str = IntOrString::new();
                int_or_str.set_type(v);

                let mes = MessageField::some(int_or_str);

                Ok(mes)  
                
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
            {
                let mut int_or_str = IntOrString::new();
                int_or_str.set_intVal(v);

                let mes = MessageField::some(int_or_str);

                Ok(mes)  
            }

            // GitHub issue https://github.com/serde-rs/serde/issues/1162#issuecomment-367955753
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
            {
                let mut int_or_str = IntOrString::new();
                int_or_str.set_type(v as i64);

                let mes = MessageField::some(int_or_str);

                Ok(mes)        
            }
            
            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
            {
                let mut int_or_str = IntOrString::new();
                int_or_str.set_intVal(v as i32);

                let mes = MessageField::some(int_or_str);

                Ok(mes)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, 
            {
                let mut int_or_str = IntOrString::new();
                int_or_str.set_strVal(v.to_owned());
        
                let mes = MessageField::some(int_or_str);
        
                Ok(mes)
            }
        }

        deserializer.deserialize_any(IntOrStringVisitor)
    }
}

pub mod quantity_parse {
    use std::collections::BTreeMap;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::apimachinery::pkg::api::resource::Quantity;
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<::std::collections::BTreeMap<::std::string::String, crate::apimachinery::pkg::api::resource::Quantity>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map_iter: BTreeMap<String, String> = BTreeMap::deserialize(deserializer).unwrap_or_default();
        let mut new_map: BTreeMap<String, crate::apimachinery::pkg::api::resource::Quantity> = BTreeMap::new();

        if map_iter.len() > 0 {
            for (k, v) in map_iter.iter() {
                let mut quantity = Quantity::new();
                quantity.set_string(v.to_string());
                new_map.insert(k.to_string(), quantity);
            }

            Ok(new_map)
        } else {
            Ok(new_map)
        }
    }

    impl Serialize for Quantity {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer {
                serializer.serialize_newtype_struct("Quantity", &self.string.as_ref().unwrap().to_string())
        }
    }

    impl <'de> Deserialize<'de> for Quantity {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de> {
                    let mut quantity = Quantity::new();
                    let inc_value = String::deserialize(deserializer).unwrap_or_default();
                    quantity.set_string(convert_incoming_quantity(&inc_value));

                    Ok(quantity)
        }
    }

    fn convert_incoming_quantity(value: &str) -> String {
        if value.contains("Ki") {
            let (v, _) = value.split_once("Ki").unwrap();
            format!("{:.1}MB", kib_to_mb(v.parse::<f64>().unwrap()))
        } else if value.contains("Mi") {
            let (v, _) = value.split_once("Mi").unwrap();
            format!("{:.1}MB", mib_to_mb(v.parse::<f64>().unwrap()))
        } else if value.contains("Gi") {
            let (v, _) = value.split_once("Gi").unwrap();
            format!("{:.1}GB", gib_to_gb(v.parse::<f64>().unwrap()))
        } else if value.contains("n") {
            let (v, _) = value.split_once("n").unwrap();
            format!("{:.5}", from_nano_cpu(v.parse::<f64>().unwrap()))
        } else if value.contains("m") {
            let (v, _) = value.split_once("m").unwrap();
            format!("{:.5}", from_mega_cpu(v.parse::<f64>().unwrap()))
        } else if value.contains("u") {
            let (v, _) = value.split_once("u").unwrap();
            format!("{:.5}", from_mega_cpu(v.parse::<f64>().unwrap()))
        } else {
            value.parse::<f64>().unwrap().to_string()
        }
    }

    fn from_nano_cpu(v: f64) -> f64 {
        v / 1_000_000_000.0
    }
    
    fn from_mega_cpu(v: f64) -> f64 {
        v / 1_000_000.0
    }

    fn kib_to_mb(val: f64) -> f64 {
        (val * 1024.0) / 1_000_000.0
    }
    
    fn mib_to_mb(val: f64) -> f64 {
        // 1 MiB = 1.048576 MB
        val * 1.048576
    }
    
    fn gib_to_gb(val: f64) -> f64 {
        // 1 GiB = 1.073741824 GB
        val * 1.073741824
    }
}

/// Converter pub
pub mod converter {
    use std::io;

    use serde::{de::DeserializeOwned, Serialize};

    /// Function for convertion from k8s_openapi to rust-protobuf
    /// # Example
    /// 
    /// ``` 
    /// use k8s_rs_pb::api::core::v1::Pod;
    /// use k8s_openapi::api::core::v1::Pod as OtherPod;
    /// 
    /// let pod_openapi = OtherPod::default();
    /// let pod_pb: Pod = k8s_rs_pb::converter::from_openapi(pod_openapi).unwrap();
    /// assert_eq!(pod_pb.has_metadata(), true);
    /// ```
    pub fn from_openapi<P, T>(val: T) -> Result<P, io::Error>
    where 
        T: Serialize,
        P: DeserializeOwned
    {
        let to_val_pb = serde_json::to_value(val)?;

        let pb_value: P = serde_json::from_value(to_val_pb)?;

        Ok(pb_value)
    }

    // pub fn to_openapi<T, P>(val: P) -> Result<T, io::Error> 
    // where 
    //     T: DeserializeOwned,
    //     P: Serialize
    // {
    //     let val_pb = serde_json::to_value(val).unwrap();

    //     let openapi_value: T = serde_json::from_value(val_pb).unwrap();

    //     Ok(openapi_value)
    // }
}


#[cfg(test)]
mod tests {
    use std::fs;

    use api::{apps::v1::DeploymentList, core::v1::{Event, Node, Pod, PodList}};
    use k8s_openapi::api::core::v1::{Event as OtherEvent, Node as OtherNode, Pod as OtherPod};

    use crate::metrics::pkg::apis::metrics::v1alpha1::PodMetrics;

    use super::*;

    #[test]
    fn succesfully_des() {
        let pod_tt = OtherPod::default();

        let pod_x: Pod = converter::from_openapi(pod_tt).unwrap();

        assert_eq!(pod_x.has_metadata(), true);

        let node_tt = OtherNode::default();

        let node_x: Node = converter::from_openapi(node_tt).unwrap();

        assert_eq!(node_x.has_metadata(), true);

        // let dpl = ObjectList<Deployment>;
    }

    #[test]
    fn succes_metrics() {
        let pod = r#"{
            "kind": "PodMetrics",
            "apiVersion": "metrics.k8s.io/v1beta1",
            "metadata": {
              "name": "metrics-server-6db985556d-nqbdz",
              "namespace": "kube-system",
              "creationTimestamp": "2022-10-09T11:51:23Z",
              "labels": {
                "k8s-app": "metrics-server",
                "pod-template-hash": "6db985556d"
              }
            },
            "timestamp": "2022-10-09T11:51:20Z",
            "containers": [
              {
                "name": "metrics-server",
                "usage": {
                  "cpu": "1611127n",
                  "memory": "45320Ki"
                }
              }
            ]
          }"#;

        let x: PodMetrics = serde_json::from_str(&pod).unwrap();
        

        println!("{:#?}", x.containers)
        
    }

    #[test]
    #[should_panic]
    fn bad_des() {
        let asadsd = String::from("ASDASDASD");

        let _px: PodList = converter::from_openapi(asadsd).unwrap();
    }

    #[test]
    fn succesfully_des_with_serde_from_str() {
        let pd_list = fs::read_to_string("testdata/podlist.json").unwrap();

        let mut pb_pd_list: PodList = serde_json::from_str(&pd_list).unwrap();

        let items = pb_pd_list.take_items();

        assert_eq!(pb_pd_list.has_metadata(), true);
        assert_eq!(items.is_empty(), false);

        let pd = fs::read_to_string("testdata/pod.json").unwrap();

        let pb_pd: Pod = serde_json::from_str(&pd).unwrap();

        assert_eq!(pb_pd.has_metadata(), true);
        assert_eq!(pb_pd.has_spec(), true);
        assert_eq!(pb_pd.has_status(), true);

        //close bug with MessageField<IntOrString>
        let dp_list = fs::read_to_string("testdata/deploy.json").unwrap();

        let x: DeploymentList = serde_json::from_str(&dp_list).unwrap();

        println!{"{:#?}", x};

        assert_eq!(x.has_metadata(), true);
    }

    #[test]
    fn event_des() {
        let open_event = OtherEvent::default();

        let pb_event: Event = converter::from_openapi(open_event).unwrap();

        assert_eq!(pb_event.has_metadata(), true);
    }
}
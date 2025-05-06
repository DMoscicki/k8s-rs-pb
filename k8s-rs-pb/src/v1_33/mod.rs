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
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use super::apimachinery::pkg::apis::meta::v1::Duration;

    use super::apimachinery::pkg::apis::meta::v1::Time as TimePb;
    use chrono::DateTime;

    impl Serialize for Duration {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer {
            serializer.serialize_i64(self.duration())
        }
    }

    impl <'de> Deserialize <'de> for Duration {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de> {
            let value = String::deserialize(deserializer).unwrap();

            let (d, _) = value.split_once("s").unwrap();

            match d.parse::<i64>() {
                Ok(val) => {
                    let mut dur = Duration::new();

                    dur.set_duration(val);
        
                    Ok(dur)
                },
                Err(e) => {
                    if e.to_string().contains("invalid digit") {
                        let mut dur = Duration::new();

                        let val = d.parse::<f64>().unwrap();

                        dur.set_duration(val as i64);
            
                        return Ok(dur);
                    }
                    Err(serde::de::Error::custom(e.to_string()))
                }
            }
        }
    }

    impl Serialize for TimePb {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
            where S: Serializer {
            let value = DateTime::from_timestamp(self.seconds(), self.nanos() as u32).unwrap();

            serializer.serialize_str(&value.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
        }
    }

    impl <'de> Deserialize <'de> for TimePb {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de> {
                let time_as_str = String::deserialize(deserializer).unwrap_or_default();
                match DateTime::parse_from_rfc3339(&time_as_str) {
                    Ok(val) => {
                        let secs = val.timestamp();
                        let nanos = val.timestamp_subsec_nanos();

                        let mut new_time = TimePb::new();

                        new_time.set_seconds(secs);
                        new_time.set_nanos(nanos as i32);
                    
                        Ok(new_time)
                    },
                    Err(err) => {
                        eprintln!("parse error: {}", err);

                        let def_time = TimePb::default();

                        Ok(def_time)
                    },
                }
        }
    }
}

pub mod intorstr {
    use std::fmt;
    use serde::{de::Visitor, Deserializer, Serializer};
    use protobuf::MessageField;
    use super::apimachinery::pkg::util::intstr::IntOrString;

    pub fn serialize<S>(value: &MessageField<IntOrString>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value.as_ref() {
            Some(int_or_str) => {
                if let Some(int_val) = int_or_str.intVal.as_ref() {
                    serializer.serialize_i32(*int_val)
                } else if let Some(str_val) = int_or_str.strVal.as_ref() {
                    serializer.serialize_str(str_val)
                } else {
                    serializer.serialize_none()
                }
            }
            None => serializer.serialize_none(),
        }
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<MessageField<IntOrString>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntOrStringVisitor;

        impl<'de> Visitor<'de> for IntOrStringVisitor {
            type Value = MessageField<IntOrString>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("int or string")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let mut int_or_str = IntOrString::new();
                int_or_str.set_intVal(v as i32);
                int_or_str.type_ = Some(0); // Устанавливаем тип для intVal
                Ok(MessageField::some(int_or_str))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_i64(v as i64)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let mut int_or_str = IntOrString::new();
                int_or_str.set_strVal(v.to_owned());
                int_or_str.type_ = Some(1); // Устанавливаем тип для strVal
                Ok(MessageField::some(int_or_str))
            }
        }

        deserializer.deserialize_any(IntOrStringVisitor)
    }
}

pub mod quantity_parse {
    use std::collections::BTreeMap;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::apimachinery::pkg::api::resource::Quantity;
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<::std::collections::BTreeMap<::std::string::String, super::apimachinery::pkg::api::resource::Quantity>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map_iter: BTreeMap<String, String> = BTreeMap::deserialize(deserializer).unwrap_or_default();
        let mut new_map: BTreeMap<String, super::apimachinery::pkg::api::resource::Quantity> = BTreeMap::new();

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
                    let subtype = vec!["u", "m", "n", "Mi", "Ki", "Gi"];

                    let inc_value = String::deserialize(deserializer).unwrap();

                    let mut quantity = Quantity::new();

                    if containe_subtype(&inc_value, subtype) {
                        quantity.set_string(convert_incoming_quantity(&inc_value));
                        Ok(quantity)
                    } else {
                        quantity.set_string(inc_value);
                        Ok(quantity)
                    }
        }
    }

    fn containe_subtype(value: &str, subtypes: Vec<&str>) -> bool {
        subtypes.iter().any(|&substring| value.contains(substring))
    }

    fn convert_incoming_quantity(value: &str) -> String {
        if value.contains("Ki") {
            let (v, _) = value.split_once("Ki").unwrap();
            return format!("{:.1}MB", kib_to_mb(v.parse::<f64>().unwrap()))
        }
        if value.contains("Mi") {
            let (v, _) = value.split_once("Mi").unwrap();
            return format!("{:.1}MB", mib_to_mb(v.parse::<f64>().unwrap()))
        }
        if value.contains("Gi") {
            let (v, _) = value.split_once("Gi").unwrap();
            return format!("{:.1}GB", gib_to_gb(v.parse::<f64>().unwrap()))
        } 
        if value.contains("n") {
            let (v, _) = value.split_once("n").unwrap();
            return format!("{:.5}", from_nano_cpu(v.parse::<f64>().unwrap()))
        }
        if value.contains("m") {
            let (v, _) = value.split_once("m").unwrap();
            return format!("{:.5}", from_micro_cpu(v.parse::<f64>().unwrap()))
        }
        if value.contains("u") {
            let (v, _) = value.split_once("u").unwrap();
            return format!("{:.5}", from_milli_cpu(v.parse::<f64>().unwrap()))
        }

        value.parse::<f64>().unwrap().to_string()
    }

    fn from_nano_cpu(v: f64) -> f64 {
        v / 1_000_000_000.0
    }
    
    fn from_micro_cpu(v: f64) -> f64 {
        v / 1_000_000.0
    }

    fn from_milli_cpu(v: f64) -> f64 {
        v / 1_000.0
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
    /// use k8s_rs_pb::v1_33::api::core::v1::Pod;
    /// use k8s_openapi::api::core::v1::Pod as OtherPod;
    /// 
    /// let pod_openapi = OtherPod::default();
    /// let pod_pb: Pod = k8s_rs_pb::v1_33::converter::from_openapi(pod_openapi).unwrap();
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

        /// Function for convertion from k8s_openapi to rust-protobuf
    /// # Example
    /// 
    /// ``` 
    /// use k8s_rs_pb::v1_33::api::core::v1::Pod;
    /// use k8s_openapi::api::core::v1::Pod as OtherPod;
    /// 
    /// let pod_pb = Pod::default();
    /// let pod_openapi: OtherPod = k8s_rs_pb::v1_33::converter::to_openapi(pod_pb).unwrap();
    /// 
    pub fn to_openapi<T, P>(val: P) -> Result<T, io::Error> 
    where 
        T: DeserializeOwned,
        P: Serialize
    {
        let val_pb = serde_json::to_value(val).unwrap();

        let openapi_value: T = serde_json::from_value(val_pb).unwrap();

        Ok(openapi_value)
    }
}


#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, fs};

    use api::{apps::v1::DeploymentList, core::v1::{Event, Node, Pod, PodList}};
    use k8s_openapi::api::{core::v1::{Event as OtherEvent, Node as OtherNode, Pod as OtherPod}, apps::v1::Deployment as OtherDeployment};
    use kube::api::{ObjectList, TypeMeta};
    use super::{apimachinery::pkg::api::resource::Quantity, metrics::pkg::apis::metrics::v1beta1::{NodeMetrics, PodMetrics}};
    use super::apimachinery::pkg::apis::meta::v1::{Duration, Time as TimePb};

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
    fn succesfully_ser() {
        let name = String::from("Name");

        let mut pod_tt = OtherPod::default();

        pod_tt.metadata.name = Some(name.clone());

        let pod_x: Pod = converter::from_openapi(pod_tt).unwrap();

        assert_eq!(pod_x.has_metadata(), true);

        let node_tt = OtherNode::default();

        let node_x: Node = converter::from_openapi(node_tt).unwrap();

        assert_eq!(node_x.has_metadata(), true);

        let other_pd: OtherPod = converter::to_openapi(pod_x.clone()).unwrap();

        assert_eq!(name, other_pd.metadata.name.unwrap());
        assert_eq!(name.as_str(), pod_x.metadata().name());
    }

    #[test]
    fn success_metrics_node() {
        let node = fs::read_to_string("testdata/node_metrics.json").unwrap();

        let _nd: NodeMetrics = serde_json::from_str(&node).unwrap();
    }

    #[test]
    fn succes_metrics_pod() {
        let pod = fs::read_to_string("testdata/etcd_metrics.json").unwrap();

        let _x: PodMetrics = serde_json::from_str(&pod).unwrap();
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

        let dp_list = fs::read_to_string("testdata/deploy.json").unwrap();

        let x: DeploymentList = serde_json::from_str(&dp_list).unwrap();

        assert_eq!(x.has_metadata(), true);
    }

    #[test]
    fn succesfully_ser_with_serde_from_str() {
        let pd_list = fs::read_to_string("testdata/podlist.json").unwrap();

        let mut pb_pd_list: PodList = serde_json::from_str(&pd_list).unwrap();

        let items = pb_pd_list.take_items();

        assert_eq!(pb_pd_list.has_metadata(), true);
        assert_eq!(items.is_empty(), false);

        let mut pd_ls: ObjectList<OtherPod> = ObjectList{
            types: TypeMeta::default(),
            metadata: kube::api::ListMeta::default(),
            items: Vec::new(),
        };

        for v in items.iter() {
            let new_pod: OtherPod = converter::to_openapi(v).unwrap();
            assert_eq!(v.metadata.name(), new_pod.metadata.name.clone().unwrap().as_str());

            pd_ls.items.push(new_pod);
        }

        assert_eq!(items.len(), pd_ls.items.len());

        pd_ls.items.clear();

        let pd = fs::read_to_string("testdata/pod.json").unwrap();

        let pb_pd: Pod = serde_json::from_str(&pd).unwrap();

        assert_eq!(pb_pd.has_metadata(), true);
        assert_eq!(pb_pd.has_spec(), true);
        assert_eq!(pb_pd.has_status(), true);

        let pd_api: OtherPod = converter::to_openapi(pb_pd.clone()).unwrap();

        assert_eq!(pb_pd.metadata.name(), pd_api.metadata.name.clone().unwrap());

        let dp_list = fs::read_to_string("testdata/deploy.json").unwrap();

        let depl_list: DeploymentList = serde_json::from_str(&dp_list).unwrap();

        let mut depl_api_ls: ObjectList<OtherDeployment> = ObjectList{
            types: TypeMeta::default(),
            metadata: kube::api::ListMeta::default(),
            items: Vec::new(),
        };

        for v in depl_list.items().iter() {
            let new_deploy: OtherDeployment = converter::to_openapi(v).unwrap();
            assert_eq!(v.metadata.name(), new_deploy.metadata.name.clone().unwrap().as_str());

            depl_api_ls.items.push(new_deploy);
        }

        assert_eq!(depl_list.items().len(), depl_api_ls.items.len());
    }

    #[test]
    fn event_des() {
        let open_event = OtherEvent::default();

        let pb_event: Event = converter::from_openapi(open_event).unwrap();

        assert_eq!(pb_event.has_metadata(), true);
    }

    #[test]
    fn test_duration_serialization() {
        let mut dur = Duration::new();
        dur.set_duration(30);
        let serialized = serde_json::to_string(&dur).unwrap();
        assert_eq!(serialized, "30");
    }

    #[test]
    fn test_duration_deserialization() {
        let json_str = "\"45s\"";
        let deserialized: Duration = serde_json::from_str(json_str).unwrap();
        assert_eq!(deserialized.duration(), 45);

        let invalid_json = "\"1.5s\"";
        let result: Result<Duration, _> = serde_json::from_str(invalid_json);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().duration(), 1);
    }

    #[test]
    fn test_time_pb_round_trip() {
        let time_str = "2024-01-01T00:00:00Z";
        let deserialized: TimePb = serde_json::from_str(&format!("\"{}\"", time_str)).unwrap();
        assert_eq!(deserialized.seconds(), 1704067200);
        assert_eq!(deserialized.nanos(), 0);

        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, format!("\"{}\"", time_str));
    }

    // #[test]
    // fn test_int_or_string_serialization() {
    //     let mut int_or_str = IntOrString::new();
    //     int_or_str.set_intVal(42);
    //     let field = MessageField(Some(Box::new(int_or_str))); // Используйте обертку
    
    //     let serialized = serde_json::to_string(&field).unwrap();
    //     assert_eq!(serialized, "42");
    
    //     let deserialized: MessageFieldDef<IntOrString> = serde_json::from_str(&serialized).unwrap();
    //     assert!(deserialized.0.is_some());
    //     assert_eq!(deserialized.0.unwrap().intVal(), 42);
    // }

    #[test]
    fn test_quantity_map_deserialization() {
        let json_str = r#"{"memory": "512Mi", "cpu": "100m"}"#;
        let deserialized: BTreeMap<String, Quantity> = serde_json::from_str(json_str).unwrap();

        assert_eq!(deserialized["memory"].string(), "536.9MB");
        assert_eq!(deserialized["cpu"].string(), "0.00010");
    }

    #[test]
    fn test_converter_with_service() {
        use k8s_openapi::api::core::v1::Service;
        use api::core::v1::Service as PbService;

        let openapi_service = Service {
            metadata: Default::default(),
            spec: Some(Default::default()),
            status: Some(Default::default()),
        };

        let pb_service: PbService = converter::from_openapi(openapi_service.clone()).unwrap();
        assert!(pb_service.has_metadata());
        assert!(pb_service.has_spec());
        assert!(pb_service.has_status());

        let converted_back: Service = converter::to_openapi(pb_service).unwrap();
        assert_eq!(converted_back.metadata.uid, openapi_service.metadata.uid);
    }
}
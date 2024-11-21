// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by protoc 28.3
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `metrics/pkg/apis/external_metrics/v1beta1/generated.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(::serde::Deserialize)]
#[serde(rename_all = "snake_case")]
// @@protoc_insertion_point(message:metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValue)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ExternalMetricValue {
    // message fields
    // @@protoc_insertion_point(field:metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValue.metricName)
    pub metricName: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    // @@protoc_insertion_point(field:metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValue.metricLabels)
    pub metricLabels: ::std::collections::BTreeMap<::std::string::String, ::std::string::String>,
    #[serde(with = "crate::custom_date")]
    #[serde(default)]
    // @@protoc_insertion_point(field:metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValue.timestamp)
    pub timestamp: ::protobuf::MessageField<crate::apimachinery::pkg::apis::meta::v1::Time>,
    // @@protoc_insertion_point(field:metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValue.window)
    pub window: ::std::option::Option<i64>,
    #[serde(with = "crate::MessageFieldDef")]
    #[serde(default)]
    // @@protoc_insertion_point(field:metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValue.value)
    pub value: ::protobuf::MessageField<crate::apimachinery::pkg::api::resource::Quantity>,
    // special fields
    #[serde(skip)]
    // @@protoc_insertion_point(special_field:metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValue.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ExternalMetricValue {
    fn default() -> &'a ExternalMetricValue {
        <ExternalMetricValue as ::protobuf::Message>::default_instance()
    }
}

impl ExternalMetricValue {
    pub fn new() -> ExternalMetricValue {
        ::std::default::Default::default()
    }

    // optional string metricName = 1;

    pub fn metricName(&self) -> &str {
        match self.metricName.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_metricName(&mut self) {
        self.metricName = ::std::option::Option::None;
    }

    pub fn has_metricName(&self) -> bool {
        self.metricName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metricName(&mut self, v: ::std::string::String) {
        self.metricName = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metricName(&mut self) -> &mut ::std::string::String {
        if self.metricName.is_none() {
            self.metricName = ::std::option::Option::Some(::std::string::String::new());
        }
        self.metricName.as_mut().unwrap()
    }

    // Take field
    pub fn take_metricName(&mut self) -> ::std::string::String {
        self.metricName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // repeated .metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValue.MetricLabelsEntry metricLabels = 2;

    pub fn metricLabels(&self) -> &::std::collections::BTreeMap<::std::string::String, ::std::string::String> {
        &self.metricLabels
    }

    pub fn clear_metricLabels(&mut self) {
        self.metricLabels.clear();
    }

    // Param is passed by value, moved
    pub fn set_metricLabels(&mut self, v: ::std::collections::BTreeMap<::std::string::String, ::std::string::String>) {
        self.metricLabels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metricLabels(&mut self) -> &mut ::std::collections::BTreeMap<::std::string::String, ::std::string::String> {
        &mut self.metricLabels
    }

    // Take field
    pub fn take_metricLabels(&mut self) -> ::std::collections::BTreeMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.metricLabels, ::std::collections::BTreeMap::new())
    }

    // optional .apimachinery.pkg.apis.meta.v1.Time timestamp = 3;

    pub fn timestamp(&self) -> &crate::apimachinery::pkg::apis::meta::v1::Time {
        self.timestamp.as_ref().unwrap_or_else(|| <crate::apimachinery::pkg::apis::meta::v1::Time as ::protobuf::Message>::default_instance())
    }

    pub fn clear_timestamp(&mut self) {
        self.timestamp.clear();
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: crate::apimachinery::pkg::apis::meta::v1::Time) {
        self.timestamp = ::protobuf::MessageField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timestamp(&mut self) -> &mut crate::apimachinery::pkg::apis::meta::v1::Time {
        self.timestamp.mut_or_insert_default()
    }

    // Take field
    pub fn take_timestamp(&mut self) -> crate::apimachinery::pkg::apis::meta::v1::Time {
        self.timestamp.take().unwrap_or_else(|| crate::apimachinery::pkg::apis::meta::v1::Time::new())
    }

    // optional int64 window = 4;

    pub fn window(&self) -> i64 {
        self.window.unwrap_or(0)
    }

    pub fn clear_window(&mut self) {
        self.window = ::std::option::Option::None;
    }

    pub fn has_window(&self) -> bool {
        self.window.is_some()
    }

    // Param is passed by value, moved
    pub fn set_window(&mut self, v: i64) {
        self.window = ::std::option::Option::Some(v);
    }

    // optional .apimachinery.pkg.api.resource.Quantity value = 5;

    pub fn value(&self) -> &crate::apimachinery::pkg::api::resource::Quantity {
        self.value.as_ref().unwrap_or_else(|| <crate::apimachinery::pkg::api::resource::Quantity as ::protobuf::Message>::default_instance())
    }

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: crate::apimachinery::pkg::api::resource::Quantity) {
        self.value = ::protobuf::MessageField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut crate::apimachinery::pkg::api::resource::Quantity {
        self.value.mut_or_insert_default()
    }

    // Take field
    pub fn take_value(&mut self) -> crate::apimachinery::pkg::api::resource::Quantity {
        self.value.take().unwrap_or_else(|| crate::apimachinery::pkg::api::resource::Quantity::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "metricName",
            |m: &ExternalMetricValue| { &m.metricName },
            |m: &mut ExternalMetricValue| { &mut m.metricName },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "metricLabels",
            |m: &ExternalMetricValue| { &m.metricLabels },
            |m: &mut ExternalMetricValue| { &mut m.metricLabels },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, crate::apimachinery::pkg::apis::meta::v1::Time>(
            "timestamp",
            |m: &ExternalMetricValue| { &m.timestamp },
            |m: &mut ExternalMetricValue| { &mut m.timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "window",
            |m: &ExternalMetricValue| { &m.window },
            |m: &mut ExternalMetricValue| { &mut m.window },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, crate::apimachinery::pkg::api::resource::Quantity>(
            "value",
            |m: &ExternalMetricValue| { &m.value },
            |m: &mut ExternalMetricValue| { &mut m.value },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ExternalMetricValue>(
            "ExternalMetricValue",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ExternalMetricValue {
    const NAME: &'static str = "ExternalMetricValue";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.metricName = ::std::option::Option::Some(is.read_string()?);
                },
                18 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            18 => value = is.read_string()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.metricLabels.insert(key, value);
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.timestamp)?;
                },
                32 => {
                    self.window = ::std::option::Option::Some(is.read_int64()?);
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.value)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.metricName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for (k, v) in &self.metricLabels {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if let Some(v) = self.timestamp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.window {
            my_size += ::protobuf::rt::int64_size(4, v);
        }
        if let Some(v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.metricName.as_ref() {
            os.write_string(1, v)?;
        }
        for (k, v) in &self.metricLabels {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            os.write_raw_varint32(18)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.timestamp.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.window {
            os.write_int64(4, v)?;
        }
        if let Some(v) = self.value.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ExternalMetricValue {
        ExternalMetricValue::new()
    }

    fn clear(&mut self) {
        self.metricName = ::std::option::Option::None;
        self.metricLabels.clear();
        self.timestamp.clear();
        self.window = ::std::option::Option::None;
        self.value.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ExternalMetricValue {
        static instance: ::protobuf::rt::Lazy<ExternalMetricValue> = ::protobuf::rt::Lazy::new();
        instance.get(ExternalMetricValue::new)
    }
}

impl ::protobuf::MessageFull for ExternalMetricValue {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ExternalMetricValue").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ExternalMetricValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExternalMetricValue {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(::serde::Deserialize)]
#[serde(rename_all = "snake_case")]
// @@protoc_insertion_point(message:metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValueList)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ExternalMetricValueList {
    // message fields
    #[serde(with = "crate::MessageFieldDef")]
    #[serde(default)]
    // @@protoc_insertion_point(field:metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValueList.metadata)
    pub metadata: ::protobuf::MessageField<crate::apimachinery::pkg::apis::meta::v1::ListMeta>,
    #[serde(default)]
    // @@protoc_insertion_point(field:metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValueList.items)
    pub items: ::std::vec::Vec<ExternalMetricValue>,
    // special fields
    #[serde(skip)]
    // @@protoc_insertion_point(special_field:metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValueList.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ExternalMetricValueList {
    fn default() -> &'a ExternalMetricValueList {
        <ExternalMetricValueList as ::protobuf::Message>::default_instance()
    }
}

impl ExternalMetricValueList {
    pub fn new() -> ExternalMetricValueList {
        ::std::default::Default::default()
    }

    // optional .apimachinery.pkg.apis.meta.v1.ListMeta metadata = 1;

    pub fn metadata(&self) -> &crate::apimachinery::pkg::apis::meta::v1::ListMeta {
        self.metadata.as_ref().unwrap_or_else(|| <crate::apimachinery::pkg::apis::meta::v1::ListMeta as ::protobuf::Message>::default_instance())
    }

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: crate::apimachinery::pkg::apis::meta::v1::ListMeta) {
        self.metadata = ::protobuf::MessageField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut crate::apimachinery::pkg::apis::meta::v1::ListMeta {
        self.metadata.mut_or_insert_default()
    }

    // Take field
    pub fn take_metadata(&mut self) -> crate::apimachinery::pkg::apis::meta::v1::ListMeta {
        self.metadata.take().unwrap_or_else(|| crate::apimachinery::pkg::apis::meta::v1::ListMeta::new())
    }

    // repeated .metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValue items = 2;

    pub fn items(&self) -> &[ExternalMetricValue] {
        &self.items
    }

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::std::vec::Vec<ExternalMetricValue>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::std::vec::Vec<ExternalMetricValue> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::std::vec::Vec<ExternalMetricValue> {
        ::std::mem::replace(&mut self.items, ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, crate::apimachinery::pkg::apis::meta::v1::ListMeta>(
            "metadata",
            |m: &ExternalMetricValueList| { &m.metadata },
            |m: &mut ExternalMetricValueList| { &mut m.metadata },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "items",
            |m: &ExternalMetricValueList| { &m.items },
            |m: &mut ExternalMetricValueList| { &mut m.items },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ExternalMetricValueList>(
            "ExternalMetricValueList",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ExternalMetricValueList {
    const NAME: &'static str = "ExternalMetricValueList";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.metadata)?;
                },
                18 => {
                    self.items.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.metadata.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        for v in &self.items {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ExternalMetricValueList {
        ExternalMetricValueList::new()
    }

    fn clear(&mut self) {
        self.metadata.clear();
        self.items.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ExternalMetricValueList {
        static instance: ExternalMetricValueList = ExternalMetricValueList {
            metadata: ::protobuf::MessageField::none(),
            items: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ExternalMetricValueList {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ExternalMetricValueList").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ExternalMetricValueList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExternalMetricValueList {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n9metrics/pkg/apis/external_metrics/v1beta1/generated.proto\x12)metrics\
    .pkg.apis.external_metrics.v1beta1\x1a-apimachinery/pkg/api/resource/gen\
    erated.proto\x1a-apimachinery/pkg/apis/meta/v1/generated.proto\x1a(apima\
    chinery/pkg/runtime/generated.proto\x1a/apimachinery/pkg/runtime/schema/\
    generated.proto\"\x86\x03\n\x13ExternalMetricValue\x12\x1e\n\nmetricName\
    \x18\x01\x20\x01(\tR\nmetricName\x12t\n\x0cmetricLabels\x18\x02\x20\x03(\
    \x0b2P.metrics.pkg.apis.external_metrics.v1beta1.ExternalMetricValue.Met\
    ricLabelsEntryR\x0cmetricLabels\x12A\n\ttimestamp\x18\x03\x20\x01(\x0b2#\
    .apimachinery.pkg.apis.meta.v1.TimeR\ttimestamp\x12\x16\n\x06window\x18\
    \x04\x20\x01(\x03R\x06window\x12=\n\x05value\x18\x05\x20\x01(\x0b2'.apim\
    achinery.pkg.api.resource.QuantityR\x05value\x1a?\n\x11MetricLabelsEntry\
    \x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\
    \x20\x01(\tR\x05value:\x028\x01\"\xb4\x01\n\x17ExternalMetricValueList\
    \x12C\n\x08metadata\x18\x01\x20\x01(\x0b2'.apimachinery.pkg.apis.meta.v1\
    .ListMetaR\x08metadata\x12T\n\x05items\x18\x02\x20\x03(\x0b2>.metrics.pk\
    g.apis.external_metrics.v1beta1.ExternalMetricValueR\x05itemsB2Z0k8s.io/\
    metrics/pkg/apis/external_metrics/v1beta1\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::v1beta1::file_descriptor().clone());
            deps.push(super::v1beta1::file_descriptor().clone());
            deps.push(super::v1beta1::file_descriptor().clone());
            deps.push(super::v1beta1::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(ExternalMetricValue::generated_message_descriptor_data());
            messages.push(ExternalMetricValueList::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}

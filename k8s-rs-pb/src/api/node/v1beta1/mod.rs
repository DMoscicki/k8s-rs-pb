// This file is generated by rust-protobuf 3.7.2. Do not edit
// .proto file is parsed by protoc 3.21.12
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

//! Generated file from `api/node/v1beta1/generated.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::serde::Deserialize)]
#[serde(rename_all = "snake_case")]
// @@protoc_insertion_point(message:api.node.v1beta1.Overhead)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Overhead {
    // message fields
    #[serde(with = "crate::quantity_parse")]
    #[serde(default)]
    // @@protoc_insertion_point(field:api.node.v1beta1.Overhead.podFixed)
    pub podFixed: ::std::collections::BTreeMap<::std::string::String, crate::apimachinery::pkg::api::resource::Quantity>,
    // special fields
    #[serde(skip)]
    // @@protoc_insertion_point(special_field:api.node.v1beta1.Overhead.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Overhead {
    fn default() -> &'a Overhead {
        <Overhead as ::protobuf::Message>::default_instance()
    }
}

impl Overhead {
    pub fn new() -> Overhead {
        ::std::default::Default::default()
    }

    // repeated .api.node.v1beta1.Overhead.PodFixedEntry podFixed = 1;

    pub fn podFixed(&self) -> &::std::collections::BTreeMap<::std::string::String, crate::apimachinery::pkg::api::resource::Quantity> {
        &self.podFixed
    }

    pub fn clear_podFixed(&mut self) {
        self.podFixed.clear();
    }

    // Param is passed by value, moved
    pub fn set_podFixed(&mut self, v: ::std::collections::BTreeMap<::std::string::String, crate::apimachinery::pkg::api::resource::Quantity>) {
        self.podFixed = v;
    }

    // Mutable pointer to the field.
    pub fn mut_podFixed(&mut self) -> &mut ::std::collections::BTreeMap<::std::string::String, crate::apimachinery::pkg::api::resource::Quantity> {
        &mut self.podFixed
    }

    // Take field
    pub fn take_podFixed(&mut self) -> ::std::collections::BTreeMap<::std::string::String, crate::apimachinery::pkg::api::resource::Quantity> {
        ::std::mem::replace(&mut self.podFixed, ::std::collections::BTreeMap::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "podFixed",
            |m: &Overhead| { &m.podFixed },
            |m: &mut Overhead| { &mut m.podFixed },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Overhead>(
            "Overhead",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Overhead {
    const NAME: &'static str = "Overhead";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.podFixed.insert(key, value);
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
        for (k, v) in &self.podFixed {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.podFixed {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(10)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
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

    fn new() -> Overhead {
        Overhead::new()
    }

    fn clear(&mut self) {
        self.podFixed.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Overhead {
        static instance: ::protobuf::rt::Lazy<Overhead> = ::protobuf::rt::Lazy::new();
        instance.get(Overhead::new)
    }
}

impl ::protobuf::MessageFull for Overhead {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Overhead").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Overhead {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Overhead {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(::serde::Deserialize)]
#[serde(rename_all = "snake_case")]
// @@protoc_insertion_point(message:api.node.v1beta1.RuntimeClass)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RuntimeClass {
    // message fields
    #[serde(with = "crate::MessageFieldDef")]
    #[serde(default)]
    // @@protoc_insertion_point(field:api.node.v1beta1.RuntimeClass.metadata)
    pub metadata: ::protobuf::MessageField<crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    // @@protoc_insertion_point(field:api.node.v1beta1.RuntimeClass.handler)
    pub handler: ::std::option::Option<::std::string::String>,
    #[serde(with = "crate::MessageFieldDef")]
    #[serde(default)]
    // @@protoc_insertion_point(field:api.node.v1beta1.RuntimeClass.overhead)
    pub overhead: ::protobuf::MessageField<Overhead>,
    #[serde(with = "crate::MessageFieldDef")]
    #[serde(default)]
    // @@protoc_insertion_point(field:api.node.v1beta1.RuntimeClass.scheduling)
    pub scheduling: ::protobuf::MessageField<Scheduling>,
    // special fields
    #[serde(skip)]
    // @@protoc_insertion_point(special_field:api.node.v1beta1.RuntimeClass.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RuntimeClass {
    fn default() -> &'a RuntimeClass {
        <RuntimeClass as ::protobuf::Message>::default_instance()
    }
}

impl RuntimeClass {
    pub fn new() -> RuntimeClass {
        ::std::default::Default::default()
    }

    // optional .apimachinery.pkg.apis.meta.v1.ObjectMeta metadata = 1;

    pub fn metadata(&self) -> &crate::apimachinery::pkg::apis::meta::v1::ObjectMeta {
        self.metadata.as_ref().unwrap_or_else(|| <crate::apimachinery::pkg::apis::meta::v1::ObjectMeta as ::protobuf::Message>::default_instance())
    }

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: crate::apimachinery::pkg::apis::meta::v1::ObjectMeta) {
        self.metadata = ::protobuf::MessageField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut crate::apimachinery::pkg::apis::meta::v1::ObjectMeta {
        self.metadata.mut_or_insert_default()
    }

    // Take field
    pub fn take_metadata(&mut self) -> crate::apimachinery::pkg::apis::meta::v1::ObjectMeta {
        self.metadata.take().unwrap_or_else(|| crate::apimachinery::pkg::apis::meta::v1::ObjectMeta::new())
    }

    // optional string handler = 2;

    pub fn handler(&self) -> &str {
        match self.handler.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_handler(&mut self) {
        self.handler = ::std::option::Option::None;
    }

    pub fn has_handler(&self) -> bool {
        self.handler.is_some()
    }

    // Param is passed by value, moved
    pub fn set_handler(&mut self, v: ::std::string::String) {
        self.handler = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_handler(&mut self) -> &mut ::std::string::String {
        if self.handler.is_none() {
            self.handler = ::std::option::Option::Some(::std::string::String::new());
        }
        self.handler.as_mut().unwrap()
    }

    // Take field
    pub fn take_handler(&mut self) -> ::std::string::String {
        self.handler.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional .api.node.v1beta1.Overhead overhead = 3;

    pub fn overhead(&self) -> &Overhead {
        self.overhead.as_ref().unwrap_or_else(|| <Overhead as ::protobuf::Message>::default_instance())
    }

    pub fn clear_overhead(&mut self) {
        self.overhead.clear();
    }

    pub fn has_overhead(&self) -> bool {
        self.overhead.is_some()
    }

    // Param is passed by value, moved
    pub fn set_overhead(&mut self, v: Overhead) {
        self.overhead = ::protobuf::MessageField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_overhead(&mut self) -> &mut Overhead {
        self.overhead.mut_or_insert_default()
    }

    // Take field
    pub fn take_overhead(&mut self) -> Overhead {
        self.overhead.take().unwrap_or_else(|| Overhead::new())
    }

    // optional .api.node.v1beta1.Scheduling scheduling = 4;

    pub fn scheduling(&self) -> &Scheduling {
        self.scheduling.as_ref().unwrap_or_else(|| <Scheduling as ::protobuf::Message>::default_instance())
    }

    pub fn clear_scheduling(&mut self) {
        self.scheduling.clear();
    }

    pub fn has_scheduling(&self) -> bool {
        self.scheduling.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scheduling(&mut self, v: Scheduling) {
        self.scheduling = ::protobuf::MessageField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scheduling(&mut self) -> &mut Scheduling {
        self.scheduling.mut_or_insert_default()
    }

    // Take field
    pub fn take_scheduling(&mut self) -> Scheduling {
        self.scheduling.take().unwrap_or_else(|| Scheduling::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, crate::apimachinery::pkg::apis::meta::v1::ObjectMeta>(
            "metadata",
            |m: &RuntimeClass| { &m.metadata },
            |m: &mut RuntimeClass| { &mut m.metadata },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "handler",
            |m: &RuntimeClass| { &m.handler },
            |m: &mut RuntimeClass| { &mut m.handler },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Overhead>(
            "overhead",
            |m: &RuntimeClass| { &m.overhead },
            |m: &mut RuntimeClass| { &mut m.overhead },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Scheduling>(
            "scheduling",
            |m: &RuntimeClass| { &m.scheduling },
            |m: &mut RuntimeClass| { &mut m.scheduling },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RuntimeClass>(
            "RuntimeClass",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RuntimeClass {
    const NAME: &'static str = "RuntimeClass";

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
                    self.handler = ::std::option::Option::Some(is.read_string()?);
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.overhead)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.scheduling)?;
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
        if let Some(v) = self.handler.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.overhead.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.scheduling.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.metadata.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.handler.as_ref() {
            os.write_string(2, v)?;
        }
        if let Some(v) = self.overhead.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.scheduling.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> RuntimeClass {
        RuntimeClass::new()
    }

    fn clear(&mut self) {
        self.metadata.clear();
        self.handler = ::std::option::Option::None;
        self.overhead.clear();
        self.scheduling.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RuntimeClass {
        static instance: RuntimeClass = RuntimeClass {
            metadata: ::protobuf::MessageField::none(),
            handler: ::std::option::Option::None,
            overhead: ::protobuf::MessageField::none(),
            scheduling: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RuntimeClass {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RuntimeClass").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RuntimeClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RuntimeClass {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(::serde::Deserialize)]
#[serde(rename_all = "snake_case")]
// @@protoc_insertion_point(message:api.node.v1beta1.RuntimeClassList)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RuntimeClassList {
    // message fields
    #[serde(with = "crate::MessageFieldDef")]
    #[serde(default)]
    // @@protoc_insertion_point(field:api.node.v1beta1.RuntimeClassList.metadata)
    pub metadata: ::protobuf::MessageField<crate::apimachinery::pkg::apis::meta::v1::ListMeta>,
    #[serde(default)]
    // @@protoc_insertion_point(field:api.node.v1beta1.RuntimeClassList.items)
    pub items: ::std::vec::Vec<RuntimeClass>,
    // special fields
    #[serde(skip)]
    // @@protoc_insertion_point(special_field:api.node.v1beta1.RuntimeClassList.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RuntimeClassList {
    fn default() -> &'a RuntimeClassList {
        <RuntimeClassList as ::protobuf::Message>::default_instance()
    }
}

impl RuntimeClassList {
    pub fn new() -> RuntimeClassList {
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

    // repeated .api.node.v1beta1.RuntimeClass items = 2;

    pub fn items(&self) -> &[RuntimeClass] {
        &self.items
    }

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::std::vec::Vec<RuntimeClass>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::std::vec::Vec<RuntimeClass> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::std::vec::Vec<RuntimeClass> {
        ::std::mem::replace(&mut self.items, ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, crate::apimachinery::pkg::apis::meta::v1::ListMeta>(
            "metadata",
            |m: &RuntimeClassList| { &m.metadata },
            |m: &mut RuntimeClassList| { &mut m.metadata },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "items",
            |m: &RuntimeClassList| { &m.items },
            |m: &mut RuntimeClassList| { &mut m.items },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RuntimeClassList>(
            "RuntimeClassList",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RuntimeClassList {
    const NAME: &'static str = "RuntimeClassList";

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

    fn new() -> RuntimeClassList {
        RuntimeClassList::new()
    }

    fn clear(&mut self) {
        self.metadata.clear();
        self.items.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RuntimeClassList {
        static instance: RuntimeClassList = RuntimeClassList {
            metadata: ::protobuf::MessageField::none(),
            items: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RuntimeClassList {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RuntimeClassList").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RuntimeClassList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RuntimeClassList {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(::serde::Deserialize)]
#[serde(rename_all = "snake_case")]
// @@protoc_insertion_point(message:api.node.v1beta1.Scheduling)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Scheduling {
    // message fields
    #[serde(default)]
    // @@protoc_insertion_point(field:api.node.v1beta1.Scheduling.nodeSelector)
    pub nodeSelector: ::std::collections::BTreeMap<::std::string::String, ::std::string::String>,
    #[serde(default)]
    // @@protoc_insertion_point(field:api.node.v1beta1.Scheduling.tolerations)
    pub tolerations: ::std::vec::Vec<crate::api::core::v1::Toleration>,
    // special fields
    #[serde(skip)]
    // @@protoc_insertion_point(special_field:api.node.v1beta1.Scheduling.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Scheduling {
    fn default() -> &'a Scheduling {
        <Scheduling as ::protobuf::Message>::default_instance()
    }
}

impl Scheduling {
    pub fn new() -> Scheduling {
        ::std::default::Default::default()
    }

    // repeated .api.node.v1beta1.Scheduling.NodeSelectorEntry nodeSelector = 1;

    pub fn nodeSelector(&self) -> &::std::collections::BTreeMap<::std::string::String, ::std::string::String> {
        &self.nodeSelector
    }

    pub fn clear_nodeSelector(&mut self) {
        self.nodeSelector.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodeSelector(&mut self, v: ::std::collections::BTreeMap<::std::string::String, ::std::string::String>) {
        self.nodeSelector = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodeSelector(&mut self) -> &mut ::std::collections::BTreeMap<::std::string::String, ::std::string::String> {
        &mut self.nodeSelector
    }

    // Take field
    pub fn take_nodeSelector(&mut self) -> ::std::collections::BTreeMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.nodeSelector, ::std::collections::BTreeMap::new())
    }

    // repeated .api.core.v1.Toleration tolerations = 2;

    pub fn tolerations(&self) -> &[crate::api::core::v1::Toleration] {
        &self.tolerations
    }

    pub fn clear_tolerations(&mut self) {
        self.tolerations.clear();
    }

    // Param is passed by value, moved
    pub fn set_tolerations(&mut self, v: ::std::vec::Vec<crate::api::core::v1::Toleration>) {
        self.tolerations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tolerations(&mut self) -> &mut ::std::vec::Vec<crate::api::core::v1::Toleration> {
        &mut self.tolerations
    }

    // Take field
    pub fn take_tolerations(&mut self) -> ::std::vec::Vec<crate::api::core::v1::Toleration> {
        ::std::mem::replace(&mut self.tolerations, ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "nodeSelector",
            |m: &Scheduling| { &m.nodeSelector },
            |m: &mut Scheduling| { &mut m.nodeSelector },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "tolerations",
            |m: &Scheduling| { &m.tolerations },
            |m: &mut Scheduling| { &mut m.tolerations },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Scheduling>(
            "Scheduling",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Scheduling {
    const NAME: &'static str = "Scheduling";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
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
                    self.nodeSelector.insert(key, value);
                },
                18 => {
                    self.tolerations.push(is.read_message()?);
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
        for (k, v) in &self.nodeSelector {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.tolerations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.nodeSelector {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            os.write_raw_varint32(10)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_string(2, &v)?;
        };
        for v in &self.tolerations {
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

    fn new() -> Scheduling {
        Scheduling::new()
    }

    fn clear(&mut self) {
        self.nodeSelector.clear();
        self.tolerations.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Scheduling {
        static instance: ::protobuf::rt::Lazy<Scheduling> = ::protobuf::rt::Lazy::new();
        instance.get(Scheduling::new)
    }
}

impl ::protobuf::MessageFull for Scheduling {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Scheduling").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Scheduling {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Scheduling {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20api/node/v1beta1/generated.proto\x12\x10api.node.v1beta1\x1a\x1bap\
    i/core/v1/generated.proto\x1a-apimachinery/pkg/api/resource/generated.pr\
    oto\x1a-apimachinery/pkg/apis/meta/v1/generated.proto\x1a(apimachinery/p\
    kg/runtime/generated.proto\x1a/apimachinery/pkg/runtime/schema/generated\
    .proto\"\xb6\x01\n\x08Overhead\x12D\n\x08podFixed\x18\x01\x20\x03(\x0b2(\
    .api.node.v1beta1.Overhead.PodFixedEntryR\x08podFixed\x1ad\n\rPodFixedEn\
    try\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12=\n\x05value\x18\x02\
    \x20\x01(\x0b2'.apimachinery.pkg.api.resource.QuantityR\x05value:\x028\
    \x01\"\xe5\x01\n\x0cRuntimeClass\x12E\n\x08metadata\x18\x01\x20\x01(\x0b\
    2).apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x12\x18\n\x07ha\
    ndler\x18\x02\x20\x01(\tR\x07handler\x126\n\x08overhead\x18\x03\x20\x01(\
    \x0b2\x1a.api.node.v1beta1.OverheadR\x08overhead\x12<\n\nscheduling\x18\
    \x04\x20\x01(\x0b2\x1c.api.node.v1beta1.SchedulingR\nscheduling\"\x8d\
    \x01\n\x10RuntimeClassList\x12C\n\x08metadata\x18\x01\x20\x01(\x0b2'.api\
    machinery.pkg.apis.meta.v1.ListMetaR\x08metadata\x124\n\x05items\x18\x02\
    \x20\x03(\x0b2\x1e.api.node.v1beta1.RuntimeClassR\x05items\"\xdc\x01\n\n\
    Scheduling\x12R\n\x0cnodeSelector\x18\x01\x20\x03(\x0b2..api.node.v1beta\
    1.Scheduling.NodeSelectorEntryR\x0cnodeSelector\x129\n\x0btolerations\
    \x18\x02\x20\x03(\x0b2\x17.api.core.v1.TolerationR\x0btolerations\x1a?\n\
    \x11NodeSelectorEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\
    \x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01B\x19Z\x17k8s.io/a\
    pi/node/v1beta1\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::v1beta1::file_descriptor().clone());
            deps.push(super::v1beta1::file_descriptor().clone());
            deps.push(super::v1beta1::file_descriptor().clone());
            deps.push(super::v1beta1::file_descriptor().clone());
            deps.push(super::v1beta1::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(Overhead::generated_message_descriptor_data());
            messages.push(RuntimeClass::generated_message_descriptor_data());
            messages.push(RuntimeClassList::generated_message_descriptor_data());
            messages.push(Scheduling::generated_message_descriptor_data());
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

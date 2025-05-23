// This file is generated by rust-protobuf 3.7.2. Do not edit
// .proto file is parsed by protoc 30.2
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

//! Generated file from `api/scheduling/v1alpha1/generated.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(rename_all = "snake_case")]
// @@protoc_insertion_point(message:api.scheduling.v1alpha1.PriorityClass)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PriorityClass {
    // message fields
    #[serde(with = "crate::v1_30::MessageFieldDef")]
    #[serde(default)]
    // @@protoc_insertion_point(field:api.scheduling.v1alpha1.PriorityClass.metadata)
    pub metadata: ::protobuf::MessageField<crate::v1_30::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    // @@protoc_insertion_point(field:api.scheduling.v1alpha1.PriorityClass.value)
    pub value: ::std::option::Option<i32>,
    // @@protoc_insertion_point(field:api.scheduling.v1alpha1.PriorityClass.globalDefault)
    pub globalDefault: ::std::option::Option<bool>,
    // @@protoc_insertion_point(field:api.scheduling.v1alpha1.PriorityClass.description)
    pub description: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:api.scheduling.v1alpha1.PriorityClass.preemptionPolicy)
    pub preemptionPolicy: ::std::option::Option<::std::string::String>,
    // special fields
    #[serde(skip)]
    // @@protoc_insertion_point(special_field:api.scheduling.v1alpha1.PriorityClass.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PriorityClass {
    fn default() -> &'a PriorityClass {
        <PriorityClass as ::protobuf::Message>::default_instance()
    }
}

impl PriorityClass {
    pub fn new() -> PriorityClass {
        ::std::default::Default::default()
    }

    // optional .apimachinery.pkg.apis.meta.v1.ObjectMeta metadata = 1;

    pub fn metadata(&self) -> &crate::v1_30::apimachinery::pkg::apis::meta::v1::ObjectMeta {
        self.metadata.as_ref().unwrap_or_else(|| <crate::v1_30::apimachinery::pkg::apis::meta::v1::ObjectMeta as ::protobuf::Message>::default_instance())
    }

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: crate::v1_30::apimachinery::pkg::apis::meta::v1::ObjectMeta) {
        self.metadata = ::protobuf::MessageField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut crate::v1_30::apimachinery::pkg::apis::meta::v1::ObjectMeta {
        self.metadata.mut_or_insert_default()
    }

    // Take field
    pub fn take_metadata(&mut self) -> crate::v1_30::apimachinery::pkg::apis::meta::v1::ObjectMeta {
        self.metadata.take().unwrap_or_else(|| crate::v1_30::apimachinery::pkg::apis::meta::v1::ObjectMeta::new())
    }

    // optional int32 value = 2;

    pub fn value(&self) -> i32 {
        self.value.unwrap_or(0)
    }

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i32) {
        self.value = ::std::option::Option::Some(v);
    }

    // optional bool globalDefault = 3;

    pub fn globalDefault(&self) -> bool {
        self.globalDefault.unwrap_or(false)
    }

    pub fn clear_globalDefault(&mut self) {
        self.globalDefault = ::std::option::Option::None;
    }

    pub fn has_globalDefault(&self) -> bool {
        self.globalDefault.is_some()
    }

    // Param is passed by value, moved
    pub fn set_globalDefault(&mut self, v: bool) {
        self.globalDefault = ::std::option::Option::Some(v);
    }

    // optional string description = 4;

    pub fn description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_description(&mut self) {
        self.description = ::std::option::Option::None;
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description = ::std::option::Option::Some(::std::string::String::new());
        }
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string preemptionPolicy = 5;

    pub fn preemptionPolicy(&self) -> &str {
        match self.preemptionPolicy.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_preemptionPolicy(&mut self) {
        self.preemptionPolicy = ::std::option::Option::None;
    }

    pub fn has_preemptionPolicy(&self) -> bool {
        self.preemptionPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_preemptionPolicy(&mut self, v: ::std::string::String) {
        self.preemptionPolicy = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_preemptionPolicy(&mut self) -> &mut ::std::string::String {
        if self.preemptionPolicy.is_none() {
            self.preemptionPolicy = ::std::option::Option::Some(::std::string::String::new());
        }
        self.preemptionPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_preemptionPolicy(&mut self) -> ::std::string::String {
        self.preemptionPolicy.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, crate::v1_30::apimachinery::pkg::apis::meta::v1::ObjectMeta>(
            "metadata",
            |m: &PriorityClass| { &m.metadata },
            |m: &mut PriorityClass| { &mut m.metadata },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "value",
            |m: &PriorityClass| { &m.value },
            |m: &mut PriorityClass| { &mut m.value },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "globalDefault",
            |m: &PriorityClass| { &m.globalDefault },
            |m: &mut PriorityClass| { &mut m.globalDefault },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "description",
            |m: &PriorityClass| { &m.description },
            |m: &mut PriorityClass| { &mut m.description },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "preemptionPolicy",
            |m: &PriorityClass| { &m.preemptionPolicy },
            |m: &mut PriorityClass| { &mut m.preemptionPolicy },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PriorityClass>(
            "PriorityClass",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PriorityClass {
    const NAME: &'static str = "PriorityClass";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.metadata)?;
                },
                16 => {
                    self.value = ::std::option::Option::Some(is.read_int32()?);
                },
                24 => {
                    self.globalDefault = ::std::option::Option::Some(is.read_bool()?);
                },
                34 => {
                    self.description = ::std::option::Option::Some(is.read_string()?);
                },
                42 => {
                    self.preemptionPolicy = ::std::option::Option::Some(is.read_string()?);
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
        if let Some(v) = self.value {
            my_size += ::protobuf::rt::int32_size(2, v);
        }
        if let Some(v) = self.globalDefault {
            my_size += 1 + 1;
        }
        if let Some(v) = self.description.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.preemptionPolicy.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.metadata.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.value {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.globalDefault {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.description.as_ref() {
            os.write_string(4, v)?;
        }
        if let Some(v) = self.preemptionPolicy.as_ref() {
            os.write_string(5, v)?;
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

    fn new() -> PriorityClass {
        PriorityClass::new()
    }

    fn clear(&mut self) {
        self.metadata.clear();
        self.value = ::std::option::Option::None;
        self.globalDefault = ::std::option::Option::None;
        self.description = ::std::option::Option::None;
        self.preemptionPolicy = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PriorityClass {
        static instance: PriorityClass = PriorityClass {
            metadata: ::protobuf::MessageField::none(),
            value: ::std::option::Option::None,
            globalDefault: ::std::option::Option::None,
            description: ::std::option::Option::None,
            preemptionPolicy: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PriorityClass {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PriorityClass").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PriorityClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PriorityClass {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(rename_all = "snake_case")]
// @@protoc_insertion_point(message:api.scheduling.v1alpha1.PriorityClassList)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PriorityClassList {
    // message fields
    #[serde(with = "crate::v1_30::MessageFieldDef")]
    #[serde(default)]
    // @@protoc_insertion_point(field:api.scheduling.v1alpha1.PriorityClassList.metadata)
    pub metadata: ::protobuf::MessageField<crate::v1_30::apimachinery::pkg::apis::meta::v1::ListMeta>,
    #[serde(default)]
    // @@protoc_insertion_point(field:api.scheduling.v1alpha1.PriorityClassList.items)
    pub items: ::std::vec::Vec<PriorityClass>,
    // special fields
    #[serde(skip)]
    // @@protoc_insertion_point(special_field:api.scheduling.v1alpha1.PriorityClassList.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PriorityClassList {
    fn default() -> &'a PriorityClassList {
        <PriorityClassList as ::protobuf::Message>::default_instance()
    }
}

impl PriorityClassList {
    pub fn new() -> PriorityClassList {
        ::std::default::Default::default()
    }

    // optional .apimachinery.pkg.apis.meta.v1.ListMeta metadata = 1;

    pub fn metadata(&self) -> &crate::v1_30::apimachinery::pkg::apis::meta::v1::ListMeta {
        self.metadata.as_ref().unwrap_or_else(|| <crate::v1_30::apimachinery::pkg::apis::meta::v1::ListMeta as ::protobuf::Message>::default_instance())
    }

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: crate::v1_30::apimachinery::pkg::apis::meta::v1::ListMeta) {
        self.metadata = ::protobuf::MessageField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut crate::v1_30::apimachinery::pkg::apis::meta::v1::ListMeta {
        self.metadata.mut_or_insert_default()
    }

    // Take field
    pub fn take_metadata(&mut self) -> crate::v1_30::apimachinery::pkg::apis::meta::v1::ListMeta {
        self.metadata.take().unwrap_or_else(|| crate::v1_30::apimachinery::pkg::apis::meta::v1::ListMeta::new())
    }

    // repeated .api.scheduling.v1alpha1.PriorityClass items = 2;

    pub fn items(&self) -> &[PriorityClass] {
        &self.items
    }

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::std::vec::Vec<PriorityClass>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::std::vec::Vec<PriorityClass> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::std::vec::Vec<PriorityClass> {
        ::std::mem::replace(&mut self.items, ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, crate::v1_30::apimachinery::pkg::apis::meta::v1::ListMeta>(
            "metadata",
            |m: &PriorityClassList| { &m.metadata },
            |m: &mut PriorityClassList| { &mut m.metadata },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "items",
            |m: &PriorityClassList| { &m.items },
            |m: &mut PriorityClassList| { &mut m.items },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PriorityClassList>(
            "PriorityClassList",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PriorityClassList {
    const NAME: &'static str = "PriorityClassList";

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

    fn new() -> PriorityClassList {
        PriorityClassList::new()
    }

    fn clear(&mut self) {
        self.metadata.clear();
        self.items.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PriorityClassList {
        static instance: PriorityClassList = PriorityClassList {
            metadata: ::protobuf::MessageField::none(),
            items: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PriorityClassList {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PriorityClassList").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PriorityClassList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PriorityClassList {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'api/scheduling/v1alpha1/generated.proto\x12\x17api.scheduling.v1alpha\
    1\x1a\x1bapi/core/v1/generated.proto\x1a-apimachinery/pkg/apis/meta/v1/g\
    enerated.proto\x1a(apimachinery/pkg/runtime/generated.proto\x1a/apimachi\
    nery/pkg/runtime/schema/generated.proto\"\xe0\x01\n\rPriorityClass\x12E\
    \n\x08metadata\x18\x01\x20\x01(\x0b2).apimachinery.pkg.apis.meta.v1.Obje\
    ctMetaR\x08metadata\x12\x14\n\x05value\x18\x02\x20\x01(\x05R\x05value\
    \x12$\n\rglobalDefault\x18\x03\x20\x01(\x08R\rglobalDefault\x12\x20\n\
    \x0bdescription\x18\x04\x20\x01(\tR\x0bdescription\x12*\n\x10preemptionP\
    olicy\x18\x05\x20\x01(\tR\x10preemptionPolicy\"\x96\x01\n\x11PriorityCla\
    ssList\x12C\n\x08metadata\x18\x01\x20\x01(\x0b2'.apimachinery.pkg.apis.m\
    eta.v1.ListMetaR\x08metadata\x12<\n\x05items\x18\x02\x20\x03(\x0b2&.api.\
    scheduling.v1alpha1.PriorityClassR\x05itemsB\x20Z\x1ek8s.io/api/scheduli\
    ng/v1alpha1\
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
            deps.push(super::v1alpha1::file_descriptor().clone());
            deps.push(super::v1alpha1::file_descriptor().clone());
            deps.push(super::v1alpha1::file_descriptor().clone());
            deps.push(super::v1alpha1::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(PriorityClass::generated_message_descriptor_data());
            messages.push(PriorityClassList::generated_message_descriptor_data());
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

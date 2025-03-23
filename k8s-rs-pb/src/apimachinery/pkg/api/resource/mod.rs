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

//! Generated file from `apimachinery/pkg/api/resource/generated.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

// @@protoc_insertion_point(message:apimachinery.pkg.api.resource.Quantity)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Quantity {
    // message fields
    // @@protoc_insertion_point(field:apimachinery.pkg.api.resource.Quantity.string)
    pub string: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:apimachinery.pkg.api.resource.Quantity.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Quantity {
    fn default() -> &'a Quantity {
        <Quantity as ::protobuf::Message>::default_instance()
    }
}

impl Quantity {
    pub fn new() -> Quantity {
        ::std::default::Default::default()
    }

    // optional string string = 1;

    pub fn string(&self) -> &str {
        match self.string.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_string(&mut self) {
        self.string = ::std::option::Option::None;
    }

    pub fn has_string(&self) -> bool {
        self.string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string(&mut self, v: ::std::string::String) {
        self.string = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string(&mut self) -> &mut ::std::string::String {
        if self.string.is_none() {
            self.string = ::std::option::Option::Some(::std::string::String::new());
        }
        self.string.as_mut().unwrap()
    }

    // Take field
    pub fn take_string(&mut self) -> ::std::string::String {
        self.string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "string",
            |m: &Quantity| { &m.string },
            |m: &mut Quantity| { &mut m.string },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Quantity>(
            "Quantity",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Quantity {
    const NAME: &'static str = "Quantity";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.string = ::std::option::Option::Some(is.read_string()?);
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
        if let Some(v) = self.string.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.string.as_ref() {
            os.write_string(1, v)?;
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

    fn new() -> Quantity {
        Quantity::new()
    }

    fn clear(&mut self) {
        self.string = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Quantity {
        static instance: Quantity = Quantity {
            string: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Quantity {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Quantity").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Quantity {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(rename_all = "snake_case")]
// @@protoc_insertion_point(message:apimachinery.pkg.api.resource.QuantityValue)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct QuantityValue {
    // message fields
    // @@protoc_insertion_point(field:apimachinery.pkg.api.resource.QuantityValue.string)
    pub string: ::std::option::Option<::std::string::String>,
    // special fields
    #[serde(skip)]
    // @@protoc_insertion_point(special_field:apimachinery.pkg.api.resource.QuantityValue.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a QuantityValue {
    fn default() -> &'a QuantityValue {
        <QuantityValue as ::protobuf::Message>::default_instance()
    }
}

impl QuantityValue {
    pub fn new() -> QuantityValue {
        ::std::default::Default::default()
    }

    // optional string string = 1;

    pub fn string(&self) -> &str {
        match self.string.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_string(&mut self) {
        self.string = ::std::option::Option::None;
    }

    pub fn has_string(&self) -> bool {
        self.string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string(&mut self, v: ::std::string::String) {
        self.string = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string(&mut self) -> &mut ::std::string::String {
        if self.string.is_none() {
            self.string = ::std::option::Option::Some(::std::string::String::new());
        }
        self.string.as_mut().unwrap()
    }

    // Take field
    pub fn take_string(&mut self) -> ::std::string::String {
        self.string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "string",
            |m: &QuantityValue| { &m.string },
            |m: &mut QuantityValue| { &mut m.string },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QuantityValue>(
            "QuantityValue",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for QuantityValue {
    const NAME: &'static str = "QuantityValue";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.string = ::std::option::Option::Some(is.read_string()?);
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
        if let Some(v) = self.string.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.string.as_ref() {
            os.write_string(1, v)?;
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

    fn new() -> QuantityValue {
        QuantityValue::new()
    }

    fn clear(&mut self) {
        self.string = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static QuantityValue {
        static instance: QuantityValue = QuantityValue {
            string: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for QuantityValue {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("QuantityValue").unwrap()).clone()
    }
}

impl ::std::fmt::Display for QuantityValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QuantityValue {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n-apimachinery/pkg/api/resource/generated.proto\x12\x1dapimachinery.pkg\
    .api.resource\"\"\n\x08Quantity\x12\x16\n\x06string\x18\x01\x20\x01(\tR\
    \x06string\"'\n\rQuantityValue\x12\x16\n\x06string\x18\x01\x20\x01(\tR\
    \x06stringB&Z$k8s.io/apimachinery/pkg/api/resource\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(Quantity::generated_message_descriptor_data());
            messages.push(QuantityValue::generated_message_descriptor_data());
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

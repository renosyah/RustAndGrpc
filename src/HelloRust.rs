// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct RequestHelloRust {
    // message fields
    pub Text: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestHelloRust {}

impl RequestHelloRust {
    pub fn new() -> RequestHelloRust {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestHelloRust {
        static mut instance: ::protobuf::lazy::Lazy<RequestHelloRust> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestHelloRust,
        };
        unsafe {
            instance.get(RequestHelloRust::new)
        }
    }

    // string Text = 1;

    pub fn clear_Text(&mut self) {
        self.Text.clear();
    }

    // Param is passed by value, moved
    pub fn set_Text(&mut self, v: ::std::string::String) {
        self.Text = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Text(&mut self) -> &mut ::std::string::String {
        &mut self.Text
    }

    // Take field
    pub fn take_Text(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.Text, ::std::string::String::new())
    }

    pub fn get_Text(&self) -> &str {
        &self.Text
    }

    fn get_Text_for_reflect(&self) -> &::std::string::String {
        &self.Text
    }

    fn mut_Text_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.Text
    }
}

impl ::protobuf::Message for RequestHelloRust {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.Text)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.Text.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.Text);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.Text.is_empty() {
            os.write_string(1, &self.Text)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestHelloRust {
    fn new() -> RequestHelloRust {
        RequestHelloRust::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestHelloRust>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "Text",
                    RequestHelloRust::get_Text_for_reflect,
                    RequestHelloRust::mut_Text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestHelloRust>(
                    "RequestHelloRust",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestHelloRust {
    fn clear(&mut self) {
        self.clear_Text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestHelloRust {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestHelloRust {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseHelloRust {
    // message fields
    pub ResponseText: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseHelloRust {}

impl ResponseHelloRust {
    pub fn new() -> ResponseHelloRust {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseHelloRust {
        static mut instance: ::protobuf::lazy::Lazy<ResponseHelloRust> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseHelloRust,
        };
        unsafe {
            instance.get(ResponseHelloRust::new)
        }
    }

    // string ResponseText = 1;

    pub fn clear_ResponseText(&mut self) {
        self.ResponseText.clear();
    }

    // Param is passed by value, moved
    pub fn set_ResponseText(&mut self, v: ::std::string::String) {
        self.ResponseText = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ResponseText(&mut self) -> &mut ::std::string::String {
        &mut self.ResponseText
    }

    // Take field
    pub fn take_ResponseText(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ResponseText, ::std::string::String::new())
    }

    pub fn get_ResponseText(&self) -> &str {
        &self.ResponseText
    }

    fn get_ResponseText_for_reflect(&self) -> &::std::string::String {
        &self.ResponseText
    }

    fn mut_ResponseText_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.ResponseText
    }
}

impl ::protobuf::Message for ResponseHelloRust {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ResponseText)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.ResponseText.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.ResponseText);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.ResponseText.is_empty() {
            os.write_string(1, &self.ResponseText)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseHelloRust {
    fn new() -> ResponseHelloRust {
        ResponseHelloRust::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseHelloRust>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ResponseText",
                    ResponseHelloRust::get_ResponseText_for_reflect,
                    ResponseHelloRust::mut_ResponseText_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseHelloRust>(
                    "ResponseHelloRust",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseHelloRust {
    fn clear(&mut self) {
        self.clear_ResponseText();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseHelloRust {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseHelloRust {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fHelloRust.proto\x12\x0fHelloRustHandle\"&\n\x10RequestHelloRust\
    \x12\x12\n\x04Text\x18\x01\x20\x01(\tR\x04Text\"7\n\x11ResponseHelloRust\
    \x12\"\n\x0cResponseText\x18\x01\x20\x01(\tR\x0cResponseText2l\n\x16Hell\
    oRustHandleService\x12R\n\tHelloRust\x12!.HelloRustHandle.RequestHelloRu\
    st\x1a\".HelloRustHandle.ResponseHelloRustb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Status {
    // message fields
    code: ::std::option::Option<i32>,
    message: ::protobuf::SingularField<::std::string::String>,
    details: ::protobuf::RepeatedField<super::any::Any>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Status {}

impl Status {
    pub fn new() -> Status {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Status {
        static mut instance: ::protobuf::lazy::Lazy<Status> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Status,
        };
        unsafe {
            instance.get(|| {
                Status {
                    code: ::std::option::Option::None,
                    message: ::protobuf::SingularField::none(),
                    details: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = ::std::option::Option::None;
    }

    pub fn has_code(&self) -> bool {
        self.code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: i32) {
        self.code = ::std::option::Option::Some(v);
    }

    pub fn get_code(&self) -> i32 {
        self.code.unwrap_or(0)
    }

    // optional string message = 2;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .google.protobuf.Any details = 3;

    pub fn clear_details(&mut self) {
        self.details.clear();
    }

    // Param is passed by value, moved
    pub fn set_details(&mut self, v: ::protobuf::RepeatedField<super::any::Any>) {
        self.details = v;
    }

    // Mutable pointer to the field.
    pub fn mut_details(&mut self) -> &mut ::protobuf::RepeatedField<super::any::Any> {
        &mut self.details
    }

    // Take field
    pub fn take_details(&mut self) -> ::protobuf::RepeatedField<super::any::Any> {
        ::std::mem::replace(&mut self.details, ::protobuf::RepeatedField::new())
    }

    pub fn get_details(&self) -> &[super::any::Any] {
        &self.details
    }
}

impl ::protobuf::Message for Status {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.code = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.details));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.code {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.message {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.details {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.code {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(2, &v));
        };
        for v in &self.details {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Status>()
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Status {
    fn new() -> Status {
        Status::new()
    }

    fn descriptor_static(_: ::std::option::Option<Status>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "code",
                    Status::has_code,
                    Status::get_code,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    Status::has_message,
                    Status::get_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "details",
                    Status::get_details,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Status>(
                    "Status",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Status {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_message();
        self.clear_details();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Status {
    fn eq(&self, other: &Status) -> bool {
        self.code == other.code &&
        self.message == other.message &&
        self.details == other.details &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x17, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x72, 0x70, 0x63, 0x2f, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x2e, 0x72, 0x70, 0x63, 0x1a, 0x19, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x61, 0x6e, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x66, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x63, 0x6f,
    0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x18,
    0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x2e, 0x0a, 0x07, 0x64, 0x65, 0x74, 0x61,
    0x69, 0x6c, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x41, 0x6e, 0x79, 0x52,
    0x07, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x42, 0x5e, 0x0a, 0x0e, 0x63, 0x6f, 0x6d, 0x2e,
    0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x72, 0x70, 0x63, 0x42, 0x0b, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x37, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x2e, 0x67, 0x6f, 0x6c, 0x61, 0x6e, 0x67, 0x2e, 0x6f, 0x72, 0x67, 0x2f, 0x67, 0x65, 0x6e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x61, 0x70, 0x69, 0x73,
    0x2f, 0x72, 0x70, 0x63, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x3b, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0xa2, 0x02, 0x03, 0x52, 0x50, 0x43, 0x4a, 0xda, 0x20, 0x0a, 0x06, 0x12, 0x04, 0x0e,
    0x00, 0x5b, 0x01, 0x0a, 0xbd, 0x04, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x0e, 0x00, 0x12, 0x32, 0xb2,
    0x04, 0x20, 0x43, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x32, 0x30, 0x31, 0x36,
    0x20, 0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x20, 0x49, 0x6e, 0x63, 0x2e, 0x0a, 0x0a, 0x20, 0x4c,
    0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x64, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65,
    0x2c, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x32, 0x2e, 0x30, 0x20, 0x28, 0x74,
    0x68, 0x65, 0x20, 0x22, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x22, 0x29, 0x3b, 0x0a, 0x20,
    0x79, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x75, 0x73, 0x65, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x65, 0x78, 0x63, 0x65, 0x70, 0x74,
    0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x69, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e,
    0x0a, 0x20, 0x59, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6f, 0x62, 0x74, 0x61, 0x69, 0x6e,
    0x20, 0x61, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c,
    0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x61, 0x74, 0x0a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x77, 0x77, 0x77, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68,
    0x65, 0x2e, 0x6f, 0x72, 0x67, 0x2f, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x73, 0x2f, 0x4c,
    0x49, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x2d, 0x32, 0x2e, 0x30, 0x0a, 0x0a, 0x20, 0x55, 0x6e, 0x6c,
    0x65, 0x73, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20,
    0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6c, 0x61, 0x77, 0x20, 0x6f,
    0x72, 0x20, 0x61, 0x67, 0x72, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x20, 0x77,
    0x72, 0x69, 0x74, 0x69, 0x6e, 0x67, 0x2c, 0x20, 0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65,
    0x0a, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x75, 0x6e,
    0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20,
    0x69, 0x73, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x6f,
    0x6e, 0x20, 0x61, 0x6e, 0x20, 0x22, 0x41, 0x53, 0x20, 0x49, 0x53, 0x22, 0x20, 0x42, 0x41, 0x53,
    0x49, 0x53, 0x2c, 0x0a, 0x20, 0x57, 0x49, 0x54, 0x48, 0x4f, 0x55, 0x54, 0x20, 0x57, 0x41, 0x52,
    0x52, 0x41, 0x4e, 0x54, 0x49, 0x45, 0x53, 0x20, 0x4f, 0x52, 0x20, 0x43, 0x4f, 0x4e, 0x44, 0x49,
    0x54, 0x49, 0x4f, 0x4e, 0x53, 0x20, 0x4f, 0x46, 0x20, 0x41, 0x4e, 0x59, 0x20, 0x4b, 0x49, 0x4e,
    0x44, 0x2c, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73,
    0x73, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x53,
    0x65, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20,
    0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x20, 0x67, 0x6f, 0x76, 0x65, 0x72, 0x6e, 0x69,
    0x6e, 0x67, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61,
    0x6e, 0x64, 0x0a, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20,
    0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73,
    0x65, 0x2e, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x10, 0x08, 0x12, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x12, 0x07, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x14,
    0x00, 0x4e, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x14, 0x00, 0x4e, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x14, 0x07, 0x11, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x14, 0x07, 0x11, 0x0a, 0x0e, 0x0a, 0x07,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x07, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x14, 0x14, 0x4d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x15, 0x00, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x15, 0x00,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x15, 0x07, 0x1a, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x15, 0x07, 0x1a, 0x0a, 0x0e,
    0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x07, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x15, 0x1d, 0x21, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x16, 0x00, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02, 0x12, 0x03,
    0x16, 0x00, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x12, 0x03, 0x16, 0x07,
    0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x12, 0x03, 0x16, 0x07, 0x1b,
    0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x07, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x07, 0x12, 0x03, 0x16, 0x1e, 0x2b, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x17, 0x00, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x03,
    0x12, 0x03, 0x17, 0x00, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x03, 0x02, 0x12, 0x03,
    0x17, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x03, 0x02, 0x00, 0x12, 0x03, 0x17,
    0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17,
    0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x03, 0x07, 0x12, 0x03, 0x17, 0x16, 0x26,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x18, 0x00, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7,
    0x07, 0x04, 0x12, 0x03, 0x18, 0x00, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x04, 0x02,
    0x12, 0x03, 0x18, 0x07, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x04, 0x02, 0x00, 0x12,
    0x03, 0x18, 0x07, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x04, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x18, 0x07, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x04, 0x07, 0x12, 0x03, 0x18,
    0x1b, 0x20, 0x0a, 0xd6, 0x13, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x4f, 0x00, 0x5b, 0x01, 0x1a,
    0xc9, 0x13, 0x20, 0x54, 0x68, 0x65, 0x20, 0x60, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x60, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x61, 0x20, 0x6c,
    0x6f, 0x67, 0x69, 0x63, 0x61, 0x6c, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x64,
    0x65, 0x6c, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x75, 0x69, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e,
    0x74, 0x0a, 0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x6d, 0x69, 0x6e, 0x67, 0x20, 0x65,
    0x6e, 0x76, 0x69, 0x72, 0x6f, 0x6e, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x2c, 0x20, 0x69, 0x6e, 0x63,
    0x6c, 0x75, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x52, 0x45, 0x53, 0x54, 0x20, 0x41, 0x50, 0x49, 0x73,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x52, 0x50, 0x43, 0x20, 0x41, 0x50, 0x49, 0x73, 0x2e, 0x20, 0x49,
    0x74, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x0a, 0x20, 0x5b, 0x67,
    0x52, 0x50, 0x43, 0x5d, 0x28, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x67, 0x69, 0x74,
    0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x67, 0x72, 0x70, 0x63, 0x29, 0x2e, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x20, 0x69,
    0x73, 0x20, 0x64, 0x65, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65,
    0x3a, 0x0a, 0x0a, 0x20, 0x2d, 0x20, 0x53, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x75, 0x73, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x73, 0x74, 0x61,
    0x6e, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x73, 0x74, 0x20, 0x75, 0x73, 0x65, 0x72,
    0x73, 0x0a, 0x20, 0x2d, 0x20, 0x46, 0x6c, 0x65, 0x78, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x65, 0x6e,
    0x6f, 0x75, 0x67, 0x68, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x65, 0x65, 0x74, 0x20, 0x75, 0x6e, 0x65,
    0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x0a, 0x0a, 0x20,
    0x23, 0x20, 0x4f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x0a, 0x0a, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x60, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x60, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x72, 0x65, 0x65,
    0x20, 0x70, 0x69, 0x65, 0x63, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x64, 0x61, 0x74, 0x61, 0x3a,
    0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x2c, 0x20, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2c, 0x0a, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x2e, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x75, 0x6d,
    0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x5b, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2e, 0x72, 0x70, 0x63, 0x2e, 0x43, 0x6f, 0x64, 0x65, 0x5d, 0x5b, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2e, 0x72, 0x70, 0x63, 0x2e, 0x43, 0x6f, 0x64, 0x65, 0x5d, 0x2c, 0x20, 0x62,
    0x75, 0x74, 0x20, 0x69, 0x74, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74,
    0x20, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x73, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x65,
    0x64, 0x2e, 0x20, 0x20, 0x54, 0x68, 0x65, 0x0a, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65,
    0x20, 0x61, 0x20, 0x64, 0x65, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x72, 0x2d, 0x66, 0x61, 0x63,
    0x69, 0x6e, 0x67, 0x20, 0x45, 0x6e, 0x67, 0x6c, 0x69, 0x73, 0x68, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x68, 0x65, 0x6c, 0x70, 0x73, 0x0a, 0x20,
    0x64, 0x65, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x72, 0x73, 0x20, 0x2a, 0x75, 0x6e, 0x64, 0x65,
    0x72, 0x73, 0x74, 0x61, 0x6e, 0x64, 0x2a, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x2a, 0x72, 0x65, 0x73,
    0x6f, 0x6c, 0x76, 0x65, 0x2a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e,
    0x20, 0x49, 0x66, 0x20, 0x61, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20,
    0x75, 0x73, 0x65, 0x72, 0x2d, 0x66, 0x61, 0x63, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x65,
    0x65, 0x64, 0x65, 0x64, 0x2c, 0x20, 0x70, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f,
    0x63, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x64, 0x65, 0x74,
    0x61, 0x69, 0x6c, 0x73, 0x20, 0x6f, 0x72, 0x0a, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x69, 0x7a,
    0x65, 0x20, 0x69, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x20, 0x6d,
    0x61, 0x79, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x72, 0x62, 0x69, 0x74,
    0x72, 0x61, 0x72, 0x79, 0x0a, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x70, 0x72,
    0x65, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x20, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x0a, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61,
    0x67, 0x65, 0x20, 0x60, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x72, 0x70, 0x63, 0x60, 0x20,
    0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65,
    0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x20, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a,
    0x20, 0x23, 0x20, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x20, 0x6d, 0x61, 0x70, 0x70,
    0x69, 0x6e, 0x67, 0x0a, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x60, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x60, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6c, 0x6f, 0x67, 0x69, 0x63, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73,
    0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x2c, 0x20, 0x62, 0x75, 0x74,
    0x20, 0x69, 0x74, 0x0a, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6e, 0x65, 0x63, 0x65,
    0x73, 0x73, 0x61, 0x72, 0x69, 0x6c, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x63, 0x74, 0x75,
    0x61, 0x6c, 0x20, 0x77, 0x69, 0x72, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x2e, 0x20,
    0x57, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x60, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x60, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x65, 0x78,
    0x70, 0x6f, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65,
    0x6e, 0x74, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x72,
    0x69, 0x65, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e,
    0x74, 0x20, 0x77, 0x69, 0x72, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73,
    0x2c, 0x20, 0x69, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x6d, 0x61, 0x70,
    0x70, 0x65, 0x64, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x2e,
    0x20, 0x46, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2c, 0x20, 0x69, 0x74,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20,
    0x6d, 0x61, 0x70, 0x70, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x20, 0x65,
    0x78, 0x63, 0x65, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x20, 0x69, 0x6e, 0x20, 0x4a, 0x61,
    0x76, 0x61, 0x2c, 0x20, 0x62, 0x75, 0x74, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x6c, 0x69, 0x6b,
    0x65, 0x6c, 0x79, 0x20, 0x6d, 0x61, 0x70, 0x70, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x6f,
    0x6d, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x73, 0x20, 0x69,
    0x6e, 0x20, 0x43, 0x2e, 0x0a, 0x0a, 0x20, 0x23, 0x20, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x75,
    0x73, 0x65, 0x73, 0x0a, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20,
    0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x60, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x60, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x63,
    0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20,
    0x76, 0x61, 0x72, 0x69, 0x65, 0x74, 0x79, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x65, 0x6e, 0x76, 0x69,
    0x72, 0x6f, 0x6e, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x2c, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6f, 0x72, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74,
    0x20, 0x41, 0x50, 0x49, 0x73, 0x2c, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64,
    0x65, 0x20, 0x61, 0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x74, 0x20,
    0x64, 0x65, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x72, 0x20, 0x65, 0x78, 0x70, 0x65, 0x72, 0x69,
    0x65, 0x6e, 0x63, 0x65, 0x20, 0x61, 0x63, 0x72, 0x6f, 0x73, 0x73, 0x20, 0x64, 0x69, 0x66, 0x66,
    0x65, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x65, 0x6e, 0x76, 0x69, 0x72, 0x6f, 0x6e, 0x6d, 0x65, 0x6e,
    0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x20, 0x45, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x75, 0x73,
    0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x20, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x3a, 0x0a,
    0x0a, 0x20, 0x2d, 0x20, 0x50, 0x61, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x20, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x73, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x61, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x20, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e,
    0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2c, 0x0a, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x69, 0x74, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x65, 0x6d, 0x62, 0x65, 0x64,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x60, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x60, 0x20, 0x69, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x61, 0x6c, 0x0a, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x20, 0x2d, 0x20, 0x57, 0x6f,
    0x72, 0x6b, 0x66, 0x6c, 0x6f, 0x77, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x2e, 0x20, 0x41,
    0x20, 0x74, 0x79, 0x70, 0x69, 0x63, 0x61, 0x6c, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x66, 0x6c, 0x6f,
    0x77, 0x20, 0x68, 0x61, 0x73, 0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x20, 0x73,
    0x74, 0x65, 0x70, 0x73, 0x2e, 0x20, 0x45, 0x61, 0x63, 0x68, 0x20, 0x73, 0x74, 0x65, 0x70, 0x20,
    0x6d, 0x61, 0x79, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x61, 0x20,
    0x60, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x60, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72,
    0x74, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x75, 0x72, 0x70, 0x6f, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x20,
    0x2d, 0x20, 0x42, 0x61, 0x74, 0x63, 0x68, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x61, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20,
    0x75, 0x73, 0x65, 0x73, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x60, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x60, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20,
    0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x69, 0x6e, 0x73, 0x69, 0x64, 0x65, 0x20,
    0x62, 0x61, 0x74, 0x63, 0x68, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2c, 0x20,
    0x6f, 0x6e, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x65, 0x61, 0x63,
    0x68, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x73, 0x75, 0x62, 0x2d, 0x72, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x20, 0x2d, 0x20, 0x41, 0x73, 0x79, 0x6e, 0x63, 0x68,
    0x72, 0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x41, 0x50, 0x49, 0x20, 0x63, 0x61, 0x6c,
    0x6c, 0x20, 0x65, 0x6d, 0x62, 0x65, 0x64, 0x73, 0x20, 0x61, 0x73, 0x79, 0x6e, 0x63, 0x68, 0x72,
    0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x69, 0x6e, 0x20,
    0x69, 0x74, 0x73, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2c, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x6f, 0x73,
    0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x73, 0x68, 0x6f,
    0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x72, 0x65, 0x70, 0x72,
    0x65, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6c, 0x79,
    0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x60, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x60, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x20, 0x2d,
    0x20, 0x4c, 0x6f, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x73, 0x6f, 0x6d,
    0x65, 0x20, 0x41, 0x50, 0x49, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x6c, 0x6f, 0x67, 0x73, 0x2c,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x60, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x60, 0x20, 0x63, 0x6f, 0x75, 0x6c, 0x64, 0x0a, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6c,
    0x79, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x73, 0x74, 0x72, 0x69,
    0x70, 0x70, 0x69, 0x6e, 0x67, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x73, 0x65, 0x63, 0x75, 0x72, 0x69, 0x74, 0x79, 0x2f, 0x70, 0x72, 0x69, 0x76, 0x61, 0x63,
    0x79, 0x20, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x4f, 0x08, 0x0e, 0x0a, 0x64, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x51, 0x02, 0x11, 0x1a, 0x57, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x2c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x73, 0x68,
    0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x75, 0x6d, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x5b, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
    0x2e, 0x72, 0x70, 0x63, 0x2e, 0x43, 0x6f, 0x64, 0x65, 0x5d, 0x5b, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x2e, 0x72, 0x70, 0x63, 0x2e, 0x43, 0x6f, 0x64, 0x65, 0x5d, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x51, 0x02, 0x4f, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x51, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x51, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x51, 0x0f, 0x10, 0x0a, 0xeb, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x56, 0x02, 0x15, 0x1a, 0xdd, 0x01, 0x20, 0x41, 0x20, 0x64, 0x65, 0x76, 0x65, 0x6c, 0x6f,
    0x70, 0x65, 0x72, 0x2d, 0x66, 0x61, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20,
    0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x45, 0x6e, 0x67,
    0x6c, 0x69, 0x73, 0x68, 0x2e, 0x20, 0x41, 0x6e, 0x79, 0x0a, 0x20, 0x75, 0x73, 0x65, 0x72, 0x2d,
    0x66, 0x61, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x6c,
    0x6f, 0x63, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x65, 0x6e,
    0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x5b, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x2e, 0x72, 0x70, 0x63, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x64, 0x65, 0x74,
    0x61, 0x69, 0x6c, 0x73, 0x5d, 0x5b, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x72, 0x70, 0x63,
    0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x5d,
    0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c,
    0x69, 0x7a, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x56,
    0x02, 0x51, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x56, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x56, 0x09, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x56, 0x13, 0x14, 0x0a, 0x7e, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x5a, 0x02, 0x2b, 0x1a, 0x71, 0x20, 0x41, 0x20, 0x6c,
    0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x61, 0x72, 0x72, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x2e, 0x20, 0x20, 0x54,
    0x68, 0x65, 0x72, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x0a, 0x20,
    0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x41, 0x50, 0x49, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x5a, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x5a, 0x1f, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x5a, 0x29, 0x2a, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

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

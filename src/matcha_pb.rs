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
pub struct Wallet {
    // message fields
    pub keypairs: ::protobuf::RepeatedField<WalletKeypair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Wallet {}

impl Wallet {
    pub fn new() -> Wallet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Wallet {
        static mut instance: ::protobuf::lazy::Lazy<Wallet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Wallet,
        };
        unsafe {
            instance.get(Wallet::new)
        }
    }

    // repeated .matcha.WalletKeypair keypairs = 1;

    pub fn clear_keypairs(&mut self) {
        self.keypairs.clear();
    }

    // Param is passed by value, moved
    pub fn set_keypairs(&mut self, v: ::protobuf::RepeatedField<WalletKeypair>) {
        self.keypairs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keypairs(&mut self) -> &mut ::protobuf::RepeatedField<WalletKeypair> {
        &mut self.keypairs
    }

    // Take field
    pub fn take_keypairs(&mut self) -> ::protobuf::RepeatedField<WalletKeypair> {
        ::std::mem::replace(&mut self.keypairs, ::protobuf::RepeatedField::new())
    }

    pub fn get_keypairs(&self) -> &[WalletKeypair] {
        &self.keypairs
    }

    fn get_keypairs_for_reflect(&self) -> &::protobuf::RepeatedField<WalletKeypair> {
        &self.keypairs
    }

    fn mut_keypairs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<WalletKeypair> {
        &mut self.keypairs
    }
}

impl ::protobuf::Message for Wallet {
    fn is_initialized(&self) -> bool {
        for v in &self.keypairs {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keypairs)?;
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
        for value in &self.keypairs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keypairs {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for Wallet {
    fn new() -> Wallet {
        Wallet::new()
    }

    fn descriptor_static(_: ::std::option::Option<Wallet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<WalletKeypair>>(
                    "keypairs",
                    Wallet::get_keypairs_for_reflect,
                    Wallet::mut_keypairs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Wallet>(
                    "Wallet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Wallet {
    fn clear(&mut self) {
        self.clear_keypairs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Wallet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Wallet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WalletKeypair {
    // message fields
    pub public_key: ::std::vec::Vec<u8>,
    pub secret_key: ::std::vec::Vec<u8>,
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WalletKeypair {}

impl WalletKeypair {
    pub fn new() -> WalletKeypair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WalletKeypair {
        static mut instance: ::protobuf::lazy::Lazy<WalletKeypair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WalletKeypair,
        };
        unsafe {
            instance.get(WalletKeypair::new)
        }
    }

    // bytes public_key = 1;

    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.public_key
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.public_key, ::std::vec::Vec::new())
    }

    pub fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }

    fn get_public_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.public_key
    }

    fn mut_public_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.public_key
    }

    // bytes secret_key = 2;

    pub fn clear_secret_key(&mut self) {
        self.secret_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_secret_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.secret_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secret_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.secret_key
    }

    // Take field
    pub fn take_secret_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.secret_key, ::std::vec::Vec::new())
    }

    pub fn get_secret_key(&self) -> &[u8] {
        &self.secret_key
    }

    fn get_secret_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.secret_key
    }

    fn mut_secret_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.secret_key
    }

    // string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for WalletKeypair {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.public_key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.secret_key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if !self.public_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.public_key);
        }
        if !self.secret_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.secret_key);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.public_key.is_empty() {
            os.write_bytes(1, &self.public_key)?;
        }
        if !self.secret_key.is_empty() {
            os.write_bytes(2, &self.secret_key)?;
        }
        if !self.name.is_empty() {
            os.write_string(3, &self.name)?;
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

impl ::protobuf::MessageStatic for WalletKeypair {
    fn new() -> WalletKeypair {
        WalletKeypair::new()
    }

    fn descriptor_static(_: ::std::option::Option<WalletKeypair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public_key",
                    WalletKeypair::get_public_key_for_reflect,
                    WalletKeypair::mut_public_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "secret_key",
                    WalletKeypair::get_secret_key_for_reflect,
                    WalletKeypair::mut_secret_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    WalletKeypair::get_name_for_reflect,
                    WalletKeypair::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WalletKeypair>(
                    "WalletKeypair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WalletKeypair {
    fn clear(&mut self) {
        self.clear_public_key();
        self.clear_secret_key();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WalletKeypair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WalletKeypair {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1asrc/protos/matcha_pb.proto\x12\x06matcha\";\n\x06Wallet\x121\n\x08\
    keypairs\x18\x01\x20\x03(\x0b2\x15.matcha.WalletKeypairR\x08keypairs\"a\
    \n\rWalletKeypair\x12\x1d\n\npublic_key\x18\x01\x20\x01(\x0cR\tpublicKey\
    \x12\x1d\n\nsecret_key\x18\x02\x20\x01(\x0cR\tsecretKey\x12\x12\n\x04nam\
    e\x18\x03\x20\x01(\tR\x04nameJ\xe3\x02\n\x06\x12\x04\0\0\x0c\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x0e\n\n\n\x02\
    \x04\0\x12\x04\x04\0\x06\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\x0e\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\x05\x02&\n\x0c\n\x05\x04\0\x02\0\x04\x12\
    \x03\x05\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x05\x0b\x18\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x05\x19!\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x05$%\n\n\n\x02\x04\x01\x12\x04\x08\0\x0c\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03\x08\x08\x15\n\x0b\n\x04\x04\x01\x02\0\x12\x03\t\x02\x17\n\r\n\
    \x05\x04\x01\x02\0\x04\x12\x04\t\x02\x08\x17\n\x0c\n\x05\x04\x01\x02\0\
    \x05\x12\x03\t\x02\x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\t\x08\x12\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\t\x15\x16\n\x0b\n\x04\x04\x01\x02\
    \x01\x12\x03\n\x02\x17\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\n\x02\t\x17\
    \n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\n\x02\x07\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\n\x08\x12\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\n\
    \x15\x16\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x0b\x02\x12\n\r\n\x05\x04\
    \x01\x02\x02\x04\x12\x04\x0b\x02\n\x17\n\x0c\n\x05\x04\x01\x02\x02\x05\
    \x12\x03\x0b\x02\x08\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x0b\t\r\n\
    \x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x0b\x10\x11b\x06proto3\
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

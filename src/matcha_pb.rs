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
    pub name: ::std::string::String,
    pub public_key: ::std::vec::Vec<u8>,
    pub secret_key: ::std::vec::Vec<u8>,
    pub amount: u64,
    pub network_type: u32,
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

    // string name = 1;

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

    // bytes public_key = 2;

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

    // bytes secret_key = 3;

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

    // uint64 amount = 4;

    pub fn clear_amount(&mut self) {
        self.amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: u64) {
        self.amount = v;
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }

    fn get_amount_for_reflect(&self) -> &u64 {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut u64 {
        &mut self.amount
    }

    // uint32 network_type = 5;

    pub fn clear_network_type(&mut self) {
        self.network_type = 0;
    }

    // Param is passed by value, moved
    pub fn set_network_type(&mut self, v: u32) {
        self.network_type = v;
    }

    pub fn get_network_type(&self) -> u32 {
        self.network_type
    }

    fn get_network_type_for_reflect(&self) -> &u32 {
        &self.network_type
    }

    fn mut_network_type_for_reflect(&mut self) -> &mut u32 {
        &mut self.network_type
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.public_key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.secret_key)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.amount = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.network_type = tmp;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.public_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.public_key);
        }
        if !self.secret_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.secret_key);
        }
        if self.amount != 0 {
            my_size += ::protobuf::rt::value_size(4, self.amount, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.network_type != 0 {
            my_size += ::protobuf::rt::value_size(5, self.network_type, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.public_key.is_empty() {
            os.write_bytes(2, &self.public_key)?;
        }
        if !self.secret_key.is_empty() {
            os.write_bytes(3, &self.secret_key)?;
        }
        if self.amount != 0 {
            os.write_uint64(4, self.amount)?;
        }
        if self.network_type != 0 {
            os.write_uint32(5, self.network_type)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    WalletKeypair::get_name_for_reflect,
                    WalletKeypair::mut_name_for_reflect,
                ));
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "amount",
                    WalletKeypair::get_amount_for_reflect,
                    WalletKeypair::mut_amount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "network_type",
                    WalletKeypair::get_network_type_for_reflect,
                    WalletKeypair::mut_network_type_for_reflect,
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
        self.clear_name();
        self.clear_public_key();
        self.clear_secret_key();
        self.clear_amount();
        self.clear_network_type();
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

#[derive(PartialEq,Clone,Default)]
pub struct InputTransaction {
    // message fields
    pub tx_id: ::std::vec::Vec<u8>,
    pub txout_index: u32,
    pub signature: ::std::vec::Vec<u8>,
    pub public_key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InputTransaction {}

impl InputTransaction {
    pub fn new() -> InputTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InputTransaction {
        static mut instance: ::protobuf::lazy::Lazy<InputTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InputTransaction,
        };
        unsafe {
            instance.get(InputTransaction::new)
        }
    }

    // bytes tx_id = 1;

    pub fn clear_tx_id(&mut self) {
        self.tx_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.tx_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx_id
    }

    // Take field
    pub fn take_tx_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tx_id, ::std::vec::Vec::new())
    }

    pub fn get_tx_id(&self) -> &[u8] {
        &self.tx_id
    }

    fn get_tx_id_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.tx_id
    }

    fn mut_tx_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx_id
    }

    // uint32 txout_index = 2;

    pub fn clear_txout_index(&mut self) {
        self.txout_index = 0;
    }

    // Param is passed by value, moved
    pub fn set_txout_index(&mut self, v: u32) {
        self.txout_index = v;
    }

    pub fn get_txout_index(&self) -> u32 {
        self.txout_index
    }

    fn get_txout_index_for_reflect(&self) -> &u32 {
        &self.txout_index
    }

    fn mut_txout_index_for_reflect(&mut self) -> &mut u32 {
        &mut self.txout_index
    }

    // bytes signature = 3;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.signature, ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        &self.signature
    }

    fn get_signature_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // bytes public_key = 4;

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
}

impl ::protobuf::Message for InputTransaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tx_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.txout_index = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signature)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.public_key)?;
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
        if !self.tx_id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.tx_id);
        }
        if self.txout_index != 0 {
            my_size += ::protobuf::rt::value_size(2, self.txout_index, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.signature);
        }
        if !self.public_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.public_key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx_id.is_empty() {
            os.write_bytes(1, &self.tx_id)?;
        }
        if self.txout_index != 0 {
            os.write_uint32(2, self.txout_index)?;
        }
        if !self.signature.is_empty() {
            os.write_bytes(3, &self.signature)?;
        }
        if !self.public_key.is_empty() {
            os.write_bytes(4, &self.public_key)?;
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

impl ::protobuf::MessageStatic for InputTransaction {
    fn new() -> InputTransaction {
        InputTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<InputTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx_id",
                    InputTransaction::get_tx_id_for_reflect,
                    InputTransaction::mut_tx_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "txout_index",
                    InputTransaction::get_txout_index_for_reflect,
                    InputTransaction::mut_txout_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    InputTransaction::get_signature_for_reflect,
                    InputTransaction::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public_key",
                    InputTransaction::get_public_key_for_reflect,
                    InputTransaction::mut_public_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InputTransaction>(
                    "InputTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InputTransaction {
    fn clear(&mut self) {
        self.clear_tx_id();
        self.clear_txout_index();
        self.clear_signature();
        self.clear_public_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InputTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InputTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OutputTransaction {
    // message fields
    pub transaction_type: OutputTransaction_TransactionType,
    pub amount: u64,
    pub normal_tx: ::protobuf::SingularPtrField<OutputTransaction_NormalTransaction>,
    pub delegate_vote_tx: ::protobuf::SingularPtrField<OutputTransaction_DelegateVoteTransaction>,
    pub username_registration_tx: ::protobuf::SingularPtrField<OutputTransaction_UsernameRegistrationTransaction>,
    pub new_post_tx: ::protobuf::SingularPtrField<OutputTransaction_NewPostTransaction>,
    pub avatar_upload_tx: ::protobuf::SingularPtrField<OutputTransaction_AvatarUploadTransaction>,
    pub favourite_post_tx: ::protobuf::SingularPtrField<OutputTransaction_FavouritePostTransaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OutputTransaction {}

impl OutputTransaction {
    pub fn new() -> OutputTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OutputTransaction {
        static mut instance: ::protobuf::lazy::Lazy<OutputTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OutputTransaction,
        };
        unsafe {
            instance.get(OutputTransaction::new)
        }
    }

    // .matcha.OutputTransaction.TransactionType transaction_type = 1;

    pub fn clear_transaction_type(&mut self) {
        self.transaction_type = OutputTransaction_TransactionType::NORMAL_TX;
    }

    // Param is passed by value, moved
    pub fn set_transaction_type(&mut self, v: OutputTransaction_TransactionType) {
        self.transaction_type = v;
    }

    pub fn get_transaction_type(&self) -> OutputTransaction_TransactionType {
        self.transaction_type
    }

    fn get_transaction_type_for_reflect(&self) -> &OutputTransaction_TransactionType {
        &self.transaction_type
    }

    fn mut_transaction_type_for_reflect(&mut self) -> &mut OutputTransaction_TransactionType {
        &mut self.transaction_type
    }

    // uint64 amount = 2;

    pub fn clear_amount(&mut self) {
        self.amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: u64) {
        self.amount = v;
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }

    fn get_amount_for_reflect(&self) -> &u64 {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut u64 {
        &mut self.amount
    }

    // .matcha.OutputTransaction.NormalTransaction normal_tx = 3;

    pub fn clear_normal_tx(&mut self) {
        self.normal_tx.clear();
    }

    pub fn has_normal_tx(&self) -> bool {
        self.normal_tx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_normal_tx(&mut self, v: OutputTransaction_NormalTransaction) {
        self.normal_tx = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_normal_tx(&mut self) -> &mut OutputTransaction_NormalTransaction {
        if self.normal_tx.is_none() {
            self.normal_tx.set_default();
        }
        self.normal_tx.as_mut().unwrap()
    }

    // Take field
    pub fn take_normal_tx(&mut self) -> OutputTransaction_NormalTransaction {
        self.normal_tx.take().unwrap_or_else(|| OutputTransaction_NormalTransaction::new())
    }

    pub fn get_normal_tx(&self) -> &OutputTransaction_NormalTransaction {
        self.normal_tx.as_ref().unwrap_or_else(|| OutputTransaction_NormalTransaction::default_instance())
    }

    fn get_normal_tx_for_reflect(&self) -> &::protobuf::SingularPtrField<OutputTransaction_NormalTransaction> {
        &self.normal_tx
    }

    fn mut_normal_tx_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OutputTransaction_NormalTransaction> {
        &mut self.normal_tx
    }

    // .matcha.OutputTransaction.DelegateVoteTransaction delegate_vote_tx = 4;

    pub fn clear_delegate_vote_tx(&mut self) {
        self.delegate_vote_tx.clear();
    }

    pub fn has_delegate_vote_tx(&self) -> bool {
        self.delegate_vote_tx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delegate_vote_tx(&mut self, v: OutputTransaction_DelegateVoteTransaction) {
        self.delegate_vote_tx = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delegate_vote_tx(&mut self) -> &mut OutputTransaction_DelegateVoteTransaction {
        if self.delegate_vote_tx.is_none() {
            self.delegate_vote_tx.set_default();
        }
        self.delegate_vote_tx.as_mut().unwrap()
    }

    // Take field
    pub fn take_delegate_vote_tx(&mut self) -> OutputTransaction_DelegateVoteTransaction {
        self.delegate_vote_tx.take().unwrap_or_else(|| OutputTransaction_DelegateVoteTransaction::new())
    }

    pub fn get_delegate_vote_tx(&self) -> &OutputTransaction_DelegateVoteTransaction {
        self.delegate_vote_tx.as_ref().unwrap_or_else(|| OutputTransaction_DelegateVoteTransaction::default_instance())
    }

    fn get_delegate_vote_tx_for_reflect(&self) -> &::protobuf::SingularPtrField<OutputTransaction_DelegateVoteTransaction> {
        &self.delegate_vote_tx
    }

    fn mut_delegate_vote_tx_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OutputTransaction_DelegateVoteTransaction> {
        &mut self.delegate_vote_tx
    }

    // .matcha.OutputTransaction.UsernameRegistrationTransaction username_registration_tx = 5;

    pub fn clear_username_registration_tx(&mut self) {
        self.username_registration_tx.clear();
    }

    pub fn has_username_registration_tx(&self) -> bool {
        self.username_registration_tx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_username_registration_tx(&mut self, v: OutputTransaction_UsernameRegistrationTransaction) {
        self.username_registration_tx = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username_registration_tx(&mut self) -> &mut OutputTransaction_UsernameRegistrationTransaction {
        if self.username_registration_tx.is_none() {
            self.username_registration_tx.set_default();
        }
        self.username_registration_tx.as_mut().unwrap()
    }

    // Take field
    pub fn take_username_registration_tx(&mut self) -> OutputTransaction_UsernameRegistrationTransaction {
        self.username_registration_tx.take().unwrap_or_else(|| OutputTransaction_UsernameRegistrationTransaction::new())
    }

    pub fn get_username_registration_tx(&self) -> &OutputTransaction_UsernameRegistrationTransaction {
        self.username_registration_tx.as_ref().unwrap_or_else(|| OutputTransaction_UsernameRegistrationTransaction::default_instance())
    }

    fn get_username_registration_tx_for_reflect(&self) -> &::protobuf::SingularPtrField<OutputTransaction_UsernameRegistrationTransaction> {
        &self.username_registration_tx
    }

    fn mut_username_registration_tx_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OutputTransaction_UsernameRegistrationTransaction> {
        &mut self.username_registration_tx
    }

    // .matcha.OutputTransaction.NewPostTransaction new_post_tx = 6;

    pub fn clear_new_post_tx(&mut self) {
        self.new_post_tx.clear();
    }

    pub fn has_new_post_tx(&self) -> bool {
        self.new_post_tx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_post_tx(&mut self, v: OutputTransaction_NewPostTransaction) {
        self.new_post_tx = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_post_tx(&mut self) -> &mut OutputTransaction_NewPostTransaction {
        if self.new_post_tx.is_none() {
            self.new_post_tx.set_default();
        }
        self.new_post_tx.as_mut().unwrap()
    }

    // Take field
    pub fn take_new_post_tx(&mut self) -> OutputTransaction_NewPostTransaction {
        self.new_post_tx.take().unwrap_or_else(|| OutputTransaction_NewPostTransaction::new())
    }

    pub fn get_new_post_tx(&self) -> &OutputTransaction_NewPostTransaction {
        self.new_post_tx.as_ref().unwrap_or_else(|| OutputTransaction_NewPostTransaction::default_instance())
    }

    fn get_new_post_tx_for_reflect(&self) -> &::protobuf::SingularPtrField<OutputTransaction_NewPostTransaction> {
        &self.new_post_tx
    }

    fn mut_new_post_tx_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OutputTransaction_NewPostTransaction> {
        &mut self.new_post_tx
    }

    // .matcha.OutputTransaction.AvatarUploadTransaction avatar_upload_tx = 7;

    pub fn clear_avatar_upload_tx(&mut self) {
        self.avatar_upload_tx.clear();
    }

    pub fn has_avatar_upload_tx(&self) -> bool {
        self.avatar_upload_tx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avatar_upload_tx(&mut self, v: OutputTransaction_AvatarUploadTransaction) {
        self.avatar_upload_tx = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_avatar_upload_tx(&mut self) -> &mut OutputTransaction_AvatarUploadTransaction {
        if self.avatar_upload_tx.is_none() {
            self.avatar_upload_tx.set_default();
        }
        self.avatar_upload_tx.as_mut().unwrap()
    }

    // Take field
    pub fn take_avatar_upload_tx(&mut self) -> OutputTransaction_AvatarUploadTransaction {
        self.avatar_upload_tx.take().unwrap_or_else(|| OutputTransaction_AvatarUploadTransaction::new())
    }

    pub fn get_avatar_upload_tx(&self) -> &OutputTransaction_AvatarUploadTransaction {
        self.avatar_upload_tx.as_ref().unwrap_or_else(|| OutputTransaction_AvatarUploadTransaction::default_instance())
    }

    fn get_avatar_upload_tx_for_reflect(&self) -> &::protobuf::SingularPtrField<OutputTransaction_AvatarUploadTransaction> {
        &self.avatar_upload_tx
    }

    fn mut_avatar_upload_tx_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OutputTransaction_AvatarUploadTransaction> {
        &mut self.avatar_upload_tx
    }

    // .matcha.OutputTransaction.FavouritePostTransaction favourite_post_tx = 8;

    pub fn clear_favourite_post_tx(&mut self) {
        self.favourite_post_tx.clear();
    }

    pub fn has_favourite_post_tx(&self) -> bool {
        self.favourite_post_tx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_favourite_post_tx(&mut self, v: OutputTransaction_FavouritePostTransaction) {
        self.favourite_post_tx = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_favourite_post_tx(&mut self) -> &mut OutputTransaction_FavouritePostTransaction {
        if self.favourite_post_tx.is_none() {
            self.favourite_post_tx.set_default();
        }
        self.favourite_post_tx.as_mut().unwrap()
    }

    // Take field
    pub fn take_favourite_post_tx(&mut self) -> OutputTransaction_FavouritePostTransaction {
        self.favourite_post_tx.take().unwrap_or_else(|| OutputTransaction_FavouritePostTransaction::new())
    }

    pub fn get_favourite_post_tx(&self) -> &OutputTransaction_FavouritePostTransaction {
        self.favourite_post_tx.as_ref().unwrap_or_else(|| OutputTransaction_FavouritePostTransaction::default_instance())
    }

    fn get_favourite_post_tx_for_reflect(&self) -> &::protobuf::SingularPtrField<OutputTransaction_FavouritePostTransaction> {
        &self.favourite_post_tx
    }

    fn mut_favourite_post_tx_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OutputTransaction_FavouritePostTransaction> {
        &mut self.favourite_post_tx
    }
}

impl ::protobuf::Message for OutputTransaction {
    fn is_initialized(&self) -> bool {
        for v in &self.normal_tx {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.delegate_vote_tx {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.username_registration_tx {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.new_post_tx {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.avatar_upload_tx {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.favourite_post_tx {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.transaction_type = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.amount = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.normal_tx)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.delegate_vote_tx)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.username_registration_tx)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.new_post_tx)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.avatar_upload_tx)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.favourite_post_tx)?;
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
        if self.transaction_type != OutputTransaction_TransactionType::NORMAL_TX {
            my_size += ::protobuf::rt::enum_size(1, self.transaction_type);
        }
        if self.amount != 0 {
            my_size += ::protobuf::rt::value_size(2, self.amount, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.normal_tx.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.delegate_vote_tx.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.username_registration_tx.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.new_post_tx.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.avatar_upload_tx.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.favourite_post_tx.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.transaction_type != OutputTransaction_TransactionType::NORMAL_TX {
            os.write_enum(1, self.transaction_type.value())?;
        }
        if self.amount != 0 {
            os.write_uint64(2, self.amount)?;
        }
        if let Some(ref v) = self.normal_tx.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.delegate_vote_tx.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.username_registration_tx.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.new_post_tx.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.avatar_upload_tx.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.favourite_post_tx.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for OutputTransaction {
    fn new() -> OutputTransaction {
        OutputTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<OutputTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OutputTransaction_TransactionType>>(
                    "transaction_type",
                    OutputTransaction::get_transaction_type_for_reflect,
                    OutputTransaction::mut_transaction_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "amount",
                    OutputTransaction::get_amount_for_reflect,
                    OutputTransaction::mut_amount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OutputTransaction_NormalTransaction>>(
                    "normal_tx",
                    OutputTransaction::get_normal_tx_for_reflect,
                    OutputTransaction::mut_normal_tx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OutputTransaction_DelegateVoteTransaction>>(
                    "delegate_vote_tx",
                    OutputTransaction::get_delegate_vote_tx_for_reflect,
                    OutputTransaction::mut_delegate_vote_tx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OutputTransaction_UsernameRegistrationTransaction>>(
                    "username_registration_tx",
                    OutputTransaction::get_username_registration_tx_for_reflect,
                    OutputTransaction::mut_username_registration_tx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OutputTransaction_NewPostTransaction>>(
                    "new_post_tx",
                    OutputTransaction::get_new_post_tx_for_reflect,
                    OutputTransaction::mut_new_post_tx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OutputTransaction_AvatarUploadTransaction>>(
                    "avatar_upload_tx",
                    OutputTransaction::get_avatar_upload_tx_for_reflect,
                    OutputTransaction::mut_avatar_upload_tx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OutputTransaction_FavouritePostTransaction>>(
                    "favourite_post_tx",
                    OutputTransaction::get_favourite_post_tx_for_reflect,
                    OutputTransaction::mut_favourite_post_tx_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OutputTransaction>(
                    "OutputTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OutputTransaction {
    fn clear(&mut self) {
        self.clear_transaction_type();
        self.clear_amount();
        self.clear_normal_tx();
        self.clear_delegate_vote_tx();
        self.clear_username_registration_tx();
        self.clear_new_post_tx();
        self.clear_avatar_upload_tx();
        self.clear_favourite_post_tx();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OutputTransaction_NormalTransaction {
    // message fields
    pub public_key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OutputTransaction_NormalTransaction {}

impl OutputTransaction_NormalTransaction {
    pub fn new() -> OutputTransaction_NormalTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OutputTransaction_NormalTransaction {
        static mut instance: ::protobuf::lazy::Lazy<OutputTransaction_NormalTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OutputTransaction_NormalTransaction,
        };
        unsafe {
            instance.get(OutputTransaction_NormalTransaction::new)
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
}

impl ::protobuf::Message for OutputTransaction_NormalTransaction {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.public_key.is_empty() {
            os.write_bytes(1, &self.public_key)?;
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

impl ::protobuf::MessageStatic for OutputTransaction_NormalTransaction {
    fn new() -> OutputTransaction_NormalTransaction {
        OutputTransaction_NormalTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<OutputTransaction_NormalTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public_key",
                    OutputTransaction_NormalTransaction::get_public_key_for_reflect,
                    OutputTransaction_NormalTransaction::mut_public_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OutputTransaction_NormalTransaction>(
                    "OutputTransaction_NormalTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OutputTransaction_NormalTransaction {
    fn clear(&mut self) {
        self.clear_public_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputTransaction_NormalTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputTransaction_NormalTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OutputTransaction_DelegateVoteTransaction {
    // message fields
    pub delegate_public_key: ::std::vec::Vec<u8>,
    pub public_key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OutputTransaction_DelegateVoteTransaction {}

impl OutputTransaction_DelegateVoteTransaction {
    pub fn new() -> OutputTransaction_DelegateVoteTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OutputTransaction_DelegateVoteTransaction {
        static mut instance: ::protobuf::lazy::Lazy<OutputTransaction_DelegateVoteTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OutputTransaction_DelegateVoteTransaction,
        };
        unsafe {
            instance.get(OutputTransaction_DelegateVoteTransaction::new)
        }
    }

    // bytes delegate_public_key = 1;

    pub fn clear_delegate_public_key(&mut self) {
        self.delegate_public_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_delegate_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.delegate_public_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delegate_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.delegate_public_key
    }

    // Take field
    pub fn take_delegate_public_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.delegate_public_key, ::std::vec::Vec::new())
    }

    pub fn get_delegate_public_key(&self) -> &[u8] {
        &self.delegate_public_key
    }

    fn get_delegate_public_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.delegate_public_key
    }

    fn mut_delegate_public_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.delegate_public_key
    }

    // bytes public_key = 2;

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
}

impl ::protobuf::Message for OutputTransaction_DelegateVoteTransaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.delegate_public_key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.public_key)?;
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
        if !self.delegate_public_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.delegate_public_key);
        }
        if !self.public_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.public_key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.delegate_public_key.is_empty() {
            os.write_bytes(1, &self.delegate_public_key)?;
        }
        if !self.public_key.is_empty() {
            os.write_bytes(2, &self.public_key)?;
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

impl ::protobuf::MessageStatic for OutputTransaction_DelegateVoteTransaction {
    fn new() -> OutputTransaction_DelegateVoteTransaction {
        OutputTransaction_DelegateVoteTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<OutputTransaction_DelegateVoteTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "delegate_public_key",
                    OutputTransaction_DelegateVoteTransaction::get_delegate_public_key_for_reflect,
                    OutputTransaction_DelegateVoteTransaction::mut_delegate_public_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public_key",
                    OutputTransaction_DelegateVoteTransaction::get_public_key_for_reflect,
                    OutputTransaction_DelegateVoteTransaction::mut_public_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OutputTransaction_DelegateVoteTransaction>(
                    "OutputTransaction_DelegateVoteTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OutputTransaction_DelegateVoteTransaction {
    fn clear(&mut self) {
        self.clear_delegate_public_key();
        self.clear_public_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputTransaction_DelegateVoteTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputTransaction_DelegateVoteTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OutputTransaction_UsernameRegistrationTransaction {
    // message fields
    pub username: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OutputTransaction_UsernameRegistrationTransaction {}

impl OutputTransaction_UsernameRegistrationTransaction {
    pub fn new() -> OutputTransaction_UsernameRegistrationTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OutputTransaction_UsernameRegistrationTransaction {
        static mut instance: ::protobuf::lazy::Lazy<OutputTransaction_UsernameRegistrationTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OutputTransaction_UsernameRegistrationTransaction,
        };
        unsafe {
            instance.get(OutputTransaction_UsernameRegistrationTransaction::new)
        }
    }

    // string username = 1;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.username, ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    fn get_username_for_reflect(&self) -> &::std::string::String {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }
}

impl ::protobuf::Message for OutputTransaction_UsernameRegistrationTransaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.username)?;
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
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.username);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.username.is_empty() {
            os.write_string(1, &self.username)?;
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

impl ::protobuf::MessageStatic for OutputTransaction_UsernameRegistrationTransaction {
    fn new() -> OutputTransaction_UsernameRegistrationTransaction {
        OutputTransaction_UsernameRegistrationTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<OutputTransaction_UsernameRegistrationTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    OutputTransaction_UsernameRegistrationTransaction::get_username_for_reflect,
                    OutputTransaction_UsernameRegistrationTransaction::mut_username_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OutputTransaction_UsernameRegistrationTransaction>(
                    "OutputTransaction_UsernameRegistrationTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OutputTransaction_UsernameRegistrationTransaction {
    fn clear(&mut self) {
        self.clear_username();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputTransaction_UsernameRegistrationTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputTransaction_UsernameRegistrationTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OutputTransaction_NewPostTransaction {
    // message fields
    pub content: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OutputTransaction_NewPostTransaction {}

impl OutputTransaction_NewPostTransaction {
    pub fn new() -> OutputTransaction_NewPostTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OutputTransaction_NewPostTransaction {
        static mut instance: ::protobuf::lazy::Lazy<OutputTransaction_NewPostTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OutputTransaction_NewPostTransaction,
        };
        unsafe {
            instance.get(OutputTransaction_NewPostTransaction::new)
        }
    }

    // bytes content = 1;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.content
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.content, ::std::vec::Vec::new())
    }

    pub fn get_content(&self) -> &[u8] {
        &self.content
    }

    fn get_content_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.content
    }
}

impl ::protobuf::Message for OutputTransaction_NewPostTransaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.content)?;
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
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.content);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.content.is_empty() {
            os.write_bytes(1, &self.content)?;
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

impl ::protobuf::MessageStatic for OutputTransaction_NewPostTransaction {
    fn new() -> OutputTransaction_NewPostTransaction {
        OutputTransaction_NewPostTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<OutputTransaction_NewPostTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "content",
                    OutputTransaction_NewPostTransaction::get_content_for_reflect,
                    OutputTransaction_NewPostTransaction::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OutputTransaction_NewPostTransaction>(
                    "OutputTransaction_NewPostTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OutputTransaction_NewPostTransaction {
    fn clear(&mut self) {
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputTransaction_NewPostTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputTransaction_NewPostTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OutputTransaction_AvatarUploadTransaction {
    // message fields
    pub image_data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OutputTransaction_AvatarUploadTransaction {}

impl OutputTransaction_AvatarUploadTransaction {
    pub fn new() -> OutputTransaction_AvatarUploadTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OutputTransaction_AvatarUploadTransaction {
        static mut instance: ::protobuf::lazy::Lazy<OutputTransaction_AvatarUploadTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OutputTransaction_AvatarUploadTransaction,
        };
        unsafe {
            instance.get(OutputTransaction_AvatarUploadTransaction::new)
        }
    }

    // bytes image_data = 1;

    pub fn clear_image_data(&mut self) {
        self.image_data.clear();
    }

    // Param is passed by value, moved
    pub fn set_image_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.image_data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.image_data
    }

    // Take field
    pub fn take_image_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.image_data, ::std::vec::Vec::new())
    }

    pub fn get_image_data(&self) -> &[u8] {
        &self.image_data
    }

    fn get_image_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.image_data
    }

    fn mut_image_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.image_data
    }
}

impl ::protobuf::Message for OutputTransaction_AvatarUploadTransaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.image_data)?;
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
        if !self.image_data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.image_data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.image_data.is_empty() {
            os.write_bytes(1, &self.image_data)?;
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

impl ::protobuf::MessageStatic for OutputTransaction_AvatarUploadTransaction {
    fn new() -> OutputTransaction_AvatarUploadTransaction {
        OutputTransaction_AvatarUploadTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<OutputTransaction_AvatarUploadTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "image_data",
                    OutputTransaction_AvatarUploadTransaction::get_image_data_for_reflect,
                    OutputTransaction_AvatarUploadTransaction::mut_image_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OutputTransaction_AvatarUploadTransaction>(
                    "OutputTransaction_AvatarUploadTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OutputTransaction_AvatarUploadTransaction {
    fn clear(&mut self) {
        self.clear_image_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputTransaction_AvatarUploadTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputTransaction_AvatarUploadTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OutputTransaction_FavouritePostTransaction {
    // message fields
    pub tx_id: ::std::vec::Vec<u8>,
    pub txout_index: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OutputTransaction_FavouritePostTransaction {}

impl OutputTransaction_FavouritePostTransaction {
    pub fn new() -> OutputTransaction_FavouritePostTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OutputTransaction_FavouritePostTransaction {
        static mut instance: ::protobuf::lazy::Lazy<OutputTransaction_FavouritePostTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OutputTransaction_FavouritePostTransaction,
        };
        unsafe {
            instance.get(OutputTransaction_FavouritePostTransaction::new)
        }
    }

    // bytes tx_id = 1;

    pub fn clear_tx_id(&mut self) {
        self.tx_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.tx_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx_id
    }

    // Take field
    pub fn take_tx_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tx_id, ::std::vec::Vec::new())
    }

    pub fn get_tx_id(&self) -> &[u8] {
        &self.tx_id
    }

    fn get_tx_id_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.tx_id
    }

    fn mut_tx_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tx_id
    }

    // uint32 txout_index = 2;

    pub fn clear_txout_index(&mut self) {
        self.txout_index = 0;
    }

    // Param is passed by value, moved
    pub fn set_txout_index(&mut self, v: u32) {
        self.txout_index = v;
    }

    pub fn get_txout_index(&self) -> u32 {
        self.txout_index
    }

    fn get_txout_index_for_reflect(&self) -> &u32 {
        &self.txout_index
    }

    fn mut_txout_index_for_reflect(&mut self) -> &mut u32 {
        &mut self.txout_index
    }
}

impl ::protobuf::Message for OutputTransaction_FavouritePostTransaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tx_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.txout_index = tmp;
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
        if !self.tx_id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.tx_id);
        }
        if self.txout_index != 0 {
            my_size += ::protobuf::rt::value_size(2, self.txout_index, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx_id.is_empty() {
            os.write_bytes(1, &self.tx_id)?;
        }
        if self.txout_index != 0 {
            os.write_uint32(2, self.txout_index)?;
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

impl ::protobuf::MessageStatic for OutputTransaction_FavouritePostTransaction {
    fn new() -> OutputTransaction_FavouritePostTransaction {
        OutputTransaction_FavouritePostTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<OutputTransaction_FavouritePostTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tx_id",
                    OutputTransaction_FavouritePostTransaction::get_tx_id_for_reflect,
                    OutputTransaction_FavouritePostTransaction::mut_tx_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "txout_index",
                    OutputTransaction_FavouritePostTransaction::get_txout_index_for_reflect,
                    OutputTransaction_FavouritePostTransaction::mut_txout_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OutputTransaction_FavouritePostTransaction>(
                    "OutputTransaction_FavouritePostTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OutputTransaction_FavouritePostTransaction {
    fn clear(&mut self) {
        self.clear_tx_id();
        self.clear_txout_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputTransaction_FavouritePostTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputTransaction_FavouritePostTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OutputTransaction_TransactionType {
    NORMAL_TX = 0,
    DELEGATE_VOTE_TX = 1,
    USERNAME_REGISTRATION_TX = 2,
    NEW_POST_TX = 3,
    AVATAR_UPLOAD_TX = 4,
    FAVOURITE_POST_TX = 5,
}

impl ::protobuf::ProtobufEnum for OutputTransaction_TransactionType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OutputTransaction_TransactionType> {
        match value {
            0 => ::std::option::Option::Some(OutputTransaction_TransactionType::NORMAL_TX),
            1 => ::std::option::Option::Some(OutputTransaction_TransactionType::DELEGATE_VOTE_TX),
            2 => ::std::option::Option::Some(OutputTransaction_TransactionType::USERNAME_REGISTRATION_TX),
            3 => ::std::option::Option::Some(OutputTransaction_TransactionType::NEW_POST_TX),
            4 => ::std::option::Option::Some(OutputTransaction_TransactionType::AVATAR_UPLOAD_TX),
            5 => ::std::option::Option::Some(OutputTransaction_TransactionType::FAVOURITE_POST_TX),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OutputTransaction_TransactionType] = &[
            OutputTransaction_TransactionType::NORMAL_TX,
            OutputTransaction_TransactionType::DELEGATE_VOTE_TX,
            OutputTransaction_TransactionType::USERNAME_REGISTRATION_TX,
            OutputTransaction_TransactionType::NEW_POST_TX,
            OutputTransaction_TransactionType::AVATAR_UPLOAD_TX,
            OutputTransaction_TransactionType::FAVOURITE_POST_TX,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<OutputTransaction_TransactionType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OutputTransaction_TransactionType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OutputTransaction_TransactionType {
}

impl ::std::default::Default for OutputTransaction_TransactionType {
    fn default() -> Self {
        OutputTransaction_TransactionType::NORMAL_TX
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputTransaction_TransactionType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Transaction {
    // message fields
    pub id: ::std::vec::Vec<u8>,
    pub txins: ::protobuf::RepeatedField<InputTransaction>,
    pub txouts: ::protobuf::RepeatedField<OutputTransaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Transaction {}

impl Transaction {
    pub fn new() -> Transaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Transaction {
        static mut instance: ::protobuf::lazy::Lazy<Transaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Transaction,
        };
        unsafe {
            instance.get(Transaction::new)
        }
    }

    // bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.id, ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.id
    }

    // repeated .matcha.InputTransaction txins = 2;

    pub fn clear_txins(&mut self) {
        self.txins.clear();
    }

    // Param is passed by value, moved
    pub fn set_txins(&mut self, v: ::protobuf::RepeatedField<InputTransaction>) {
        self.txins = v;
    }

    // Mutable pointer to the field.
    pub fn mut_txins(&mut self) -> &mut ::protobuf::RepeatedField<InputTransaction> {
        &mut self.txins
    }

    // Take field
    pub fn take_txins(&mut self) -> ::protobuf::RepeatedField<InputTransaction> {
        ::std::mem::replace(&mut self.txins, ::protobuf::RepeatedField::new())
    }

    pub fn get_txins(&self) -> &[InputTransaction] {
        &self.txins
    }

    fn get_txins_for_reflect(&self) -> &::protobuf::RepeatedField<InputTransaction> {
        &self.txins
    }

    fn mut_txins_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<InputTransaction> {
        &mut self.txins
    }

    // repeated .matcha.OutputTransaction txouts = 3;

    pub fn clear_txouts(&mut self) {
        self.txouts.clear();
    }

    // Param is passed by value, moved
    pub fn set_txouts(&mut self, v: ::protobuf::RepeatedField<OutputTransaction>) {
        self.txouts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_txouts(&mut self) -> &mut ::protobuf::RepeatedField<OutputTransaction> {
        &mut self.txouts
    }

    // Take field
    pub fn take_txouts(&mut self) -> ::protobuf::RepeatedField<OutputTransaction> {
        ::std::mem::replace(&mut self.txouts, ::protobuf::RepeatedField::new())
    }

    pub fn get_txouts(&self) -> &[OutputTransaction] {
        &self.txouts
    }

    fn get_txouts_for_reflect(&self) -> &::protobuf::RepeatedField<OutputTransaction> {
        &self.txouts
    }

    fn mut_txouts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OutputTransaction> {
        &mut self.txouts
    }
}

impl ::protobuf::Message for Transaction {
    fn is_initialized(&self) -> bool {
        for v in &self.txins {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.txouts {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.txins)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.txouts)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.id);
        }
        for value in &self.txins {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.txouts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_bytes(1, &self.id)?;
        }
        for v in &self.txins {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.txouts {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Transaction {
    fn new() -> Transaction {
        Transaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<Transaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "id",
                    Transaction::get_id_for_reflect,
                    Transaction::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<InputTransaction>>(
                    "txins",
                    Transaction::get_txins_for_reflect,
                    Transaction::mut_txins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OutputTransaction>>(
                    "txouts",
                    Transaction::get_txouts_for_reflect,
                    Transaction::mut_txouts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Transaction>(
                    "Transaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Transaction {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_txins();
        self.clear_txouts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Transaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Block {
    // message fields
    pub version: u32,
    pub public_key: ::std::vec::Vec<u8>,
    pub previous_hash: ::std::vec::Vec<u8>,
    pub transaction_root: ::std::vec::Vec<u8>,
    pub timestamp: u64,
    pub transactions: ::protobuf::RepeatedField<Transaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Block {}

impl Block {
    pub fn new() -> Block {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Block {
        static mut instance: ::protobuf::lazy::Lazy<Block> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Block,
        };
        unsafe {
            instance.get(Block::new)
        }
    }

    // uint32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = v;
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u32 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u32 {
        &mut self.version
    }

    // bytes public_key = 2;

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

    // bytes previous_hash = 3;

    pub fn clear_previous_hash(&mut self) {
        self.previous_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_previous_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.previous_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_previous_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.previous_hash
    }

    // Take field
    pub fn take_previous_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.previous_hash, ::std::vec::Vec::new())
    }

    pub fn get_previous_hash(&self) -> &[u8] {
        &self.previous_hash
    }

    fn get_previous_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.previous_hash
    }

    fn mut_previous_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.previous_hash
    }

    // bytes transaction_root = 4;

    pub fn clear_transaction_root(&mut self) {
        self.transaction_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_transaction_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.transaction_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transaction_root
    }

    // Take field
    pub fn take_transaction_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.transaction_root, ::std::vec::Vec::new())
    }

    pub fn get_transaction_root(&self) -> &[u8] {
        &self.transaction_root
    }

    fn get_transaction_root_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.transaction_root
    }

    fn mut_transaction_root_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transaction_root
    }

    // uint64 timestamp = 5;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &u64 {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut u64 {
        &mut self.timestamp
    }

    // repeated .matcha.Transaction transactions = 6;

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions(&mut self, v: ::protobuf::RepeatedField<Transaction>) {
        self.transactions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transactions(&mut self) -> &mut ::protobuf::RepeatedField<Transaction> {
        &mut self.transactions
    }

    // Take field
    pub fn take_transactions(&mut self) -> ::protobuf::RepeatedField<Transaction> {
        ::std::mem::replace(&mut self.transactions, ::protobuf::RepeatedField::new())
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    fn get_transactions_for_reflect(&self) -> &::protobuf::RepeatedField<Transaction> {
        &self.transactions
    }

    fn mut_transactions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Transaction> {
        &mut self.transactions
    }
}

impl ::protobuf::Message for Block {
    fn is_initialized(&self) -> bool {
        for v in &self.transactions {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.public_key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.previous_hash)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.transaction_root)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.transactions)?;
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
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(1, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.public_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.public_key);
        }
        if !self.previous_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.previous_hash);
        }
        if !self.transaction_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.transaction_root);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(5, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.transactions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.version != 0 {
            os.write_uint32(1, self.version)?;
        }
        if !self.public_key.is_empty() {
            os.write_bytes(2, &self.public_key)?;
        }
        if !self.previous_hash.is_empty() {
            os.write_bytes(3, &self.previous_hash)?;
        }
        if !self.transaction_root.is_empty() {
            os.write_bytes(4, &self.transaction_root)?;
        }
        if self.timestamp != 0 {
            os.write_uint64(5, self.timestamp)?;
        }
        for v in &self.transactions {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Block {
    fn new() -> Block {
        Block::new()
    }

    fn descriptor_static(_: ::std::option::Option<Block>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    Block::get_version_for_reflect,
                    Block::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public_key",
                    Block::get_public_key_for_reflect,
                    Block::mut_public_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "previous_hash",
                    Block::get_previous_hash_for_reflect,
                    Block::mut_previous_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "transaction_root",
                    Block::get_transaction_root_for_reflect,
                    Block::mut_transaction_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    Block::get_timestamp_for_reflect,
                    Block::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Transaction>>(
                    "transactions",
                    Block::get_transactions_for_reflect,
                    Block::mut_transactions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Block>(
                    "Block",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Block {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_public_key();
        self.clear_previous_hash();
        self.clear_transaction_root();
        self.clear_timestamp();
        self.clear_transactions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Block {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignedBlock {
    // message fields
    pub signature: ::std::vec::Vec<u8>,
    pub block: ::protobuf::SingularPtrField<Block>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SignedBlock {}

impl SignedBlock {
    pub fn new() -> SignedBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignedBlock {
        static mut instance: ::protobuf::lazy::Lazy<SignedBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignedBlock,
        };
        unsafe {
            instance.get(SignedBlock::new)
        }
    }

    // bytes signature = 1;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.signature, ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        &self.signature
    }

    fn get_signature_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // .matcha.Block block = 2;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: Block) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block(&mut self) -> &mut Block {
        if self.block.is_none() {
            self.block.set_default();
        }
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> Block {
        self.block.take().unwrap_or_else(|| Block::new())
    }

    pub fn get_block(&self) -> &Block {
        self.block.as_ref().unwrap_or_else(|| Block::default_instance())
    }

    fn get_block_for_reflect(&self) -> &::protobuf::SingularPtrField<Block> {
        &self.block
    }

    fn mut_block_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Block> {
        &mut self.block
    }
}

impl ::protobuf::Message for SignedBlock {
    fn is_initialized(&self) -> bool {
        for v in &self.block {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signature)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.block)?;
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
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.signature);
        }
        if let Some(ref v) = self.block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.signature.is_empty() {
            os.write_bytes(1, &self.signature)?;
        }
        if let Some(ref v) = self.block.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for SignedBlock {
    fn new() -> SignedBlock {
        SignedBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<SignedBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    SignedBlock::get_signature_for_reflect,
                    SignedBlock::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Block>>(
                    "block",
                    SignedBlock::get_block_for_reflect,
                    SignedBlock::mut_block_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignedBlock>(
                    "SignedBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignedBlock {
    fn clear(&mut self) {
        self.clear_signature();
        self.clear_block();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignedBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignedBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FullBlock {
    // message fields
    pub hash: ::std::vec::Vec<u8>,
    pub signed_block: ::protobuf::SingularPtrField<SignedBlock>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FullBlock {}

impl FullBlock {
    pub fn new() -> FullBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FullBlock {
        static mut instance: ::protobuf::lazy::Lazy<FullBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FullBlock,
        };
        unsafe {
            instance.get(FullBlock::new)
        }
    }

    // bytes hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // .matcha.SignedBlock signed_block = 2;

    pub fn clear_signed_block(&mut self) {
        self.signed_block.clear();
    }

    pub fn has_signed_block(&self) -> bool {
        self.signed_block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signed_block(&mut self, v: SignedBlock) {
        self.signed_block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signed_block(&mut self) -> &mut SignedBlock {
        if self.signed_block.is_none() {
            self.signed_block.set_default();
        }
        self.signed_block.as_mut().unwrap()
    }

    // Take field
    pub fn take_signed_block(&mut self) -> SignedBlock {
        self.signed_block.take().unwrap_or_else(|| SignedBlock::new())
    }

    pub fn get_signed_block(&self) -> &SignedBlock {
        self.signed_block.as_ref().unwrap_or_else(|| SignedBlock::default_instance())
    }

    fn get_signed_block_for_reflect(&self) -> &::protobuf::SingularPtrField<SignedBlock> {
        &self.signed_block
    }

    fn mut_signed_block_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SignedBlock> {
        &mut self.signed_block
    }
}

impl ::protobuf::Message for FullBlock {
    fn is_initialized(&self) -> bool {
        for v in &self.signed_block {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.signed_block)?;
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
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.hash);
        }
        if let Some(ref v) = self.signed_block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.hash.is_empty() {
            os.write_bytes(1, &self.hash)?;
        }
        if let Some(ref v) = self.signed_block.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for FullBlock {
    fn new() -> FullBlock {
        FullBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<FullBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    FullBlock::get_hash_for_reflect,
                    FullBlock::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SignedBlock>>(
                    "signed_block",
                    FullBlock::get_signed_block_for_reflect,
                    FullBlock::mut_signed_block_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FullBlock>(
                    "FullBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FullBlock {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_signed_block();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FullBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FullBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    pub network_type: Message_NetworkType,
    pub field_type: Message_Type,
    pub request: ::protobuf::SingularPtrField<Request>,
    pub response: ::protobuf::SingularPtrField<Response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message {}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(Message::new)
        }
    }

    // .matcha.Message.NetworkType network_type = 1;

    pub fn clear_network_type(&mut self) {
        self.network_type = Message_NetworkType::PRODNET;
    }

    // Param is passed by value, moved
    pub fn set_network_type(&mut self, v: Message_NetworkType) {
        self.network_type = v;
    }

    pub fn get_network_type(&self) -> Message_NetworkType {
        self.network_type
    }

    fn get_network_type_for_reflect(&self) -> &Message_NetworkType {
        &self.network_type
    }

    fn mut_network_type_for_reflect(&mut self) -> &mut Message_NetworkType {
        &mut self.network_type
    }

    // .matcha.Message.Type type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = Message_Type::REQUEST;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Message_Type) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> Message_Type {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &Message_Type {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut Message_Type {
        &mut self.field_type
    }

    // .matcha.Request request = 3;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: Request) {
        self.request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request(&mut self) -> &mut Request {
        if self.request.is_none() {
            self.request.set_default();
        }
        self.request.as_mut().unwrap()
    }

    // Take field
    pub fn take_request(&mut self) -> Request {
        self.request.take().unwrap_or_else(|| Request::new())
    }

    pub fn get_request(&self) -> &Request {
        self.request.as_ref().unwrap_or_else(|| Request::default_instance())
    }

    fn get_request_for_reflect(&self) -> &::protobuf::SingularPtrField<Request> {
        &self.request
    }

    fn mut_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Request> {
        &mut self.request
    }

    // .matcha.Response response = 4;

    pub fn clear_response(&mut self) {
        self.response.clear();
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: Response) {
        self.response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response(&mut self) -> &mut Response {
        if self.response.is_none() {
            self.response.set_default();
        }
        self.response.as_mut().unwrap()
    }

    // Take field
    pub fn take_response(&mut self) -> Response {
        self.response.take().unwrap_or_else(|| Response::new())
    }

    pub fn get_response(&self) -> &Response {
        self.response.as_ref().unwrap_or_else(|| Response::default_instance())
    }

    fn get_response_for_reflect(&self) -> &::protobuf::SingularPtrField<Response> {
        &self.response
    }

    fn mut_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Response> {
        &mut self.response
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        for v in &self.request {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.network_type = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.request)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response)?;
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
        if self.network_type != Message_NetworkType::PRODNET {
            my_size += ::protobuf::rt::enum_size(1, self.network_type);
        }
        if self.field_type != Message_Type::REQUEST {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        }
        if let Some(ref v) = self.request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.network_type != Message_NetworkType::PRODNET {
            os.write_enum(1, self.network_type.value())?;
        }
        if self.field_type != Message_Type::REQUEST {
            os.write_enum(2, self.field_type.value())?;
        }
        if let Some(ref v) = self.request.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.response.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Message_NetworkType>>(
                    "network_type",
                    Message::get_network_type_for_reflect,
                    Message::mut_network_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Message_Type>>(
                    "type",
                    Message::get_field_type_for_reflect,
                    Message::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Request>>(
                    "request",
                    Message::get_request_for_reflect,
                    Message::mut_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Response>>(
                    "response",
                    Message::get_response_for_reflect,
                    Message::mut_response_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_network_type();
        self.clear_field_type();
        self.clear_request();
        self.clear_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Message_Type {
    REQUEST = 0,
    RESPONSE = 1,
}

impl ::protobuf::ProtobufEnum for Message_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Message_Type> {
        match value {
            0 => ::std::option::Option::Some(Message_Type::REQUEST),
            1 => ::std::option::Option::Some(Message_Type::RESPONSE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Message_Type] = &[
            Message_Type::REQUEST,
            Message_Type::RESPONSE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Message_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Message_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Message_Type {
}

impl ::std::default::Default for Message_Type {
    fn default() -> Self {
        Message_Type::REQUEST
    }
}

impl ::protobuf::reflect::ProtobufValue for Message_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Message_NetworkType {
    PRODNET = 0,
    TESTNET = 1,
}

impl ::protobuf::ProtobufEnum for Message_NetworkType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Message_NetworkType> {
        match value {
            0 => ::std::option::Option::Some(Message_NetworkType::PRODNET),
            1 => ::std::option::Option::Some(Message_NetworkType::TESTNET),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Message_NetworkType] = &[
            Message_NetworkType::PRODNET,
            Message_NetworkType::TESTNET,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Message_NetworkType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Message_NetworkType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Message_NetworkType {
}

impl ::std::default::Default for Message_NetworkType {
    fn default() -> Self {
        Message_NetworkType::PRODNET
    }
}

impl ::protobuf::reflect::ProtobufValue for Message_NetworkType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    pub field_type: Request_Type,
    pub ping_request: ::protobuf::SingularPtrField<PingRequest>,
    pub peer_list_request: ::protobuf::SingularPtrField<PeerListRequest>,
    pub version_request: ::protobuf::SingularPtrField<Version>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }

    // .matcha.Request.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = Request_Type::VERSION_REQUEST;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Request_Type) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> Request_Type {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &Request_Type {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut Request_Type {
        &mut self.field_type
    }

    // .matcha.PingRequest ping_request = 2;

    pub fn clear_ping_request(&mut self) {
        self.ping_request.clear();
    }

    pub fn has_ping_request(&self) -> bool {
        self.ping_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_request(&mut self, v: PingRequest) {
        self.ping_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping_request(&mut self) -> &mut PingRequest {
        if self.ping_request.is_none() {
            self.ping_request.set_default();
        }
        self.ping_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping_request(&mut self) -> PingRequest {
        self.ping_request.take().unwrap_or_else(|| PingRequest::new())
    }

    pub fn get_ping_request(&self) -> &PingRequest {
        self.ping_request.as_ref().unwrap_or_else(|| PingRequest::default_instance())
    }

    fn get_ping_request_for_reflect(&self) -> &::protobuf::SingularPtrField<PingRequest> {
        &self.ping_request
    }

    fn mut_ping_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PingRequest> {
        &mut self.ping_request
    }

    // .matcha.PeerListRequest peer_list_request = 3;

    pub fn clear_peer_list_request(&mut self) {
        self.peer_list_request.clear();
    }

    pub fn has_peer_list_request(&self) -> bool {
        self.peer_list_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer_list_request(&mut self, v: PeerListRequest) {
        self.peer_list_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer_list_request(&mut self) -> &mut PeerListRequest {
        if self.peer_list_request.is_none() {
            self.peer_list_request.set_default();
        }
        self.peer_list_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer_list_request(&mut self) -> PeerListRequest {
        self.peer_list_request.take().unwrap_or_else(|| PeerListRequest::new())
    }

    pub fn get_peer_list_request(&self) -> &PeerListRequest {
        self.peer_list_request.as_ref().unwrap_or_else(|| PeerListRequest::default_instance())
    }

    fn get_peer_list_request_for_reflect(&self) -> &::protobuf::SingularPtrField<PeerListRequest> {
        &self.peer_list_request
    }

    fn mut_peer_list_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PeerListRequest> {
        &mut self.peer_list_request
    }

    // .matcha.Version version_request = 4;

    pub fn clear_version_request(&mut self) {
        self.version_request.clear();
    }

    pub fn has_version_request(&self) -> bool {
        self.version_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version_request(&mut self, v: Version) {
        self.version_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version_request(&mut self) -> &mut Version {
        if self.version_request.is_none() {
            self.version_request.set_default();
        }
        self.version_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_version_request(&mut self) -> Version {
        self.version_request.take().unwrap_or_else(|| Version::new())
    }

    pub fn get_version_request(&self) -> &Version {
        self.version_request.as_ref().unwrap_or_else(|| Version::default_instance())
    }

    fn get_version_request_for_reflect(&self) -> &::protobuf::SingularPtrField<Version> {
        &self.version_request
    }

    fn mut_version_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Version> {
        &mut self.version_request
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        for v in &self.ping_request {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.peer_list_request {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.version_request {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping_request)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer_list_request)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.version_request)?;
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
        if self.field_type != Request_Type::VERSION_REQUEST {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if let Some(ref v) = self.ping_request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.peer_list_request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.version_request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != Request_Type::VERSION_REQUEST {
            os.write_enum(1, self.field_type.value())?;
        }
        if let Some(ref v) = self.ping_request.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.peer_list_request.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.version_request.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Request_Type>>(
                    "type",
                    Request::get_field_type_for_reflect,
                    Request::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PingRequest>>(
                    "ping_request",
                    Request::get_ping_request_for_reflect,
                    Request::mut_ping_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PeerListRequest>>(
                    "peer_list_request",
                    Request::get_peer_list_request_for_reflect,
                    Request::mut_peer_list_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Version>>(
                    "version_request",
                    Request::get_version_request_for_reflect,
                    Request::mut_version_request_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_ping_request();
        self.clear_peer_list_request();
        self.clear_version_request();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Request_Type {
    VERSION_REQUEST = 0,
    PING_REQUEST = 1,
    PEER_LIST_REQUEST = 2,
}

impl ::protobuf::ProtobufEnum for Request_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Request_Type> {
        match value {
            0 => ::std::option::Option::Some(Request_Type::VERSION_REQUEST),
            1 => ::std::option::Option::Some(Request_Type::PING_REQUEST),
            2 => ::std::option::Option::Some(Request_Type::PEER_LIST_REQUEST),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Request_Type] = &[
            Request_Type::VERSION_REQUEST,
            Request_Type::PING_REQUEST,
            Request_Type::PEER_LIST_REQUEST,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Request_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Request_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Request_Type {
}

impl ::std::default::Default for Request_Type {
    fn default() -> Self {
        Request_Type::VERSION_REQUEST
    }
}

impl ::protobuf::reflect::ProtobufValue for Request_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    pub status: Response_Status,
    pub field_type: Response_Type,
    pub description: ::std::string::String,
    pub ping_response: ::protobuf::SingularPtrField<PingResponse>,
    pub peer_list_response: ::protobuf::SingularPtrField<PeerListResponse>,
    pub version_response: ::protobuf::SingularPtrField<Version>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }

    // .matcha.Response.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = Response_Status::INVALID_REQUEST;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Response_Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> Response_Status {
        self.status
    }

    fn get_status_for_reflect(&self) -> &Response_Status {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut Response_Status {
        &mut self.status
    }

    // .matcha.Response.Type type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = Response_Type::DESCRIPTION_ONLY;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Response_Type) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> Response_Type {
        self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &Response_Type {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut Response_Type {
        &mut self.field_type
    }

    // string description = 3;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // .matcha.PingResponse ping_response = 4;

    pub fn clear_ping_response(&mut self) {
        self.ping_response.clear();
    }

    pub fn has_ping_response(&self) -> bool {
        self.ping_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_response(&mut self, v: PingResponse) {
        self.ping_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping_response(&mut self) -> &mut PingResponse {
        if self.ping_response.is_none() {
            self.ping_response.set_default();
        }
        self.ping_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping_response(&mut self) -> PingResponse {
        self.ping_response.take().unwrap_or_else(|| PingResponse::new())
    }

    pub fn get_ping_response(&self) -> &PingResponse {
        self.ping_response.as_ref().unwrap_or_else(|| PingResponse::default_instance())
    }

    fn get_ping_response_for_reflect(&self) -> &::protobuf::SingularPtrField<PingResponse> {
        &self.ping_response
    }

    fn mut_ping_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PingResponse> {
        &mut self.ping_response
    }

    // .matcha.PeerListResponse peer_list_response = 5;

    pub fn clear_peer_list_response(&mut self) {
        self.peer_list_response.clear();
    }

    pub fn has_peer_list_response(&self) -> bool {
        self.peer_list_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer_list_response(&mut self, v: PeerListResponse) {
        self.peer_list_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer_list_response(&mut self) -> &mut PeerListResponse {
        if self.peer_list_response.is_none() {
            self.peer_list_response.set_default();
        }
        self.peer_list_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer_list_response(&mut self) -> PeerListResponse {
        self.peer_list_response.take().unwrap_or_else(|| PeerListResponse::new())
    }

    pub fn get_peer_list_response(&self) -> &PeerListResponse {
        self.peer_list_response.as_ref().unwrap_or_else(|| PeerListResponse::default_instance())
    }

    fn get_peer_list_response_for_reflect(&self) -> &::protobuf::SingularPtrField<PeerListResponse> {
        &self.peer_list_response
    }

    fn mut_peer_list_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PeerListResponse> {
        &mut self.peer_list_response
    }

    // .matcha.Version version_response = 6;

    pub fn clear_version_response(&mut self) {
        self.version_response.clear();
    }

    pub fn has_version_response(&self) -> bool {
        self.version_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version_response(&mut self, v: Version) {
        self.version_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version_response(&mut self) -> &mut Version {
        if self.version_response.is_none() {
            self.version_response.set_default();
        }
        self.version_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_version_response(&mut self) -> Version {
        self.version_response.take().unwrap_or_else(|| Version::new())
    }

    pub fn get_version_response(&self) -> &Version {
        self.version_response.as_ref().unwrap_or_else(|| Version::default_instance())
    }

    fn get_version_response_for_reflect(&self) -> &::protobuf::SingularPtrField<Version> {
        &self.version_response
    }

    fn mut_version_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Version> {
        &mut self.version_response
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        for v in &self.ping_response {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.peer_list_response {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.version_response {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping_response)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer_list_response)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.version_response)?;
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
        if self.status != Response_Status::INVALID_REQUEST {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        if self.field_type != Response_Type::DESCRIPTION_ONLY {
            my_size += ::protobuf::rt::enum_size(2, self.field_type);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.description);
        }
        if let Some(ref v) = self.ping_response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.peer_list_response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.version_response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status != Response_Status::INVALID_REQUEST {
            os.write_enum(1, self.status.value())?;
        }
        if self.field_type != Response_Type::DESCRIPTION_ONLY {
            os.write_enum(2, self.field_type.value())?;
        }
        if !self.description.is_empty() {
            os.write_string(3, &self.description)?;
        }
        if let Some(ref v) = self.ping_response.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.peer_list_response.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.version_response.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Response_Status>>(
                    "status",
                    Response::get_status_for_reflect,
                    Response::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Response_Type>>(
                    "type",
                    Response::get_field_type_for_reflect,
                    Response::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    Response::get_description_for_reflect,
                    Response::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PingResponse>>(
                    "ping_response",
                    Response::get_ping_response_for_reflect,
                    Response::mut_ping_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PeerListResponse>>(
                    "peer_list_response",
                    Response::get_peer_list_response_for_reflect,
                    Response::mut_peer_list_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Version>>(
                    "version_response",
                    Response::get_version_response_for_reflect,
                    Response::mut_version_response_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_field_type();
        self.clear_description();
        self.clear_ping_response();
        self.clear_peer_list_response();
        self.clear_version_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Response_Status {
    INVALID_REQUEST = 0,
    ACK = 1,
    TOO_MANY_REQUESTS = 2,
    INTERNAL_SERVER_ERROR = 3,
}

impl ::protobuf::ProtobufEnum for Response_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Response_Status> {
        match value {
            0 => ::std::option::Option::Some(Response_Status::INVALID_REQUEST),
            1 => ::std::option::Option::Some(Response_Status::ACK),
            2 => ::std::option::Option::Some(Response_Status::TOO_MANY_REQUESTS),
            3 => ::std::option::Option::Some(Response_Status::INTERNAL_SERVER_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Response_Status] = &[
            Response_Status::INVALID_REQUEST,
            Response_Status::ACK,
            Response_Status::TOO_MANY_REQUESTS,
            Response_Status::INTERNAL_SERVER_ERROR,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Response_Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Response_Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Response_Status {
}

impl ::std::default::Default for Response_Status {
    fn default() -> Self {
        Response_Status::INVALID_REQUEST
    }
}

impl ::protobuf::reflect::ProtobufValue for Response_Status {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Response_Type {
    DESCRIPTION_ONLY = 0,
    VERSION_RESPONSE = 1,
    PING_RESPONSE = 2,
    PEER_LIST_RESPONSE = 3,
}

impl ::protobuf::ProtobufEnum for Response_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Response_Type> {
        match value {
            0 => ::std::option::Option::Some(Response_Type::DESCRIPTION_ONLY),
            1 => ::std::option::Option::Some(Response_Type::VERSION_RESPONSE),
            2 => ::std::option::Option::Some(Response_Type::PING_RESPONSE),
            3 => ::std::option::Option::Some(Response_Type::PEER_LIST_RESPONSE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Response_Type] = &[
            Response_Type::DESCRIPTION_ONLY,
            Response_Type::VERSION_RESPONSE,
            Response_Type::PING_RESPONSE,
            Response_Type::PEER_LIST_RESPONSE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Response_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Response_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Response_Type {
}

impl ::std::default::Default for Response_Type {
    fn default() -> Self {
        Response_Type::DESCRIPTION_ONLY
    }
}

impl ::protobuf::reflect::ProtobufValue for Response_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Version {
    // message fields
    pub version_number: u32,
    pub timestamp: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Version {}

impl Version {
    pub fn new() -> Version {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Version {
        static mut instance: ::protobuf::lazy::Lazy<Version> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Version,
        };
        unsafe {
            instance.get(Version::new)
        }
    }

    // uint32 version_number = 1;

    pub fn clear_version_number(&mut self) {
        self.version_number = 0;
    }

    // Param is passed by value, moved
    pub fn set_version_number(&mut self, v: u32) {
        self.version_number = v;
    }

    pub fn get_version_number(&self) -> u32 {
        self.version_number
    }

    fn get_version_number_for_reflect(&self) -> &u32 {
        &self.version_number
    }

    fn mut_version_number_for_reflect(&mut self) -> &mut u32 {
        &mut self.version_number
    }

    // uint64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &u64 {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut u64 {
        &mut self.timestamp
    }
}

impl ::protobuf::Message for Version {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version_number = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
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
        if self.version_number != 0 {
            my_size += ::protobuf::rt::value_size(1, self.version_number, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(2, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.version_number != 0 {
            os.write_uint32(1, self.version_number)?;
        }
        if self.timestamp != 0 {
            os.write_uint64(2, self.timestamp)?;
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

impl ::protobuf::MessageStatic for Version {
    fn new() -> Version {
        Version::new()
    }

    fn descriptor_static(_: ::std::option::Option<Version>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version_number",
                    Version::get_version_number_for_reflect,
                    Version::mut_version_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    Version::get_timestamp_for_reflect,
                    Version::mut_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Version>(
                    "Version",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Version {
    fn clear(&mut self) {
        self.clear_version_number();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Version {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Version {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Peer {
    // message fields
    pub addr: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Peer {}

impl Peer {
    pub fn new() -> Peer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Peer {
        static mut instance: ::protobuf::lazy::Lazy<Peer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Peer,
        };
        unsafe {
            instance.get(Peer::new)
        }
    }

    // string addr = 2;

    pub fn clear_addr(&mut self) {
        self.addr.clear();
    }

    // Param is passed by value, moved
    pub fn set_addr(&mut self, v: ::std::string::String) {
        self.addr = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_addr(&mut self) -> &mut ::std::string::String {
        &mut self.addr
    }

    // Take field
    pub fn take_addr(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.addr, ::std::string::String::new())
    }

    pub fn get_addr(&self) -> &str {
        &self.addr
    }

    fn get_addr_for_reflect(&self) -> &::std::string::String {
        &self.addr
    }

    fn mut_addr_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.addr
    }
}

impl ::protobuf::Message for Peer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.addr)?;
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
        if !self.addr.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.addr);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.addr.is_empty() {
            os.write_string(2, &self.addr)?;
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

impl ::protobuf::MessageStatic for Peer {
    fn new() -> Peer {
        Peer::new()
    }

    fn descriptor_static(_: ::std::option::Option<Peer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "addr",
                    Peer::get_addr_for_reflect,
                    Peer::mut_addr_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Peer>(
                    "Peer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Peer {
    fn clear(&mut self) {
        self.clear_addr();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Peer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Peer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PeerList {
    // message fields
    pub peers: ::protobuf::RepeatedField<Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PeerList {}

impl PeerList {
    pub fn new() -> PeerList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PeerList {
        static mut instance: ::protobuf::lazy::Lazy<PeerList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeerList,
        };
        unsafe {
            instance.get(PeerList::new)
        }
    }

    // repeated .matcha.Peer peers = 1;

    pub fn clear_peers(&mut self) {
        self.peers.clear();
    }

    // Param is passed by value, moved
    pub fn set_peers(&mut self, v: ::protobuf::RepeatedField<Peer>) {
        self.peers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_peers(&mut self) -> &mut ::protobuf::RepeatedField<Peer> {
        &mut self.peers
    }

    // Take field
    pub fn take_peers(&mut self) -> ::protobuf::RepeatedField<Peer> {
        ::std::mem::replace(&mut self.peers, ::protobuf::RepeatedField::new())
    }

    pub fn get_peers(&self) -> &[Peer] {
        &self.peers
    }

    fn get_peers_for_reflect(&self) -> &::protobuf::RepeatedField<Peer> {
        &self.peers
    }

    fn mut_peers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Peer> {
        &mut self.peers
    }
}

impl ::protobuf::Message for PeerList {
    fn is_initialized(&self) -> bool {
        for v in &self.peers {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.peers)?;
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
        for value in &self.peers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.peers {
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

impl ::protobuf::MessageStatic for PeerList {
    fn new() -> PeerList {
        PeerList::new()
    }

    fn descriptor_static(_: ::std::option::Option<PeerList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Peer>>(
                    "peers",
                    PeerList::get_peers_for_reflect,
                    PeerList::mut_peers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeerList>(
                    "PeerList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PeerList {
    fn clear(&mut self) {
        self.clear_peers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PeerList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PeerList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PingRequest {
    // message fields
    pub peer: ::protobuf::SingularPtrField<Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingRequest {}

impl PingRequest {
    pub fn new() -> PingRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingRequest {
        static mut instance: ::protobuf::lazy::Lazy<PingRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingRequest,
        };
        unsafe {
            instance.get(PingRequest::new)
        }
    }

    // .matcha.Peer peer = 1;

    pub fn clear_peer(&mut self) {
        self.peer.clear();
    }

    pub fn has_peer(&self) -> bool {
        self.peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer(&mut self, v: Peer) {
        self.peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer(&mut self) -> &mut Peer {
        if self.peer.is_none() {
            self.peer.set_default();
        }
        self.peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer(&mut self) -> Peer {
        self.peer.take().unwrap_or_else(|| Peer::new())
    }

    pub fn get_peer(&self) -> &Peer {
        self.peer.as_ref().unwrap_or_else(|| Peer::default_instance())
    }

    fn get_peer_for_reflect(&self) -> &::protobuf::SingularPtrField<Peer> {
        &self.peer
    }

    fn mut_peer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Peer> {
        &mut self.peer
    }
}

impl ::protobuf::Message for PingRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.peer {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer)?;
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
        if let Some(ref v) = self.peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.peer.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for PingRequest {
    fn new() -> PingRequest {
        PingRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Peer>>(
                    "peer",
                    PingRequest::get_peer_for_reflect,
                    PingRequest::mut_peer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PingRequest>(
                    "PingRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingRequest {
    fn clear(&mut self) {
        self.clear_peer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PingRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PingResponse {
    // message fields
    pub peer: ::protobuf::SingularPtrField<Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PingResponse {}

impl PingResponse {
    pub fn new() -> PingResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingResponse {
        static mut instance: ::protobuf::lazy::Lazy<PingResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingResponse,
        };
        unsafe {
            instance.get(PingResponse::new)
        }
    }

    // .matcha.Peer peer = 1;

    pub fn clear_peer(&mut self) {
        self.peer.clear();
    }

    pub fn has_peer(&self) -> bool {
        self.peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer(&mut self, v: Peer) {
        self.peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer(&mut self) -> &mut Peer {
        if self.peer.is_none() {
            self.peer.set_default();
        }
        self.peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer(&mut self) -> Peer {
        self.peer.take().unwrap_or_else(|| Peer::new())
    }

    pub fn get_peer(&self) -> &Peer {
        self.peer.as_ref().unwrap_or_else(|| Peer::default_instance())
    }

    fn get_peer_for_reflect(&self) -> &::protobuf::SingularPtrField<Peer> {
        &self.peer
    }

    fn mut_peer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Peer> {
        &mut self.peer
    }
}

impl ::protobuf::Message for PingResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.peer {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer)?;
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
        if let Some(ref v) = self.peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.peer.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for PingResponse {
    fn new() -> PingResponse {
        PingResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Peer>>(
                    "peer",
                    PingResponse::get_peer_for_reflect,
                    PingResponse::mut_peer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PingResponse>(
                    "PingResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingResponse {
    fn clear(&mut self) {
        self.clear_peer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PingResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PingResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PeerListRequest {
    // message fields
    pub peer_list: ::protobuf::SingularPtrField<PeerList>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PeerListRequest {}

impl PeerListRequest {
    pub fn new() -> PeerListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PeerListRequest {
        static mut instance: ::protobuf::lazy::Lazy<PeerListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeerListRequest,
        };
        unsafe {
            instance.get(PeerListRequest::new)
        }
    }

    // .matcha.PeerList peer_list = 1;

    pub fn clear_peer_list(&mut self) {
        self.peer_list.clear();
    }

    pub fn has_peer_list(&self) -> bool {
        self.peer_list.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer_list(&mut self, v: PeerList) {
        self.peer_list = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer_list(&mut self) -> &mut PeerList {
        if self.peer_list.is_none() {
            self.peer_list.set_default();
        }
        self.peer_list.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer_list(&mut self) -> PeerList {
        self.peer_list.take().unwrap_or_else(|| PeerList::new())
    }

    pub fn get_peer_list(&self) -> &PeerList {
        self.peer_list.as_ref().unwrap_or_else(|| PeerList::default_instance())
    }

    fn get_peer_list_for_reflect(&self) -> &::protobuf::SingularPtrField<PeerList> {
        &self.peer_list
    }

    fn mut_peer_list_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PeerList> {
        &mut self.peer_list
    }
}

impl ::protobuf::Message for PeerListRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.peer_list {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer_list)?;
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
        if let Some(ref v) = self.peer_list.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.peer_list.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for PeerListRequest {
    fn new() -> PeerListRequest {
        PeerListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PeerListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PeerList>>(
                    "peer_list",
                    PeerListRequest::get_peer_list_for_reflect,
                    PeerListRequest::mut_peer_list_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeerListRequest>(
                    "PeerListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PeerListRequest {
    fn clear(&mut self) {
        self.clear_peer_list();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PeerListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PeerListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PeerListResponse {
    // message fields
    pub peer_list: ::protobuf::SingularPtrField<PeerList>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PeerListResponse {}

impl PeerListResponse {
    pub fn new() -> PeerListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PeerListResponse {
        static mut instance: ::protobuf::lazy::Lazy<PeerListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeerListResponse,
        };
        unsafe {
            instance.get(PeerListResponse::new)
        }
    }

    // .matcha.PeerList peer_list = 1;

    pub fn clear_peer_list(&mut self) {
        self.peer_list.clear();
    }

    pub fn has_peer_list(&self) -> bool {
        self.peer_list.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer_list(&mut self, v: PeerList) {
        self.peer_list = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer_list(&mut self) -> &mut PeerList {
        if self.peer_list.is_none() {
            self.peer_list.set_default();
        }
        self.peer_list.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer_list(&mut self) -> PeerList {
        self.peer_list.take().unwrap_or_else(|| PeerList::new())
    }

    pub fn get_peer_list(&self) -> &PeerList {
        self.peer_list.as_ref().unwrap_or_else(|| PeerList::default_instance())
    }

    fn get_peer_list_for_reflect(&self) -> &::protobuf::SingularPtrField<PeerList> {
        &self.peer_list
    }

    fn mut_peer_list_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PeerList> {
        &mut self.peer_list
    }
}

impl ::protobuf::Message for PeerListResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.peer_list {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer_list)?;
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
        if let Some(ref v) = self.peer_list.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.peer_list.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for PeerListResponse {
    fn new() -> PeerListResponse {
        PeerListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PeerListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PeerList>>(
                    "peer_list",
                    PeerListResponse::get_peer_list_for_reflect,
                    PeerListResponse::mut_peer_list_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeerListResponse>(
                    "PeerListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PeerListResponse {
    fn clear(&mut self) {
        self.clear_peer_list();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PeerListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PeerListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1asrc/protos/matcha_pb.proto\x12\x06matcha\";\n\x06Wallet\x121\n\x08\
    keypairs\x18\x01\x20\x03(\x0b2\x15.matcha.WalletKeypairR\x08keypairs\"\
    \x9c\x01\n\rWalletKeypair\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\
    \x12\x1d\n\npublic_key\x18\x02\x20\x01(\x0cR\tpublicKey\x12\x1d\n\nsecre\
    t_key\x18\x03\x20\x01(\x0cR\tsecretKey\x12\x16\n\x06amount\x18\x04\x20\
    \x01(\x04R\x06amount\x12!\n\x0cnetwork_type\x18\x05\x20\x01(\rR\x0bnetwo\
    rkType\"\x85\x01\n\x10InputTransaction\x12\x13\n\x05tx_id\x18\x01\x20\
    \x01(\x0cR\x04txId\x12\x1f\n\x0btxout_index\x18\x02\x20\x01(\rR\ntxoutIn\
    dex\x12\x1c\n\tsignature\x18\x03\x20\x01(\x0cR\tsignature\x12\x1d\n\npub\
    lic_key\x18\x04\x20\x01(\x0cR\tpublicKey\"\xd6\t\n\x11OutputTransaction\
    \x12T\n\x10transaction_type\x18\x01\x20\x01(\x0e2).matcha.OutputTransact\
    ion.TransactionTypeR\x0ftransactionType\x12\x16\n\x06amount\x18\x02\x20\
    \x01(\x04R\x06amount\x12H\n\tnormal_tx\x18\x03\x20\x01(\x0b2+.matcha.Out\
    putTransaction.NormalTransactionR\x08normalTx\x12[\n\x10delegate_vote_tx\
    \x18\x04\x20\x01(\x0b21.matcha.OutputTransaction.DelegateVoteTransaction\
    R\x0edelegateVoteTx\x12s\n\x18username_registration_tx\x18\x05\x20\x01(\
    \x0b29.matcha.OutputTransaction.UsernameRegistrationTransactionR\x16user\
    nameRegistrationTx\x12L\n\x0bnew_post_tx\x18\x06\x20\x01(\x0b2,.matcha.O\
    utputTransaction.NewPostTransactionR\tnewPostTx\x12[\n\x10avatar_upload_\
    tx\x18\x07\x20\x01(\x0b21.matcha.OutputTransaction.AvatarUploadTransacti\
    onR\x0eavatarUploadTx\x12^\n\x11favourite_post_tx\x18\x08\x20\x01(\x0b22\
    .matcha.OutputTransaction.FavouritePostTransactionR\x0ffavouritePostTx\
    \x1a2\n\x11NormalTransaction\x12\x1d\n\npublic_key\x18\x01\x20\x01(\x0cR\
    \tpublicKey\x1ah\n\x17DelegateVoteTransaction\x12.\n\x13delegate_public_\
    key\x18\x01\x20\x01(\x0cR\x11delegatePublicKey\x12\x1d\n\npublic_key\x18\
    \x02\x20\x01(\x0cR\tpublicKey\x1a=\n\x1fUsernameRegistrationTransaction\
    \x12\x1a\n\x08username\x18\x01\x20\x01(\tR\x08username\x1a.\n\x12NewPost\
    Transaction\x12\x18\n\x07content\x18\x01\x20\x01(\x0cR\x07content\x1a8\n\
    \x17AvatarUploadTransaction\x12\x1d\n\nimage_data\x18\x01\x20\x01(\x0cR\
    \timageData\x1aP\n\x18FavouritePostTransaction\x12\x13\n\x05tx_id\x18\
    \x01\x20\x01(\x0cR\x04txId\x12\x1f\n\x0btxout_index\x18\x02\x20\x01(\rR\
    \ntxoutIndex\"\x92\x01\n\x0fTransactionType\x12\r\n\tNORMAL_TX\x10\0\x12\
    \x14\n\x10DELEGATE_VOTE_TX\x10\x01\x12\x1c\n\x18USERNAME_REGISTRATION_TX\
    \x10\x02\x12\x0f\n\x0bNEW_POST_TX\x10\x03\x12\x14\n\x10AVATAR_UPLOAD_TX\
    \x10\x04\x12\x15\n\x11FAVOURITE_POST_TX\x10\x05\"\x80\x01\n\x0bTransacti\
    on\x12\x0e\n\x02id\x18\x01\x20\x01(\x0cR\x02id\x12.\n\x05txins\x18\x02\
    \x20\x03(\x0b2\x18.matcha.InputTransactionR\x05txins\x121\n\x06txouts\
    \x18\x03\x20\x03(\x0b2\x19.matcha.OutputTransactionR\x06txouts\"\xe7\x01\
    \n\x05Block\x12\x18\n\x07version\x18\x01\x20\x01(\rR\x07version\x12\x1d\
    \n\npublic_key\x18\x02\x20\x01(\x0cR\tpublicKey\x12#\n\rprevious_hash\
    \x18\x03\x20\x01(\x0cR\x0cpreviousHash\x12)\n\x10transaction_root\x18\
    \x04\x20\x01(\x0cR\x0ftransactionRoot\x12\x1c\n\ttimestamp\x18\x05\x20\
    \x01(\x04R\ttimestamp\x127\n\x0ctransactions\x18\x06\x20\x03(\x0b2\x13.m\
    atcha.TransactionR\x0ctransactions\"P\n\x0bSignedBlock\x12\x1c\n\tsignat\
    ure\x18\x01\x20\x01(\x0cR\tsignature\x12#\n\x05block\x18\x02\x20\x01(\
    \x0b2\r.matcha.BlockR\x05block\"W\n\tFullBlock\x12\x12\n\x04hash\x18\x01\
    \x20\x01(\x0cR\x04hash\x126\n\x0csigned_block\x18\x02\x20\x01(\x0b2\x13.\
    matcha.SignedBlockR\x0bsignedBlock\"\x98\x02\n\x07Message\x12>\n\x0cnetw\
    ork_type\x18\x01\x20\x01(\x0e2\x1b.matcha.Message.NetworkTypeR\x0bnetwor\
    kType\x12(\n\x04type\x18\x02\x20\x01(\x0e2\x14.matcha.Message.TypeR\x04t\
    ype\x12)\n\x07request\x18\x03\x20\x01(\x0b2\x0f.matcha.RequestR\x07reque\
    st\x12,\n\x08response\x18\x04\x20\x01(\x0b2\x10.matcha.ResponseR\x08resp\
    onse\"!\n\x04Type\x12\x0b\n\x07REQUEST\x10\0\x12\x0c\n\x08RESPONSE\x10\
    \x01\"'\n\x0bNetworkType\x12\x0b\n\x07PRODNET\x10\0\x12\x0b\n\x07TESTNET\
    \x10\x01\"\xb0\x02\n\x07Request\x12(\n\x04type\x18\x01\x20\x01(\x0e2\x14\
    .matcha.Request.TypeR\x04type\x126\n\x0cping_request\x18\x02\x20\x01(\
    \x0b2\x13.matcha.PingRequestR\x0bpingRequest\x12C\n\x11peer_list_request\
    \x18\x03\x20\x01(\x0b2\x17.matcha.PeerListRequestR\x0fpeerListRequest\
    \x128\n\x0fversion_request\x18\x04\x20\x01(\x0b2\x0f.matcha.VersionR\x0e\
    versionRequest\"D\n\x04Type\x12\x13\n\x0fVERSION_REQUEST\x10\0\x12\x10\n\
    \x0cPING_REQUEST\x10\x01\x12\x15\n\x11PEER_LIST_REQUEST\x10\x02\"\x80\
    \x04\n\x08Response\x12/\n\x06status\x18\x01\x20\x01(\x0e2\x17.matcha.Res\
    ponse.StatusR\x06status\x12)\n\x04type\x18\x02\x20\x01(\x0e2\x15.matcha.\
    Response.TypeR\x04type\x12\x20\n\x0bdescription\x18\x03\x20\x01(\tR\x0bd\
    escription\x129\n\rping_response\x18\x04\x20\x01(\x0b2\x14.matcha.PingRe\
    sponseR\x0cpingResponse\x12F\n\x12peer_list_response\x18\x05\x20\x01(\
    \x0b2\x18.matcha.PeerListResponseR\x10peerListResponse\x12:\n\x10version\
    _response\x18\x06\x20\x01(\x0b2\x0f.matcha.VersionR\x0fversionResponse\"\
    X\n\x06Status\x12\x13\n\x0fINVALID_REQUEST\x10\0\x12\x07\n\x03ACK\x10\
    \x01\x12\x15\n\x11TOO_MANY_REQUESTS\x10\x02\x12\x19\n\x15INTERNAL_SERVER\
    _ERROR\x10\x03\"]\n\x04Type\x12\x14\n\x10DESCRIPTION_ONLY\x10\0\x12\x14\
    \n\x10VERSION_RESPONSE\x10\x01\x12\x11\n\rPING_RESPONSE\x10\x02\x12\x16\
    \n\x12PEER_LIST_RESPONSE\x10\x03\"N\n\x07Version\x12%\n\x0eversion_numbe\
    r\x18\x01\x20\x01(\rR\rversionNumber\x12\x1c\n\ttimestamp\x18\x02\x20\
    \x01(\x04R\ttimestamp\"\x1a\n\x04Peer\x12\x12\n\x04addr\x18\x02\x20\x01(\
    \tR\x04addr\".\n\x08PeerList\x12\"\n\x05peers\x18\x01\x20\x03(\x0b2\x0c.\
    matcha.PeerR\x05peers\"/\n\x0bPingRequest\x12\x20\n\x04peer\x18\x01\x20\
    \x01(\x0b2\x0c.matcha.PeerR\x04peer\"0\n\x0cPingResponse\x12\x20\n\x04pe\
    er\x18\x01\x20\x01(\x0b2\x0c.matcha.PeerR\x04peer\"@\n\x0fPeerListReques\
    t\x12-\n\tpeer_list\x18\x01\x20\x01(\x0b2\x10.matcha.PeerListR\x08peerLi\
    st\"A\n\x10PeerListResponse\x12-\n\tpeer_list\x18\x01\x20\x01(\x0b2\x10.\
    matcha.PeerListR\x08peerListb\x06proto3\
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

// This file is generated by rust-protobuf 2.8.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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
//! Generated file from `engine_gcmessages.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_1;

#[derive(PartialEq,Clone,Default)]
pub struct CEngineGotvSyncPacket {
    // message fields
    match_id: ::std::option::Option<u64>,
    instance_id: ::std::option::Option<u32>,
    signupfragment: ::std::option::Option<u32>,
    currentfragment: ::std::option::Option<u32>,
    tickrate: ::std::option::Option<f32>,
    tick: ::std::option::Option<u32>,
    rtdelay: ::std::option::Option<f32>,
    rcvage: ::std::option::Option<f32>,
    keyframe_interval: ::std::option::Option<f32>,
    cdndelay: ::std::option::Option<u32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CEngineGotvSyncPacket {
    fn default() -> &'a CEngineGotvSyncPacket {
        <CEngineGotvSyncPacket as ::protobuf::Message>::default_instance()
    }
}

impl CEngineGotvSyncPacket {
    pub fn new() -> CEngineGotvSyncPacket {
        ::std::default::Default::default()
    }

    // optional uint64 match_id = 1;


    pub fn get_match_id(&self) -> u64 {
        self.match_id.unwrap_or(0)
    }
    pub fn clear_match_id(&mut self) {
        self.match_id = ::std::option::Option::None;
    }

    pub fn has_match_id(&self) -> bool {
        self.match_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_id(&mut self, v: u64) {
        self.match_id = ::std::option::Option::Some(v);
    }

    // optional uint32 instance_id = 2;


    pub fn get_instance_id(&self) -> u32 {
        self.instance_id.unwrap_or(0)
    }
    pub fn clear_instance_id(&mut self) {
        self.instance_id = ::std::option::Option::None;
    }

    pub fn has_instance_id(&self) -> bool {
        self.instance_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_instance_id(&mut self, v: u32) {
        self.instance_id = ::std::option::Option::Some(v);
    }

    // optional uint32 signupfragment = 3;


    pub fn get_signupfragment(&self) -> u32 {
        self.signupfragment.unwrap_or(0)
    }
    pub fn clear_signupfragment(&mut self) {
        self.signupfragment = ::std::option::Option::None;
    }

    pub fn has_signupfragment(&self) -> bool {
        self.signupfragment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signupfragment(&mut self, v: u32) {
        self.signupfragment = ::std::option::Option::Some(v);
    }

    // optional uint32 currentfragment = 4;


    pub fn get_currentfragment(&self) -> u32 {
        self.currentfragment.unwrap_or(0)
    }
    pub fn clear_currentfragment(&mut self) {
        self.currentfragment = ::std::option::Option::None;
    }

    pub fn has_currentfragment(&self) -> bool {
        self.currentfragment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentfragment(&mut self, v: u32) {
        self.currentfragment = ::std::option::Option::Some(v);
    }

    // optional float tickrate = 5;


    pub fn get_tickrate(&self) -> f32 {
        self.tickrate.unwrap_or(0.)
    }
    pub fn clear_tickrate(&mut self) {
        self.tickrate = ::std::option::Option::None;
    }

    pub fn has_tickrate(&self) -> bool {
        self.tickrate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tickrate(&mut self, v: f32) {
        self.tickrate = ::std::option::Option::Some(v);
    }

    // optional uint32 tick = 6;


    pub fn get_tick(&self) -> u32 {
        self.tick.unwrap_or(0)
    }
    pub fn clear_tick(&mut self) {
        self.tick = ::std::option::Option::None;
    }

    pub fn has_tick(&self) -> bool {
        self.tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick(&mut self, v: u32) {
        self.tick = ::std::option::Option::Some(v);
    }

    // optional float rtdelay = 8;


    pub fn get_rtdelay(&self) -> f32 {
        self.rtdelay.unwrap_or(0.)
    }
    pub fn clear_rtdelay(&mut self) {
        self.rtdelay = ::std::option::Option::None;
    }

    pub fn has_rtdelay(&self) -> bool {
        self.rtdelay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rtdelay(&mut self, v: f32) {
        self.rtdelay = ::std::option::Option::Some(v);
    }

    // optional float rcvage = 9;


    pub fn get_rcvage(&self) -> f32 {
        self.rcvage.unwrap_or(0.)
    }
    pub fn clear_rcvage(&mut self) {
        self.rcvage = ::std::option::Option::None;
    }

    pub fn has_rcvage(&self) -> bool {
        self.rcvage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rcvage(&mut self, v: f32) {
        self.rcvage = ::std::option::Option::Some(v);
    }

    // optional float keyframe_interval = 10;


    pub fn get_keyframe_interval(&self) -> f32 {
        self.keyframe_interval.unwrap_or(0.)
    }
    pub fn clear_keyframe_interval(&mut self) {
        self.keyframe_interval = ::std::option::Option::None;
    }

    pub fn has_keyframe_interval(&self) -> bool {
        self.keyframe_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyframe_interval(&mut self, v: f32) {
        self.keyframe_interval = ::std::option::Option::Some(v);
    }

    // optional uint32 cdndelay = 11;


    pub fn get_cdndelay(&self) -> u32 {
        self.cdndelay.unwrap_or(0)
    }
    pub fn clear_cdndelay(&mut self) {
        self.cdndelay = ::std::option::Option::None;
    }

    pub fn has_cdndelay(&self) -> bool {
        self.cdndelay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cdndelay(&mut self, v: u32) {
        self.cdndelay = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for CEngineGotvSyncPacket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.match_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.instance_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.signupfragment = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.currentfragment = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.tickrate = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.tick = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.rtdelay = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.rcvage = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.keyframe_interval = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.cdndelay = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.match_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.instance_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.signupfragment {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.currentfragment {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.tickrate {
            my_size += 5;
        }
        if let Some(v) = self.tick {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.rtdelay {
            my_size += 5;
        }
        if let Some(v) = self.rcvage {
            my_size += 5;
        }
        if let Some(v) = self.keyframe_interval {
            my_size += 5;
        }
        if let Some(v) = self.cdndelay {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.match_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.instance_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.signupfragment {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.currentfragment {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.tickrate {
            os.write_float(5, v)?;
        }
        if let Some(v) = self.tick {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.rtdelay {
            os.write_float(8, v)?;
        }
        if let Some(v) = self.rcvage {
            os.write_float(9, v)?;
        }
        if let Some(v) = self.keyframe_interval {
            os.write_float(10, v)?;
        }
        if let Some(v) = self.cdndelay {
            os.write_uint32(11, v)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CEngineGotvSyncPacket {
        CEngineGotvSyncPacket::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "match_id",
                    |m: &CEngineGotvSyncPacket| { &m.match_id },
                    |m: &mut CEngineGotvSyncPacket| { &mut m.match_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "instance_id",
                    |m: &CEngineGotvSyncPacket| { &m.instance_id },
                    |m: &mut CEngineGotvSyncPacket| { &mut m.instance_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "signupfragment",
                    |m: &CEngineGotvSyncPacket| { &m.signupfragment },
                    |m: &mut CEngineGotvSyncPacket| { &mut m.signupfragment },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "currentfragment",
                    |m: &CEngineGotvSyncPacket| { &m.currentfragment },
                    |m: &mut CEngineGotvSyncPacket| { &mut m.currentfragment },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "tickrate",
                    |m: &CEngineGotvSyncPacket| { &m.tickrate },
                    |m: &mut CEngineGotvSyncPacket| { &mut m.tickrate },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tick",
                    |m: &CEngineGotvSyncPacket| { &m.tick },
                    |m: &mut CEngineGotvSyncPacket| { &mut m.tick },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "rtdelay",
                    |m: &CEngineGotvSyncPacket| { &m.rtdelay },
                    |m: &mut CEngineGotvSyncPacket| { &mut m.rtdelay },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "rcvage",
                    |m: &CEngineGotvSyncPacket| { &m.rcvage },
                    |m: &mut CEngineGotvSyncPacket| { &mut m.rcvage },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "keyframe_interval",
                    |m: &CEngineGotvSyncPacket| { &m.keyframe_interval },
                    |m: &mut CEngineGotvSyncPacket| { &mut m.keyframe_interval },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cdndelay",
                    |m: &CEngineGotvSyncPacket| { &m.cdndelay },
                    |m: &mut CEngineGotvSyncPacket| { &mut m.cdndelay },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CEngineGotvSyncPacket>(
                    "CEngineGotvSyncPacket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static CEngineGotvSyncPacket {
        static mut instance: ::protobuf::lazy::Lazy<CEngineGotvSyncPacket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CEngineGotvSyncPacket,
        };
        unsafe {
            instance.get(CEngineGotvSyncPacket::new)
        }
    }
}

impl ::protobuf::Clear for CEngineGotvSyncPacket {
    fn clear(&mut self) {
        self.match_id = ::std::option::Option::None;
        self.instance_id = ::std::option::Option::None;
        self.signupfragment = ::std::option::Option::None;
        self.currentfragment = ::std::option::Option::None;
        self.tickrate = ::std::option::Option::None;
        self.tick = ::std::option::Option::None;
        self.rtdelay = ::std::option::Option::None;
        self.rcvage = ::std::option::Option::None;
        self.keyframe_interval = ::std::option::Option::None;
        self.cdndelay = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CEngineGotvSyncPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CEngineGotvSyncPacket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17engine_gcmessages.proto\x1a\x20google/protobuf/descriptor.proto\"\
    \xd0\x02\n\x15CEngineGotvSyncPacket\x12\x19\n\x08match_id\x18\x01\x20\
    \x01(\x04R\x07matchId\x12\x1f\n\x0binstance_id\x18\x02\x20\x01(\rR\ninst\
    anceId\x12&\n\x0esignupfragment\x18\x03\x20\x01(\rR\x0esignupfragment\
    \x12(\n\x0fcurrentfragment\x18\x04\x20\x01(\rR\x0fcurrentfragment\x12\
    \x1a\n\x08tickrate\x18\x05\x20\x01(\x02R\x08tickrate\x12\x12\n\x04tick\
    \x18\x06\x20\x01(\rR\x04tick\x12\x18\n\x07rtdelay\x18\x08\x20\x01(\x02R\
    \x07rtdelay\x12\x16\n\x06rcvage\x18\t\x20\x01(\x02R\x06rcvage\x12+\n\x11\
    keyframe_interval\x18\n\x20\x01(\x02R\x10keyframeInterval\x12\x1a\n\x08c\
    dndelay\x18\x0b\x20\x01(\rR\x08cdndelayB\x03\x80\x01\0\
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

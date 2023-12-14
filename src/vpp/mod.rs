#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables, unused_mut)]
#![allow(unused)]

use std::fmt::{Debug, Error, Formatter};

#[repr(C)]
pub struct vnet_sw_interface_t {
    pub _bindgen_opaque_blob: [u32; 10usize],
}

#[repr(C)]
pub struct vlib_buffer_t {
    _unused: [u8; 0],
}

#[repr(C)]
#[repr(align(8))]
pub struct vlib_error_main_t {
    pub _bindgen_opaque_blob: [u64; 4usize],
}

#[repr(C)]
#[repr(align(8))]
pub struct vlib_trace_main_t {
    pub _bindgen_opaque_blob: [u64; 4usize],
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

unsafe impl std::marker::Send for vlib_plugin_registration_t {}

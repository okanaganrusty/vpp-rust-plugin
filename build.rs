extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

pub struct __BindgenFloat16(pub u16);

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //
    // println!("cargo:rustc-link-lib=bz2");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    // let mut bindings = bindgen::Builder::default()
    let flags = vec![
        "-I../vpp/src",
        "-I../vpp/CMakeFiles",
        // "-I../vpp/build-root/install-vpp_debug-native/vpp/include",
        "-Wno-address-of-packed-member",
        "-D__AVX512VLFP16INTRIN_H",
        "-D__AVX512FP16INTRIN_H",
    ];

    let bindings: _ = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_args(flags)
        .blocklist_type("vlib_buffer_t")
        .blocklist_type("vlib_trace_main_t")
        .blocklist_type("vlib_error_main_t")
        .blocklist_type("vnet_sw_interface_t")
        .blocklist_type("vlib_trace_main_t")
        .header("wrapper.h")
        // .opaque_type("vlib_plugin_registration_t")
        .opaque_type("vnet_hw_interface_t")
        .generate()
        // Finish the builder and generate the bindings.
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    /*

    The manipulations with types above are because bindgen does not handle this
    type of C code:

    struct blah_t;

    typedef void *(blah_callback_t)(struct blah_t *param);

    typedef struct {
      blah_callback_t *blah_callback;
    } blah_t;

    */

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file_name = out_path.join("bindings.rs");
    bindings
        .write_to_file(out_file_name.clone())
        .expect("Couldn't write bindings!");

    _ = Command::new("rustup")
        .args(&["run", "nightly", "rustfmt", out_file_name.to_str().unwrap()])
        .status(); // .unwrap();
}

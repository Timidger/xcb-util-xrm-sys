extern crate bindgen;

fn main() {
    let generated = bindgen::builder()
        .header("xcb-util-xrm/include/database.h")
        .header("xcb-util-xrm/include/entry.h")
        .header("xcb-util-xrm/include/externals.h")
        .header("xcb-util-xrm/include/match.h")
        .header("xcb-util-xrm/include/resource.h")
        .header("xcb-util-xrm/include/util.h")
        .header("xcb-util-xrm/include/xcb_xrm.h")
        .no_unstable_rust()
        .ctypes_prefix("libc")
        .clang_arg("-I")
        .clang_arg("xcb-util-xrm/include")
        .generate().unwrap();
    generated.write_to_file("src/gen.rs").unwrap();
    // TODO Static linking feature
}

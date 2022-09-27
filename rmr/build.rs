// ==================================================================================
//   Copyright (c) 2022 Caurus
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.
// ==================================================================================

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:run-if-changed=build.rs");
    println!("cargo:rustc-link-lib=rmr_si");
    println!("cargo:rustc-env=LD_LIBRARY_PATH=/usr/local/lib");

    // TODO: get it from pkgconfig (but the `rmr` does not add `pkgconfig` files during
    // installation. Also, the following path is hard-coded (default during `librmr` installation).
    let include_path = PathBuf::from("/usr/local/include/rmr/");

    let mut clang_arg = String::from("-I");
    clang_arg.push_str(
        include_path
            .to_str()
            .expect("Unable to get include path for clang"),
    );

    println!("clang_arg: {:?}", clang_arg);

    // Look at the `wrapper.h` file for details.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(clang_arg)
        .generate()
        .expect("Unable to generate bindings.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

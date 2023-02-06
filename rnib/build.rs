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
use std::path::{Path, PathBuf};

fn get_proto_files<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let paths = std::fs::read_dir(dir).unwrap();
    paths
        // Removes Errors
        .flatten()
        .filter(|x| x.path().extension().unwrap() == "proto")
        .map(|r| r.path())
        .collect()
}

fn main() -> std::io::Result<()> {
    println!("cargo:run-if-changed=external/nodeb-rnib");

    // Collect .proto files and compile into entities.rs module
    let entities_sources_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("external/nodeb-rnib/entities");
    let proto_files = get_proto_files(&entities_sources_dir);
    eprintln!("{:?}", proto_files);
    prost_build::compile_protos(&proto_files, &[&entities_sources_dir]).unwrap();
    Ok(())
}

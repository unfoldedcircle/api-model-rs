// Copyright (c) 2025 Unfolded Circle ApS, Markus Zehnder <markus.z@unfoldedcircle.com>
// SPDX-License-Identifier: MPL-2.0

use std::io::Result;

fn main() -> Result<()> {
    generate_protobuf_binding_file()?;

    Ok(())
}

// We don't want to generate the protobuf binding file for every build.
// The binding file can be created on-demand and committed to Git.
// This simplifies cross-compiling and different development environments without having to install protoc everywhere.
#[cfg(feature = "generate_protobuf")]
fn generate_protobuf_binding_file() -> Result<()> {
    let mut config = prost_build::Config::new();
    // generate Rust bytes::Bytes fields for Protobuf bytes type fields.
    config.bytes(&["."]);
    config.out_dir("src/intg/proto");
    config.compile_protos(
        &[
            "src/intg/proto/voice.proto",
            "src/intg/proto/messages.proto",
        ],
        &["src/intg/proto"],
    )
}

#[cfg(not(feature = "generate_protobuf"))]
fn generate_protobuf_binding_file() -> Result<()> {
    Ok(())
}

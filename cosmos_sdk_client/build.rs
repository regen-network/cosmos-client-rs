use std::io;

fn main() -> Result<(), io::Error> {
    tonic_build::configure()
        .compile(
            &[
                "../cosmos-sdk/codec/std/codec.proto",
            ],
            &["../cosmos-sdk"],
        )
}
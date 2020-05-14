use std::io;

fn main() -> Result<(), io::Error> {
    tonic_build::configure()
        .compile(
            &[
                "../cosmos-sdk/crypto/types/types.proto",
                "../cosmos-sdk/tx/types/types.proto",
                "../cosmos-sdk/x/auth/types/types.proto",
                "../cosmos-sdk/x/auth/vesting/types/types.proto",
                "../cosmos-sdk/x/bank/types/types.proto",
                "../cosmos-sdk/x/capability/types/types.proto",
                "../cosmos-sdk/x/crisis/types/types.proto",
                "../cosmos-sdk/x/distribution/types/types.proto",
                "../cosmos-sdk/x/evidence/types/types.proto",
                "../cosmos-sdk/x/gov/types/types.proto",
                "../cosmos-sdk/x/mint/types/types.proto",
                "../cosmos-sdk/x/slashing/types/types.proto",
                "../cosmos-sdk/x/staking/types/types.proto",
                "../cosmos-sdk/x/upgrade/types/types.proto",
            ],
            &["../cosmos-sdk"],
        )
}
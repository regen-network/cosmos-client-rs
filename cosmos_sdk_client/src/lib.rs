use tonic::Request;

pub mod tendermint {
    pub mod libs {
        pub mod kv {
            tonic::include_proto!("tendermint.libs.kv");
        }
    }

    pub mod crypto {
        pub mod merkle {
            tonic::include_proto!("tendermint.crypto.merkle");
        }
    }

    pub mod abci {
        pub mod types {
            tonic::include_proto!("tendermint.abci.types");
        }
    }
}

pub mod cosmos_sdk {
    pub mod v1 {
        tonic::include_proto!("cosmos_sdk.v1");
    }
    pub mod x {
        pub mod auth {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.x.auth.v1");
            }
            pub mod vesting {
                pub mod v1 {
                    tonic::include_proto!("cosmos_sdk.x.auth.vesting.v1");
                }
            }
        }
        pub mod bank {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.x.bank.v1");
            }
        }
        pub mod crisis {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.x.crisis.v1");
            }
        }
        pub mod distribution {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.x.distribution.v1");
            }
        }
        pub mod evidence {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.x.evidence.v1");
            }
        }
        pub mod gov {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.x.gov.v1");
            }
        }
        pub mod mint {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.x.mint.v1");
            }
        }
        pub mod params {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.x.params.v1");
            }
        }
        pub mod slashing {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.x.slashing.v1");
            }
        }
        pub mod staking {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.x.staking.v1");
            }
        }
        pub mod supply {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.x.supply.v1");
            }
        }
        pub mod upgrade {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.x.upgrade.v1");
            }
        }
    }
    pub mod codec {
        pub mod std {
            pub mod v1 {
                tonic::include_proto!("cosmos_sdk.codec.std.v1");
            }
        }
    }
}

async fn test1() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = cosmos_sdk::x::bank::v1::query_service_client::QueryServiceClient::connect("localhost:1234").await?;
    let req = Request::new(
        cosmos_sdk::x::bank::v1::QueryAllBalancesRequest{
            address:Vec::new()
        }
    );
    let res = client.query_all_balances(req).await?;
    println!("RESPONSE={:?}", res);
    Ok(())
}

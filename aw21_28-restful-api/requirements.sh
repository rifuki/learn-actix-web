#!/bin/bash
cargo install sqlx-cli --features native-tls,postgres
cargo add dotenv

cargo add actix-web
cargo add reqwest
cargo add utoipa --features "actix_extras"
cargo add utoipa-swagger-ui --features "actix-web"
cargo add sqlx --features "runtime-tokio-rustls,macros,postgres,uuid,chrono,migrate"
cargo add uuid --features "v4,fast-rng,macro-diagnostics"
cargo add config
cargo add serde --features "derive"
cargo add serde_json
cargo add thiserror
cargo add argon2

cargo add futures
cargo add regex
cargo add chrono --features "serde"
cargo add lazy_static
cargo add validator --features "derive,validator_derive"
cargo add validator_derive
cargo add futures-core
cargo add actix
cargo add rand
cargo add sluggify
cargo add blob-uuid

cargo add wiremock
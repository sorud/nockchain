#!/usr/bin/env bash
source .env
export RUST_LOG
export MINIMAL_LOG_FORMAT
export MINING_PUBKEY
# target/release/nockchain --mining-pubkey ${MINING_PUBKEY}
target/release/nockchain \
    --external-miners \
    --tcp-proxy-addr 0.0.0.0:9999 \
    --mining-pubkey ${MINING_PUBKEY}


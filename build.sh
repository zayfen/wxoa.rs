#!/usr/bin/env bash

export OPENSSL_LIB_DIR=/home/zayfen/Github/openssl/

export OPENSSL_INCLUDE_DIR=/home/zayfen/Github/openssl/include/

cargo build --release  --target=x86_64-unknown-linux-musl   

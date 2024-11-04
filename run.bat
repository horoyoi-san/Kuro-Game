@echo off
start cargo run --bin config-server
start cargo run --bin hotpatch-server
start cargo run --bin login-server
start cargo run --bin gateway-server
start cargo run --bin game-server

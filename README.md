# rustimate

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](License)
[![Build Status](https://travis-ci.org/kyleu/rustimate.svg?branch=master)](https://travis-ci.org/kyleu/rustimate)
[![Crate](https://meritbadge.herokuapp.com/rustimate)](https://crates.io/crates/rustimate)
[![Docs](https://docs.rs/rustimate/badge.svg)](https://docs.rs/rustimate)
[![Dependencies](https://deps.rs/repo/github/kyleu/rustimate/status.svg)](https://deps.rs/repo/github/kyleu/rustimate)

A planning poker app, using an actix-web server communicating via websocket to WASM shared code. It's a work in progress, mostly an exercise to learn Rust.

This project's structure is available as a cargo-generate template, [generust](https://github.com/kyleu/generust)

See [installing.md](doc/installing.md) for installation guidance. After installing, run `rustimate -h` to get started.

See [scripts.md](doc/scripts.md) for available tools for building, running, and packaging the app.

## Crates

`rustimate` splits its code into several library crates:

- `rustimate-assets`: Contains embedded static files intended to be served from the web application
- `rustimate-client`: Run in the client's browser as a WebAssembly package, includes templates
- `rustimate-controllers`: Contains actix-web HTTP controllers, usually calling methods from `rustimate-service`
- `rustimate-core`: Contains definitions that are shared between server and client
- `rustimate-service`: Contains the primary logic for the application. It receives RequestMessages and emits ResponseMessages
- `rustimate-templates`: Contains Maud templates used by the server to render responses
- `rustimate`: Stored in the root of the project, this is the app's main library and binary

## Config

The project currently exclusively uses the filesystem for saved data, no database is involved.

### Directories

By default, the application stores config files in your system's user configuration directory. See `rustimate --help` to change the directory used.

- macOS: ~/Library/Application Support/rustimate
- Linux: ~/.config/rustimate
- Windows: %APPDATA%\kyleu\/rustimate

### Files

`profile/*`: User profile information
`session/*`: Estimation sessions

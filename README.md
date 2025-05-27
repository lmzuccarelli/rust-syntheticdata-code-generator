# Overview

A simple Rust library code generator, to include for the eventual synthetic data generation

Its based in a simple config.json file (see config/metrics.json) in this repo

## Usage

Clone the repo

Build the binary (assumes you have Rust installed)

```bash
cd rust-syntheticdata-code-generator

make build

update the config file (see the metrics.json as an example)

./target/release/rust-syntheticdata-code-generator --config config/metrics.json generate

```

The library will be generated in the folder specified by the field *working-dir* in the config.json file

It will have a naming convention as follows

*<working-dir>/rust-syntheticdata-generator-<name>*

Name is set by the field *name* in the config.json 

Once the libary has been generated include it in the Cargo.toml file under features with the following naming convention

*["sd-gen-<name>"]*

Update the main.rs accoringly

build the binary with the newly generated library

```
cargo build --release --fetaures = "use-libary"
```

Finally generate the synthetic data

```
./target/release/rust-syntheticdata-code-generator --config config/metrics.json execute --name metrics --count 1000
```





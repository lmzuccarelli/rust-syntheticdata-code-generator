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
cargo build --release --features "use-library"
```

Finally generate the synthetic data

```
./target/release/rust-syntheticdata-code-generator --config config/metrics.json execute --name metrics --count 1000
```

## Notes

For the current status (output) settings we have the following values

- 0 => scale down 
- 1 => nop
- 2 => scale up

This is specific for both metrics and queuemetrics. 

The next step is to make this generic and add a range

The limits_array field should have the max length of the status range, also all output fields must have a limits_array field set.

As an example if the status has 0,1,2 values (length 3) then the limit_array should look like 

```
"limit_array": "vec![vec![value,value],vec![value,value],vec![value,value]]"

                     ^ - status=0      ^ status=1        ^ status=2 
```



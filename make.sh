#!/bin/sh

set -e
set -v

cargo build --release

member="$(ar t target/release/librfunc.a |grep ^rfunc-)"
echo $member
ar x target/release/librfunc.a $member
mv -v $member rfunc.o

gcc -Wall -Werror -o caller caller.c rfunc.o -Wl,--gc-sections
size caller

# same but with overflow checking (Cargo.toml defines release-check profile)
cargo build --profile release-check
gcc -Wall -Werror -o caller-check caller.c target/release-check/librfunc.a -Wl,--gc-sections
size caller-check

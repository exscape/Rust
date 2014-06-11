#!/bin/bash

# Based on https://github.com/thestinger/rust-playpen/blob/master/bin/irc.sh

echo 'Usage: Type commands (multiple lines are allowed), end with ^D. Exit with ^C.'

while line=$(rlwrap -S '>>> ' -H ~/.rust_repl cat); do

rustc - -o out <<EOF
#![feature(globs, macro_rules, struct_variant, simd, asm)]

extern crate collections;
extern crate native;

#[allow(dead_code)]
static version: &'static str = "$(rustc -v | tail | head -1)";

fn main() {
    let r = {
        $line
    };
    println!("{}", r)
}
EOF

./out
echo

done

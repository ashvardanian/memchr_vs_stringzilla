#!/bin/bash

FILE="leipzig1m.txt";
URL="https://introcs.cs.princeton.edu/python/42sort/leipzig1m.txt";

if [ "$#" -ne 2 ]; then
    echo "./bench1m.sh <FILE> <TOKEN>";
    exit 1;
fi

if [ -f "$FILE" ]; then
    echo "$FILE exists.";
else
    echo "$FILE does not exist. Downloading...";
    wget "$URL" -O "$FILE";
    echo "Download complete.";
fi

export FILE=$1;
export TOKENS=$2;

cargo install cargo-criterion --locked
cargo criterion --jobs 8

export -n FILE;
export -n TOKENS;

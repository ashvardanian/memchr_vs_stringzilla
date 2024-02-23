#!/bin/bash

FILE="leipzig1m.txt";
URL="https://introcs.cs.princeton.edu/python/42sort/leipzig1m.txt";

if [ "$#" -ne 1 ]; then
    echo "Please provide a token for running the benchmark";
    exit 1;
fi

if [ -f "$FILE" ]; then
    echo "$FILE exists.";
else
    echo "$FILE does not exist. Downloading...";
    wget "$URL" -O "$FILE";
    echo "Download complete.";
fi

echo "Build StringZilla";
cargo build --release --bin mm --bin sz;

echo "Benching StringZilla::sz_find";
hyperfine --runs 500 --warmup 100 "./target/release/sz --path $FILE --token $1";

echo "Benching memchr::memmem::find";
hyperfine --runs 500 --warmup 100 "./target/release/mm --path $FILE --token $1";


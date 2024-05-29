chonker is a library for compressing data using byte-pair encoding.

step 1: `cargo run --release -- train hamlet.txt > vocab`
step 2: `cargo run --release -- encode hamlet.txt -v vocab > output`
step 3: `cargo run --release -- decode output -v vocab > hamlet2.txt`

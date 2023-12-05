cargo build --release
cp ./target/release/munin* .
cp ./target/release/example-analysis* .
cp ./target/release/compiler* .
rm munin.d
rm example-analysis.d
rm compiler.d
mv example-analysis* munin-examples
mv compiler* munin-compiler


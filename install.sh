cargo build --release
cp ./target/release/munin* .
cp ./target/release/example-analysis* .
cp ./target/release/assembler* .
rm munin.d
rm example-analysis.d
rm assembler.d
mv example-analysis* munin-examples
mv assembler* munin-assembler
if cargo run --release; then
    echo "Built GB-EM successfully"
    echo "The binary can be found in target/release"
    cargo run --release --help
else
    echo "Failed in building binary"
fi
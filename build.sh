
cargo build --release
mkdir -p dylibs/ && ls target/release/ && cp -v target/release/*.* dylibs/

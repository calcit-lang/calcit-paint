
cargo build --release
mkdir -p dylibs/ && ls target/release/ && cp -v target/release/*.* dylibs/


# cargo build
# mkdir -p dylibs/ && ls target/debug/ && cp -v target/debug/*.* dylibs/

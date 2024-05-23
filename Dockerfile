# Gets the rust image for this version
FROM rust:1.78

# Copies the files over
COPY ./ ./

# builds the project
RUN cargo build --release

# Runs the project
CMD ["./target/release/simple-ddns"]
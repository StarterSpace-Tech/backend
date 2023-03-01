# Use a Rust latest image as the base
FROM rust:latest

# Set the working directory
WORKDIR /start-tec-backend

# Copy the app's source code to the container
COPY . .

# Install any required dependencies
RUN apt-get update && apt-get install -y libssl-dev && \
    cargo install --path .

# Build the app
RUN cargo build --release

# Set the entry point
CMD cargo run --release

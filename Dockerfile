# Use Ubuntu as base image to avoid firewall issues with Rust installation
FROM ubuntu:22.04

# Set environment variables
ENV RUST_VERSION=1.75.0
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    libx11-dev \
    libxrandr-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxi-dev \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain ${RUST_VERSION}

# Set working directory
WORKDIR /app

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release && rm -rf src

# Copy source code
COPY src/ ./src/
COPY test/ ./test/
COPY pytest.ini run_tests.py ./

# Build the application
RUN cargo build --release

# Expose any ports if needed (adjust as necessary)
EXPOSE 8080

# Set the default command
CMD ["./target/release/ai-screenshot-analyzer"]
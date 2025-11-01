FROM rust:1.91.0-trixie

# Install additional tools for Rust development
RUN apt-get update && apt-get install -y \
    git \
    build-essential \
    curl \
    llvm \
    clang \
    lld \
    lldb \
    wget \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install rust components
RUN rustup component add rustfmt clippy rust-src llvm-tools

# Set the working directory
WORKDIR /workspace

# Keep container running
CMD ["/bin/bash"]


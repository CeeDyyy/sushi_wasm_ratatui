# ==========================================
# Stage 1: The Builder (Compiles Rust to WASM)
# ==========================================
# We use the official Rust image to get Cargo and rustc for free
FROM rust:slim AS builder

WORKDIR /usr/src/app

# Install the dependencies required to download and build wasm-pack
RUN apt-get update && apt-get install -y curl build-essential

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Copy your actual project files into the container
COPY Cargo.toml ./
COPY src ./src

# Compile the Rust code into WebAssembly!
RUN wasm-pack build --target web --release

# ==========================================
# Stage 2: The Server (Hosts the final files)
# ==========================================
# We throw away the heavy Rust compiler and just use a tiny Nginx web server
FROM nginx:alpine

# Remove the default Nginx welcome page
RUN rm -rf /usr/share/nginx/html/*

# Copy the compiled WASM and JS from the Builder stage into Nginx
COPY --from=builder /usr/src/app/pkg /usr/share/nginx/html/pkg

# Copy your frontend HTML file into Nginx
COPY index.html /usr/share/nginx/html/

# Expose port 80 for web traffic
EXPOSE 80

# Start the web server
CMD ["nginx", "-g", "daemon off;"]
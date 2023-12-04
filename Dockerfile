# Stage 1: Build the Rust application
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Stage 2: Create a smaller image for running the executable
FROM debian:latest

# Set the working directory in the second stage
WORKDIR /app

# Copy the scripts and res folder
COPY scripts /app/scripts
COPY res /app/res

# Copy the built binary from the builder stage to the final image
COPY --from=builder /app/target/release/tbt-segmentation /app/target/release/tbt-segmentation

# Best way to run docker: docker run -it --rm --name tbt-container <image_name> bash
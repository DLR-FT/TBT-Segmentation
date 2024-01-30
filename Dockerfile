# SPDX-FileCopyrightText: 2023 German Aerospace Center (DLR)
# SPDX-License-Identifier: Apache-2.0

# To build: docker build -t tbt .

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

# Install python
RUN apt-get update
RUN apt-get install -y python3
RUN apt-get install -y python3-matplotlib
RUN apt-get install -y python3-pandas

# Set the working directory in the second stage
WORKDIR /app

# Copy the scripts and res folder
COPY scripts /app/scripts
COPY res /app/res

# Copy the built binary from the builder stage to the final image
COPY --from=builder /app/target/release/tbt-segmentation /app/target/release/tbt-segmentation

# Best way to run docker: docker run -it --rm --name tbt-container <image_name> bash
# If you want to have a look at the .png files call from your host system: docker cp <container_id>:<location_png> <location_to_be_stored>

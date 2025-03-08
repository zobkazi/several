# Use the official Rust image as the base image
FROM rust:1.66 as builder

# Set the working directory in the container
WORKDIR /usr/src/several

# Copy the project files to the working directory
COPY . .

# Install the dependencies and build the project
RUN cargo build --release

# Use a minimal image for the final stage
FROM debian:bullseye-slim

# Install required dependencies to run the Actix Web app
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled app from the builder stage
COPY --from=builder /usr/src/several/target/release/several /usr/local/bin/several

# Expose the application port
EXPOSE 8080

# Run the application
CMD ["several"]

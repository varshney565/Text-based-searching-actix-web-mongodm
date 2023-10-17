# Use the official Rust image as the base image
FROM rust:1.48

# Set the working directory in the container
WORKDIR /app

# Copy the application files into the working directory
COPY . /app

# Build the application
RUN cargo build --release

# Expose port 8080
EXPOSE 8080

# Define the entry point for the container
CMD ["./target/release/app"]
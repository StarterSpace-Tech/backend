# Start from a small, Alpine-based image
FROM alpine:3.14

# Copy the compiled executable into the container
COPY target/release/star-tec-backend /usr/local/bin/star-tec-backend

# Set the default command to run the executable
CMD ["/usr/local/bin/star-tec-backend"]

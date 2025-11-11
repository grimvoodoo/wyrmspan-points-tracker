# Use a minimal base image
FROM debian:bookworm-slim

# Create a non-root user
RUN useradd -m appuser

# Set the working directory
WORKDIR /app

# Copy the built binary from your local machine
COPY target/release/wyrmspan-points-tracker /app/wyrmspan-points-tracker

# copy html and css
COPY static /app/static/

# Create /data directory and initialize with empty scores file
RUN mkdir -p /data && touch /data/scores.txt

# Set file permissions (optional but good practice)
RUN chown -R appuser:appuser /app && chown -R appuser:appuser /data

# Switch to non-root user
USER appuser

# Expose a port if your app listens on one (change 8080 to whatever you're using)
EXPOSE 3000

# Set the entrypoint
CMD ["/app/wyrmspan-points-tracker"]

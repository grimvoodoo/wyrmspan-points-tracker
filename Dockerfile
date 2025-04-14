# Use a minimal base image
FROM debian:bookworm-slim

# Create a non-root user
RUN useradd -m appuser

# Set the working directory
WORKDIR /app

# Copy the built binary from your local machine
COPY target/release/wyrmspan-points-tracker /app/wyrmspan-points-tracker

# Copy the data file(s)
COPY /data/scores.txt /app/data/scores.txt

# copy html and css
COPY static /app/static/

# Set file permissions (optional but good practice)
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose a port if your app listens on one (change 8080 to whatever you're using)
EXPOSE 3000

# Set the entrypoint
CMD ["/app/wyrmspan-points-tracker"]

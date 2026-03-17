# Stage 1: Builder
FROM golang:1.21-alpine AS builder

WORKDIR /app

# Copy go mod files
COPY go.mod go.sum ./

# Download dependencies
RUN go mod download

# Copy source code
COPY . .

# Build the application
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o main .

# Stage 2: Runner
FROM alpine:latest

WORKDIR /app

# Install ca-certificates for HTTPS
RUN apk --no-cache add ca-certificates

# Create a non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Copy binary from builder
COPY --from=builder /app/main .

# Copy templates
COPY --from=builder /app/templates ./templates

# Set correct permissions
RUN chown -R appuser:appgroup /app

USER appuser

# Set environment variables
ENV GIN_MODE=release
ENV PORT=8080

EXPOSE 8080

CMD ["./main"]

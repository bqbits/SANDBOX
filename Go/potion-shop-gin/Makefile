.PHONY: run build clean test install help

# Default target
help:
	@echo "Benson's Potion Shop - Available Commands:"
	@echo ""
	@echo "  make run       - Run the application in development mode"
	@echo "  make build     - Build the application binary"
	@echo "  make clean     - Remove built binaries"
	@echo "  make test      - Test the API endpoints (server must be running)"
	@echo "  make install   - Download and tidy dependencies"
	@echo "  make help      - Show this help message"
	@echo ""

# Run the application
run:
	@echo "Starting Benson's Potion Shop..."
	go run main.go

# Build the binary
build:
	@echo "Building potion-shop binary..."
	go build -o potion-shop main.go
	@echo "Build complete! Run with: ./potion-shop"

# Clean built binaries
clean:
	@echo "Cleaning build artifacts..."
	rm -f potion-shop potion-shop-gin
	@echo "Clean complete!"

# Test the API (requires server to be running)
test:
	@echo "Testing API endpoints..."
	@./test-api.sh

# Install dependencies
install:
	@echo "Installing dependencies..."
	go mod download
	go mod tidy
	@echo "Dependencies installed!"

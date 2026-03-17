# Quick Start Guide

Get Benson's Potion Shop up and running in 30 seconds!

## Option 1: Using Make (Recommended)

```bash
# Navigate to the project directory
cd /Users/benson.quach/benson/SANDBOX/Go/potion-shop-gin

# Install dependencies (first time only)
make install

# Run the application
make run
```

## Option 2: Using Go Commands

```bash
# Navigate to the project directory
cd /Users/benson.quach/benson/SANDBOX/Go/potion-shop-gin

# Install dependencies (first time only)
go mod download

# Run the application
go run main.go
```

## Option 3: Build and Run Binary

```bash
# Navigate to the project directory
cd /Users/benson.quach/benson/SANDBOX/Go/potion-shop-gin

# Build the binary
go build -o potion-shop

# Run the binary
./potion-shop
```

## Access the Shop

Once the server is running, open your browser and visit:

**http://localhost:8080**

You should see the Zelda-inspired potion shop interface!

## Test the API

In a separate terminal, you can test the API endpoints:

```bash
# Make sure the server is running first!

# Run the test script
./test-api.sh

# Or test individual endpoints
curl http://localhost:8080/api/health
curl http://localhost:8080/api/stats
curl -X POST http://localhost:8080/api/buy
curl -X POST http://localhost:8080/api/reset
```

## Stop the Server

Press `Ctrl+C` in the terminal where the server is running.

## Troubleshooting

### Port Already in Use

If port 8080 is already in use, you'll see an error. You can either:
1. Stop the application using port 8080
2. Or modify the port in `main.go` (change the `port` variable)

### Dependencies Not Found

Run `go mod download` to fetch all dependencies.

### Permission Denied on test-api.sh

Make the script executable:
```bash
chmod +x test-api.sh
```

## Next Steps

- Read the full [README.md](README.md) for detailed documentation
- Explore the code in `handlers/`, `models/`, and `store/` directories
- Customize the messages in `store/store.go`
- Modify the frontend in `static/index.html`

Happy potion selling! It's dangerous to go alone!

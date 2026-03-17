# Benson's Potion Shop - Go Edition

A Zelda-inspired potion shop web application built with Go and the Gin web framework. This is the Go version of the Next.js potion shop, featuring a retro pixel art theme and fun purchase messages!

## Features

- Zelda-inspired pixel art design with animations
- In-memory shop statistics tracking
- Random fun purchase messages
- RESTful API endpoints
- Clean architecture with separated concerns (handlers, models, store)
- Thread-safe concurrent access to shop statistics

## Prerequisites

- Go 1.21 or higher
- Git (optional, for cloning)

## Installation

1. Navigate to the project directory:
```bash
cd /Users/benson.quach/benson/SANDBOX/Go/potion-shop-gin
```

2. Download dependencies:
```bash
go mod download
```

## Running the Application

### Development Mode

Run the application with detailed logging:

```bash
go run main.go
```

### Production Mode

Build and run the optimized binary:

```bash
# Build the binary
go build -o potion-shop

# Run the binary
./potion-shop
```

The server will start on `http://localhost:8080`

## API Endpoints

### GET /
Main shop page - serves the HTML interface

### GET /api/stats
Returns current shop statistics

**Response:**
```json
{
  "potions_sold": 0,
  "gold_earned": 0
}
```

### POST /api/buy
Purchase a Red Potion (costs 50 gold)

**Response:**
```json
{
  "success": true,
  "message": "You got a Red Potion! Your hearts are refilled!",
  "stats": {
    "potions_sold": 1,
    "gold_earned": 50
  }
}
```

### POST /api/reset
Reset shop statistics to zero

**Response:**
```json
{
  "success": true,
  "stats": {
    "potions_sold": 0,
    "gold_earned": 0
  }
}
```

### GET /api/health
Health check endpoint

**Response:**
```json
{
  "status": "ok"
}
```

## Project Structure

```
potion-shop-gin/
├── main.go              # Application entry point and router setup
├── go.mod               # Go module dependencies
├── handlers/
│   └── handlers.go      # HTTP request handlers
├── models/
│   └── models.go        # Data models and response structures
├── store/
│   └── store.go         # In-memory data store with thread-safety
├── static/
│   └── index.html       # Frontend HTML/CSS/JavaScript
└── README.md            # This file
```

## Configuration

- **Port**: The server runs on port 8080 by default. To change it, modify the `port` variable in `main.go`
- **Release Mode**: Uncomment `gin.SetMode(gin.ReleaseMode)` in `main.go` for production deployment

## Development

### Adding New Potion Types

1. Update the `store/store.go` to add new potion prices
2. Modify `handlers/handlers.go` to handle different potion types
3. Update the frontend in `static/index.html` to display new potions

### Thread Safety

The store uses `sync.RWMutex` to ensure thread-safe concurrent access to shop statistics. Multiple users can purchase potions simultaneously without data races.

## Fun Purchase Messages

The shop randomly selects from 8 different fun messages when you purchase a potion:
- "You got a Red Potion! Your hearts are refilled!"
- "A fine purchase, brave hero! May it serve you well!"
- "One Red Potion coming right up! Stay safe out there!"
- "Excellent choice! This potion has saved many adventurers!"
- "A wise investment! You'll thank yourself later!"
- "Thank you for your patronage, noble warrior!"
- "It's dangerous to go alone! Take this potion!"
- "May this potion guide you to victory!"

## Testing

You can test the API endpoints using curl:

```bash
# Get current stats
curl http://localhost:8080/api/stats

# Buy a potion
curl -X POST http://localhost:8080/api/buy

# Reset stats
curl -X POST http://localhost:8080/api/reset

# Health check
curl http://localhost:8080/api/health
```

## Technologies Used

- **Go 1.21+**: Programming language
- **Gin**: Web framework for building the API
- **HTML/CSS/JavaScript**: Frontend interface
- **sync.RWMutex**: Thread-safe concurrent access

## License

This is a demo project created for learning purposes.

## Credits

Inspired by The Legend of Zelda series by Nintendo.
Built with love by Benson.

---

It's dangerous to go alone! Take a potion!

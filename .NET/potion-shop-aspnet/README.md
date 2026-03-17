# Benson's Potion Store (.NET Edition)

A Zelda-inspired ASP.NET Core MVC potion shop for testing APM solutions with .NET applications.

## Features

- Retro pixel art Zelda-style theme
- Red Potions for sale (50 gold each)
- Real-time shop statistics
- Fun purchase messages
- Lightweight and perfect for APM testing
- Built with ASP.NET Core MVC and C#

## Quick Start

```bash
# Navigate to the shop
cd /Users/benson.quach/benson/SANDBOX/.NET/potion-shop-aspnet

# Run the development server
dotnet run
```

Visit: http://localhost:5000 (or https://localhost:5001 for HTTPS)

## API Endpoints

- `GET /` - Main shop page (MVC View)
- `POST /api/buy` - Purchase a potion (returns JSON: success, message, stats)
- `GET /api/stats` - Get shop statistics (JSON: potions_sold, gold_earned, gold_per_potion)
- `POST /api/reset` - Reset shop statistics
- `GET /api/health` - Health check endpoint (returns { "status": "healthy" })

## API Examples

### Purchase a Potion
```bash
curl -X POST http://localhost:5000/api/buy
```

Response:
```json
{
  "success": true,
  "message": "You got a Red Potion! Your hearts are refilled!",
  "stats": {
    "potionsSold": 1,
    "goldEarned": 50
  }
}
```

### Get Shop Statistics
```bash
curl http://localhost:5000/api/stats
```

Response:
```json
{
  "potionsSold": 5,
  "goldEarned": 250,
  "goldPerPotion": 50
}
```

### Reset Statistics
```bash
curl -X POST http://localhost:5000/api/reset
```

### Health Check
```bash
curl http://localhost:5000/api/health
```

Response:
```json
{
  "status": "healthy",
  "shop": "open",
  "potionsAvailable": true
}
```

## Testing Scenarios

1. **Basic HTTP Traces**: Visit the shop and buy potions
2. **API Calls**: Test POST request tracing with /api/buy
3. **JSON API**: Call the `/api/stats` endpoint
4. **Multiple Requests**: Simulate traffic with curl/fetch/browser
5. **Client-Side Interactions**: AJAX calls and real-time updates

## Shop Stats

- Tracks total potions sold
- Tracks total gold earned
- Displays stats on the main page and `/api/stats` endpoint
- Real-time updates without page refresh via AJAX

## Tech Stack

- **ASP.NET Core MVC** - Web framework
- **C#** - Programming language
- **Razor** - View engine for server-side rendering
- **JavaScript** - Client-side interactions (fetch API)
- **CSS** - Custom styling with Zelda-inspired theme

## Project Structure

```
potion-shop-aspnet/
├── Controllers/
│   ├── ApiController.cs       # API endpoints
│   └── HomeController.cs      # Main shop view
├── Models/
│   └── ShopStats.cs          # Data models
├── Services/
│   ├── ShopStore.cs          # In-memory state management
│   └── Messages.cs           # Random purchase messages
├── Views/
│   └── Home/
│       └── Index.cshtml      # Main shop UI
├── wwwroot/
│   ├── css/
│   │   └── site.css          # Zelda-themed styles
│   └── images/
│       ├── banner.svg        # Shop banner
│       └── red-potion.svg    # Potion image
└── Program.cs                # Application configuration
```

## Building for Production

```bash
# Build the application
dotnet build --configuration Release

# Publish for deployment
dotnet publish --configuration Release --output ./publish

# Run the published application
cd publish
dotnet potion-shop.dll
```

## Configuration

The application runs on the following ports by default:
- HTTP: `http://localhost:5000`
- HTTPS: `https://localhost:5001`

You can modify these settings in `Properties/launchSettings.json` or by using environment variables:

```bash
# Set custom port
ASPNETCORE_URLS="http://localhost:3000" dotnet run
```

## Development

### Adding New Features

1. **Models**: Add new data models in `Models/`
2. **Services**: Create services in `Services/` and register them in `Program.cs`
3. **Controllers**: Add new controllers in `Controllers/`
4. **Views**: Create new Razor views in `Views/`

### In-Memory State

The `ShopStore` service uses in-memory state management with thread-safe operations. This means:
- Statistics are stored in memory (not persisted)
- Stats reset when the application restarts
- Thread-safe for concurrent requests
- Perfect for testing and demonstrations

For production use, consider using a database like SQL Server, PostgreSQL, or Redis.

## Node.js vs .NET Comparison

This is the .NET equivalent of the Node.js Next.js application at:
`/Users/benson.quach/benson/SANDBOX/Node.js/potion-shop-nextjs`

Key differences:
- **Framework**: ASP.NET Core MVC vs Next.js
- **Language**: C# vs TypeScript
- **Rendering**: Razor views vs React components
- **API Routes**: Controller methods vs Next.js API routes
- **State Management**: Singleton service vs in-memory module

Both applications offer:
- Same Zelda-themed UI
- Same API endpoints
- Same functionality
- Same testing capabilities

---

*"It's dangerous to go alone! Take a potion!" - Benson, probably*

# Potion Shop - Spring Boot Edition

A retro Zelda-themed potion shop web application built with Spring Boot 3.x and Java 17. This is a Spring Boot version of the Next.js potion shop application, featuring the same charming retro aesthetic and functionality.

## Features

- Retro Zelda-themed UI with pixel art styling
- Buy potions with fun random purchase messages
- Real-time statistics tracking (potions sold, gold earned)
- Reset statistics functionality
- Health check endpoint
- In-memory storage (no database required)
- Responsive design
- RESTful API endpoints

## Technology Stack

- **Java 17**
- **Spring Boot 3.2.3**
- **Spring Web** - REST API endpoints
- **Thymeleaf** - Server-side HTML templating
- **Maven** - Build and dependency management

## Project Structure

```
potion-shop-springboot/
├── src/
│   ├── main/
│   │   ├── java/com/benson/potionshop/
│   │   │   ├── PotionShopApplication.java      # Main Spring Boot application
│   │   │   ├── controller/
│   │   │   │   └── ShopController.java         # REST API and view controller
│   │   │   ├── service/
│   │   │   │   ├── ShopService.java            # Shop statistics service
│   │   │   │   └── MessageService.java         # Random message generator
│   │   │   └── model/
│   │   │       └── ShopStats.java              # Shop statistics model
│   │   └── resources/
│   │       ├── application.properties          # Application configuration
│   │       ├── static/
│   │       │   ├── css/style.css               # Retro Zelda-themed styling
│   │       │   ├── js/shop.js                  # Frontend API calls
│   │       │   └── images/
│   │       │       ├── banner.svg              # Shop banner
│   │       │       └── red-potion.svg          # Potion image
│   │       └── templates/
│   │           └── index.html                  # Main HTML template
├── pom.xml                                     # Maven configuration
└── README.md                                   # This file
```

## Prerequisites

- Java 17 or higher
- Maven 3.6 or higher (or use the Maven wrapper included)

## Setup and Installation

1. **Clone or navigate to the project directory:**
   ```bash
   cd /Users/benson.quach/benson/SANDBOX/Java/potion-shop-springboot
   ```

2. **Build the project:**
   ```bash
   mvn clean install
   ```

3. **Run the application:**
   ```bash
   mvn spring-boot:run
   ```

   Alternatively, you can run the JAR file:
   ```bash
   java -jar target/potion-shop-springboot-1.0.0.jar
   ```

4. **Access the application:**
   Open your browser and navigate to:
   ```
   http://localhost:8080
   ```

## API Endpoints

### Main Application
- `GET /` - Serves the main potion shop page

### REST API
- `POST /api/buy` - Purchase a potion
  - Returns: `{ success: true, message: "...", stats: {...} }`

- `GET /api/stats` - Get shop statistics
  - Returns: `{ potions_sold: 0, gold_earned: 0, gold_per_potion: 50 }`

- `POST /api/reset` - Reset shop statistics
  - Returns: `{ success: true, message: "...", stats: {...} }`

- `GET /api/health` - Health check endpoint
  - Returns: `{ status: "healthy", shop: "open", potions_available: true }`

## How It Works

### Backend (Spring Boot)
- **PotionShopApplication.java** - Main entry point for the Spring Boot application
- **ShopController.java** - Handles all HTTP requests (both REST API and page serving)
- **ShopService.java** - Manages shop statistics in memory (singleton pattern)
- **MessageService.java** - Provides random Zelda-themed purchase messages
- **ShopStats.java** - Simple POJO for shop statistics

### Frontend
- **index.html** - Thymeleaf template with three potion cards
- **style.css** - Retro Zelda-themed styling with animations
- **shop.js** - Vanilla JavaScript for API calls and DOM manipulation

### Business Logic
- Each red potion costs 50 gold
- Statistics are stored in memory (resets when application restarts)
- 8 different random purchase messages inspired by The Legend of Zelda
- Real-time updates to statistics after each purchase

## Development

To run in development mode with auto-reload:

```bash
mvn spring-boot:run
```

The application includes Spring Boot DevTools for automatic restart when files change.

## Configuration

Edit `src/main/resources/application.properties` to customize:
- Server port (default: 8080)
- Logging levels
- Thymeleaf settings

## Production Build

To create a production-ready JAR:

```bash
mvn clean package
```

The JAR file will be in the `target/` directory.

## Differences from Next.js Version

While functionally equivalent, this Spring Boot version has some differences:
- Uses Thymeleaf for templating instead of React
- Uses vanilla JavaScript instead of React hooks
- Server-side rendering with Thymeleaf
- Traditional MVC architecture vs. Next.js App Router
- Java backend instead of Node.js/TypeScript

## License

This is a demo application for educational purposes.

## Author

Created by Benson Quach
Based on the Next.js version at `/Users/benson.quach/benson/SANDBOX/Node.js/potion-shop-nextjs/`

---

**It's dangerous to go alone! Take this potion!** ⚔️

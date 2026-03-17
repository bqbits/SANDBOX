# Benson's Potion Store - Laravel Edition

A Zelda-inspired potion shop built with Laravel. This is a single-page shop that displays 3 Red Potions for sale (50 gold each) with retro pixel art styling and fun purchase messages.

## Features

- Single-page shop interface with Zelda-inspired retro styling
- 3 Red Potions available for purchase at 50 gold each
- Session-based statistics tracking (potions sold, gold earned)
- AJAX functionality for seamless purchases without page reloads
- Random purchase messages for each transaction
- Statistics reset functionality
- Health check endpoint

## Requirements

- PHP 8.1 or higher
- Composer
- SQLite (for session storage)

## Installation

1. **Clone or navigate to the project directory:**
   ```bash
   cd /Users/benson.quach/benson/SANDBOX/PHP/potion-shop-laravel
   ```

2. **Install dependencies:**
   ```bash
   composer install
   ```

3. **Set up environment file:**
   ```bash
   cp .env.example .env
   ```

4. **Generate application key:**
   ```bash
   php artisan key:generate
   ```

5. **Create SQLite database file:**
   ```bash
   touch database/database.sqlite
   ```

6. **Set proper permissions for storage:**
   ```bash
   chmod -R 775 storage bootstrap/cache
   ```

## Running the Application

Start the Laravel development server:

```bash
php artisan serve
```

The application will be available at: `http://localhost:8000`

## API Endpoints

### GET /
Main shop page with Zelda-inspired styling

### POST /api/buy
Purchase a potion
- **Response:**
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

### GET /api/stats
Get shop statistics
- **Response:**
  ```json
  {
    "potions_sold": 0,
    "gold_earned": 0
  }
  ```

### POST /api/reset
Reset shop statistics to zero
- **Response:**
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
- **Response:**
  ```json
  {
    "status": "ok"
  }
  ```

## Purchase Messages

The following messages are randomly selected when purchasing a potion:
- "You got a Red Potion! Your hearts are refilled!"
- "A fine purchase, brave hero! May it serve you well!"
- "One Red Potion coming right up! Stay safe out there!"
- "Excellent choice! This potion has saved many adventurers!"
- "A wise investment! You'll thank yourself later!"
- "Thank you for your patronage, noble warrior!"
- "It's dangerous to go alone! Take this potion!"
- "May this potion guide you to victory!"

## Project Structure

```
potion-shop-laravel/
├── app/
│   └── Http/
│       └── Controllers/
│           └── ShopController.php      # Main controller with all shop logic
├── bootstrap/
│   └── app.php                         # Application bootstrap
├── config/                             # Configuration files
├── database/                           # Database files
├── public/
│   └── index.php                       # Entry point
├── resources/
│   └── views/
│       └── shop.blade.php              # Main shop view with styling
├── routes/
│   ├── web.php                         # Web routes
│   ├── api.php                         # API routes
│   └── console.php                     # Console routes
├── storage/                            # Storage for logs, cache, sessions
├── .env.example                        # Environment configuration template
├── artisan                             # Artisan CLI
├── composer.json                       # PHP dependencies
└── README.md                           # This file
```

## Technology Stack

- **Backend:** Laravel 11
- **Frontend:** Vanilla JavaScript with AJAX
- **Styling:** Inline CSS with Zelda-inspired retro theme
- **Session Storage:** File-based sessions (can be configured to use database)

## Design Features

- Retro pixel art style
- Green gradient background (#1a472a to #0d2818)
- Golden borders and accents
- Animated elements (floating banner, sparkling cards, rotating gold coins)
- Responsive grid layout for potion cards
- Heart beat animation for health indicators
- Pulse animation for purchase messages

## Notes

- Statistics are stored in PHP sessions and persist across page reloads
- Each session is isolated, so different browser sessions will have separate statistics
- The application uses CSRF protection for all POST requests
- No database setup required for basic functionality (uses file-based sessions)

## Credits

Inspired by the classic Zelda potion shops and based on the Next.js version at:
`/Users/benson.quach/benson/SANDBOX/Node.js/potion-shop`

# Quick Start Guide

## Prerequisites

Make sure you have installed:
- PHP 8.1 or higher
- Composer

## Installation (Automated)

Run the setup script:

```bash
chmod +x setup.sh
./setup.sh
```

## Installation (Manual)

If you prefer to set up manually:

```bash
# 1. Install dependencies
composer install

# 2. Copy environment file
cp .env.example .env

# 3. Generate application key
php artisan key:generate

# 4. Create database file
touch database/database.sqlite

# 5. Set permissions
chmod -R 775 storage bootstrap/cache
```

## Running the Application

```bash
php artisan serve
```

Visit: http://localhost:8000

## Testing the API

### Health Check
```bash
curl http://localhost:8000/api/health
```

### Get Stats
```bash
curl http://localhost:8000/api/stats
```

### Buy a Potion
```bash
curl -X POST http://localhost:8000/api/buy \
  -H "Content-Type: application/json"
```

### Reset Stats
```bash
curl -X POST http://localhost:8000/api/reset \
  -H "Content-Type: application/json"
```

## Running Tests

```bash
php artisan test
```

## Troubleshooting

### Permission Issues
```bash
chmod -R 775 storage bootstrap/cache
```

### Session Not Working
Make sure the `storage/framework/sessions` directory exists:
```bash
mkdir -p storage/framework/sessions
chmod -R 775 storage
```

### Application Key Error
```bash
php artisan key:generate
```

## Features to Try

1. Visit the shop and buy some potions
2. Watch the stats update in real-time
3. Try resetting the stats
4. Notice the different purchase messages
5. Enjoy the Zelda-inspired animations!

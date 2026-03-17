# Benson's Potion Shop (Rails)

A Zelda-inspired potion shop built with Ruby on Rails 8. This is a charming e-commerce demo application featuring a retro pixel-art aesthetic, perfect for testing APM tracing and learning Rails basics.

![Rails Version](https://img.shields.io/badge/rails-8.1.2-red)
![Ruby Version](https://img.shields.io/badge/ruby-3.2.2-red)

## Overview

This application is a simple but fully functional potion shop where customers can:
- Browse and purchase red potions for 50 gold each
- Track shop statistics (potions sold, gold earned)
- Reset statistics
- Enjoy a nostalgic Zelda-themed gaming experience

## Features

- 🎮 **Retro Zelda-inspired UI** with pixel art styling
- 💰 **Real-time statistics tracking** (potions sold, gold earned)
- 🧪 **Interactive potion purchasing** with fun flavor-text messages
- ✨ **Smooth animations** (floating banner, sparkles, rotating coins)
- 📱 **Responsive design** (mobile, tablet, desktop)
- 🚀 **RESTful JSON API** for all operations
- 🎯 **Rails 8 best practices** (Hotwire, Turbo, Stimulus-ready)

## Tech Stack

- **Framework**: Ruby on Rails 8.1.2
- **Language**: Ruby 3.2.2
- **Database**: SQLite3
- **Frontend**: ERB templates + vanilla JavaScript
- **Styling**: Custom CSS with Google Fonts (Press Start 2P)
- **Assets**: SVG graphics (banner, potion bottle)

## Quick Start

### Prerequisites

- Ruby 3.2.2 (use rbenv or rvm)
- Bundler
- SQLite3

### Installation

1. **Navigate to the project directory**
   ```bash
   cd /Users/benson.quach/benson/SANDBOX/Ruby/potion-shop-rails
   ```

2. **Install dependencies**
   ```bash
   bundle install
   ```

3. **Set up the database**
   ```bash
   rails db:create
   rails db:migrate
   ```

4. **Start the server**
   ```bash
   rails server
   ```

5. **Open your browser**
   ```
   http://localhost:3000
   ```

## API Documentation

All API endpoints return JSON responses.

### Endpoints

#### GET /api/stats
Get current shop statistics.

**Response:**
```json
{
  "potions_sold": 5,
  "gold_earned": 250
}
```

**Example:**
```bash
curl http://localhost:3000/api/stats
```

---

#### POST /api/buy
Purchase a red potion for 50 gold.

**Response:**
```json
{
  "message": "It's dangerous to go alone! Take this.",
  "potions_sold": 6,
  "gold_earned": 300
}
```

**Example:**
```bash
curl -X POST http://localhost:3000/api/buy \
  -H "Content-Type: application/json"
```

---

#### POST /api/reset
Reset all statistics to zero.

**Response:**
```json
{
  "message": "Statistics have been reset!",
  "stats": {
    "potions_sold": 0,
    "gold_earned": 0
  }
}
```

**Example:**
```bash
curl -X POST http://localhost:3000/api/reset \
  -H "Content-Type: application/json"
```

---

#### GET /api/health
Health check endpoint for monitoring.

**Response:**
```json
{
  "status": "ok",
  "timestamp": "2026-03-17T19:23:45Z",
  "service": "potion-shop-rails"
}
```

**Example:**
```bash
curl http://localhost:3000/api/health
```

## Project Structure

```
potion-shop-rails/
├── app/
│   ├── controllers/
│   │   ├── shop_controller.rb           # Main shop page controller
│   │   └── api/
│   │       └── shop_controller.rb       # API endpoints
│   ├── models/
│   │   └── shop_stat.rb                 # Shop statistics model
│   ├── views/
│   │   └── shop/
│   │       └── index.html.erb           # Main shop UI
│   └── assets/
│       ├── stylesheets/
│       │   └── application.css          # Zelda-themed CSS
│       └── images/
│           ├── banner.svg               # Shop banner
│           └── red-potion.svg           # Potion icon
├── config/
│   ├── routes.rb                        # Application routes
│   └── database.yml                     # Database configuration
├── db/
│   ├── migrate/
│   │   └── *_create_shop_stats.rb      # Database migration
│   └── schema.rb                        # Database schema
└── README.md                            # This file
```

## Database Schema

### shop_stats

| Column | Type | Default | Description |
|--------|------|---------|-------------|
| id | integer | - | Primary key |
| potions_sold | integer | 0 | Total potions sold |
| gold_earned | integer | 0 | Total gold earned |
| created_at | datetime | - | Record creation time |
| updated_at | datetime | - | Record update time |

## Purchase Messages

The shop randomly selects from 8 fun messages when you purchase a potion:

1. "It's dangerous to go alone! Take this."
2. "You got a Red Potion! Your health is restored!"
3. "This will help you on your quest, traveler."
4. "A fine choice! That'll be 50 gold."
5. "Thank you for your business, hero!"
6. "May this potion serve you well in battle!"
7. "One Red Potion, coming right up!"
8. "You look like you could use this, adventurer."

## Development

### Rails Console

```bash
rails console
```

Interact with the shop statistics:

```ruby
# Get current stats
ShopStat.current_stats

# Add a purchase
ShopStat.add_purchase

# Reset statistics
ShopStat.reset_stats
```

### Running Tests

```bash
# Add tests as needed
rails test
```

### Database Operations

```bash
# Reset database
rails db:reset

# Rollback migration
rails db:rollback

# View routes
rails routes
```

## Deployment

### Using Kamal (Rails 8 default)

```bash
# Configure deployment
kamal setup

# Deploy
kamal deploy
```

### Using Heroku

```bash
# Create Heroku app
heroku create bensons-potion-shop

# Deploy
git push heroku main

# Run migrations
heroku run rails db:migrate

# Open app
heroku open
```

## Customization

### Changing Potion Price

Edit `app/models/shop_stat.rb`:

```ruby
POTION_PRICE = 100  # Change from 50 to 100
```

### Adding New Messages

Edit `app/models/shop_stat.rb`:

```ruby
PURCHASE_MESSAGES = [
  "Your custom message here!",
  # ... add more messages
].freeze
```

### Changing Theme Colors

Edit `app/assets/stylesheets/application.css`:

```css
body {
  background: linear-gradient(135deg, #your-color1 0%, #your-color2 100%);
}
```

## APM Integration (Optional)

### Adding Datadog APM

1. Add to Gemfile:
   ```ruby
   gem 'datadog', '~> 2.0'
   ```

2. Create `config/initializers/datadog.rb`:
   ```ruby
   Datadog.configure do |c|
     c.service = 'potion-shop-rails'
     c.env = Rails.env
     c.version = '1.0.0'
   end
   ```

3. Set environment variables:
   ```bash
   export DD_TRACE_ENABLED=true
   export DD_SERVICE=potion-shop-rails
   ```

## Troubleshooting

### Port already in use
```bash
# Find process using port 3000
lsof -i :3000

# Kill the process
kill -9 <PID>
```

### Database locked
```bash
# Reset database
rails db:reset
```

### Assets not loading
```bash
# Clear cache
rails tmp:clear
```

## Contributing

This is a demo application for learning and testing purposes. Feel free to:

- Fork the repository
- Add new potion types
- Enhance the UI
- Add user authentication
- Create an admin dashboard

## License

MIT License - feel free to use this project for learning and testing!

## Credits

- Inspired by The Legend of Zelda series
- Built with Ruby on Rails 8
- Press Start 2P font from Google Fonts
- Created for APM testing and Rails education

## Related Projects

- **potion-shop-rails-k8s**: Kubernetes-ready version with Docker support
- **potion-shop-nextjs**: Next.js version of this application

---

**Happy adventuring! May your potions serve you well!** ⚔️🧪✨

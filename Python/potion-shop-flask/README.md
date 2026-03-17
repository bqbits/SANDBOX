# Benson's Potion Store 🧪⚔️

A Zelda-inspired Flask potion shop for testing Datadog APM Python tracer (dd-trace-py).

## Features

- 🎮 Retro pixel art Zelda-style theme
- 🧪 Red Potions for sale (50 gold each)
- 📊 Real-time shop statistics
- ✨ Fun purchase messages
- 🎯 Lightweight and perfect for APM testing

## Quick Start

```bash
# Navigate to the shop
cd /Users/benson.quach/benson/SANDBOX/Python/potion-shop-flask

# Create virtual environment
python3 -m venv venv
source venv/bin/activate

# Install dependencies
pip install -r requirements.txt

# Run the shop
python app.py
```

Visit: http://localhost:5000

## Using with dd-trace-py

```bash
# Install dd-trace-py
pip install ddtrace

# Run with Datadog tracing
DD_TRACE_ENABLED=true DD_SERVICE=bensons-potion-shop ddtrace-run python app.py

# Or with custom configuration
DD_TRACE_ENABLED=true \
DD_SERVICE=bensons-potion-shop \
DD_ENV=local \
DD_VERSION=1.0.0 \
ddtrace-run python app.py
```

## API Endpoints

- `GET /` - Main shop page
- `POST /buy` - Purchase a potion
- `GET /stats` - Get shop statistics (JSON)
- `POST /reset` - Reset shop statistics
- `GET /health` - Health check endpoint

## Testing Scenarios

1. **Basic HTTP Traces**: Visit the shop and buy potions
2. **Form Submissions**: Test POST request tracing
3. **JSON API**: Call the `/stats` endpoint
4. **Multiple Requests**: Simulate traffic with curl/browser

## Shop Stats

- Tracks total potions sold
- Tracks total gold earned
- Displays stats on the main page and `/stats` endpoint

---

*"It's dangerous to go alone! Take a potion!" - Benson, probably*

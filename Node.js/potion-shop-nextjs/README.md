# Benson's Potion Store

A Zelda-inspired Next.js potion shop for testing Datadog APM Node.js tracer (dd-trace).

## Features

- Retro pixel art Zelda-style theme
- Red Potions for sale (50 gold each)
- Real-time shop statistics
- Fun purchase messages
- Lightweight and perfect for APM testing
- Built with Next.js 14 and TypeScript

## Quick Start

```bash
# Navigate to the shop
cd /Users/benson.quach/benson/SANDBOX/Node.js/potion-shop-nextjs

# Install dependencies
npm install

# Run the development server
npm run dev
```

Visit: http://localhost:3000

## Using with dd-trace

```bash
# Install dd-trace
npm install dd-trace

# Create a custom server file (server.js) to initialize dd-trace
# Then run with Datadog tracing
node server.js

# Or use environment variables
DD_TRACE_ENABLED=true \
DD_SERVICE=bensons-potion-shop \
DD_ENV=local \
DD_VERSION=1.0.0 \
node --require dd-trace/init server.js
```

### Example server.js for dd-trace

Create a `server.js` file:

```javascript
// Initialize dd-trace before importing Next.js
require('dd-trace').init({
  service: 'bensons-potion-shop',
  env: process.env.DD_ENV || 'local',
  version: process.env.DD_VERSION || '1.0.0',
});

const { createServer } = require('http');
const { parse } = require('url');
const next = require('next');

const dev = process.env.NODE_ENV !== 'production';
const app = next({ dev });
const handle = app.getRequestHandler();

app.prepare().then(() => {
  createServer((req, res) => {
    const parsedUrl = parse(req.url, true);
    handle(req, res, parsedUrl);
  }).listen(3000, (err) => {
    if (err) throw err;
    console.log('> Ready on http://localhost:3000');
  });
});
```

## API Endpoints

- `GET /` - Main shop page
- `POST /api/buy` - Purchase a potion (returns JSON with message and stats)
- `GET /api/stats` - Get shop statistics (JSON)
- `POST /api/reset` - Reset shop statistics
- `GET /api/health` - Health check endpoint

## Testing Scenarios

1. **Basic HTTP Traces**: Visit the shop and buy potions
2. **API Calls**: Test POST request tracing with /api/buy
3. **JSON API**: Call the `/api/stats` endpoint
4. **Multiple Requests**: Simulate traffic with fetch/browser
5. **Client-Side Interactions**: Next.js client-side routing and state updates

## Shop Stats

- Tracks total potions sold
- Tracks total gold earned
- Displays stats on the main page and `/api/stats` endpoint
- Real-time updates without page refresh

## Tech Stack

- **Next.js 14** - React framework with App Router
- **TypeScript** - Type-safe development
- **React** - UI components
- **CSS** - Custom styling with Zelda-inspired theme

## Building for Production

```bash
# Build the application
npm run build

# Run the production server
npm start
```

---

*"It's dangerous to go alone! Take a potion!" - Benson, probably*

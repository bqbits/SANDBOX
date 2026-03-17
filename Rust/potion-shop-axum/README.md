# Benson's Potion Shop (Rust/Axum)

A Zelda-themed potion shop web application built with Rust and Axum, mimicking the Node.js Next.js version.

## Running Locally

```bash
cargo run
```

Open http://localhost:3000

## API Endpoints

| Method | Route | Description |
|--------|-------|-------------|
| GET | / | Main shop UI |
| POST | /api/buy | Buy a potion (50 gold) |
| GET | /api/stats | Get shop statistics |
| POST | /api/reset | Reset statistics |
| GET | /api/health | Health check |

## Environment Variables

- `PORT` — Server port (default: 3000)

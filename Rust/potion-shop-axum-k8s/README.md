# Benson's Potion Shop (Rust/Axum + Kubernetes)

A Zelda-themed potion shop built with Rust and Axum, deployable to Kubernetes.

## Running Locally

```bash
cargo run
```

Open http://localhost:3000

## Docker Build

```bash
docker build -t potion-shop-axum-k8s:latest .
docker run -p 3000:3000 potion-shop-axum-k8s:latest
```

## Kubernetes Deploy

```bash
kubectl apply -f k8s/
kubectl get pods
```

Access via NodePort 30081.

## API Endpoints

| Method | Route | Description |
|--------|-------|-------------|
| GET | / | Main shop UI |
| POST | /api/buy | Buy a potion (50 gold) |
| GET | /api/stats | Get shop statistics |
| POST | /api/reset | Reset statistics |
| GET | /api/health | Health check (used by K8s probes) |

## Environment Variables

- `PORT` — Server port (default: 3000)

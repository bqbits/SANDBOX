# Benson's Potion Store (Kubernetes Edition)

A Zelda-inspired Next.js potion shop with Docker and Kubernetes support for testing Datadog APM Node.js tracer (dd-trace).

## Features

- Retro pixel art Zelda-style theme
- Red Potions for sale (50 gold each)
- Real-time shop statistics
- Fun purchase messages
- Lightweight and perfect for APM testing
- Built with Next.js 14 and TypeScript
- **Docker containerized**
- **Kubernetes ready with manifests**
- Health check endpoints for K8s probes

## Quick Start (Local Development)

```bash
# Navigate to the shop
cd /Users/benson.quach/benson/SANDBOX/Node.js/potion-shop-nextjs-k8s

# Install dependencies
npm install

# Run the development server
npm run dev
```

Visit: http://localhost:3000

## Docker Setup

### Build Docker Image

```bash
# Build the Docker image
docker build -t potion-shop-k8s:latest .

# Run the container
docker run -p 3000:3000 potion-shop-k8s:latest
```

Visit: http://localhost:3000

### Docker Compose (Optional)

Create a `docker-compose.yml`:

```yaml
version: '3.8'
services:
  potion-shop:
    build: .
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production
    restart: unless-stopped
```

Run with: `docker-compose up`

## Kubernetes Deployment

### Prerequisites

- Kubernetes cluster (minikube, Docker Desktop K8s, or cloud provider)
- kubectl configured

### Deploy to Kubernetes

```bash
# Build and load image (for local K8s like minikube)
docker build -t potion-shop-k8s:latest .

# For minikube, load the image
minikube image load potion-shop-k8s:latest

# Apply Kubernetes manifests
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml

# Check deployment status
kubectl get pods
kubectl get services

# Access the application
# For minikube:
minikube service potion-shop-k8s

# For Docker Desktop K8s:
# Visit http://localhost:30080
```

### Kubernetes Resources

The deployment includes:
- **Deployment**: 2 replicas for high availability
- **Service**: NodePort service exposing port 30080
- **Health Checks**: Liveness and readiness probes on `/api/health`
- **Resource Limits**: CPU and memory constraints

### View Logs

```bash
# View logs from all pods
kubectl logs -l app=potion-shop-k8s

# Follow logs
kubectl logs -f deployment/potion-shop-k8s
```

### Scale the Deployment

```bash
# Scale to 3 replicas
kubectl scale deployment potion-shop-k8s --replicas=3

# Check scaling
kubectl get pods -w
```

## Using with dd-trace

### Docker with dd-trace

Update the Dockerfile to include dd-trace:

```dockerfile
# Add to dependencies stage
RUN npm install dd-trace
```

Then run with environment variables:

```bash
docker run -p 3000:3000 \
  -e DD_TRACE_ENABLED=true \
  -e DD_SERVICE=potion-shop-k8s \
  -e DD_ENV=docker \
  -e DD_VERSION=1.0.0 \
  -e DD_AGENT_HOST=host.docker.internal \
  potion-shop-k8s:latest
```

### Kubernetes with dd-trace

Update the deployment to include Datadog environment variables:

```yaml
env:
- name: DD_TRACE_ENABLED
  value: "true"
- name: DD_SERVICE
  value: "potion-shop-k8s"
- name: DD_ENV
  value: "kubernetes"
- name: DD_VERSION
  value: "1.0.0"
- name: DD_AGENT_HOST
  valueFrom:
    fieldRef:
      fieldPath: status.hostIP
```

## API Endpoints

- `GET /` - Main shop page
- `POST /api/buy` - Purchase a potion (returns JSON with message and stats)
- `GET /api/stats` - Get shop statistics (JSON)
- `POST /api/reset` - Reset shop statistics
- `GET /api/health` - Health check endpoint (used by K8s probes)

## Testing Scenarios

1. **Basic HTTP Traces**: Visit the shop and buy potions
2. **API Calls**: Test POST request tracing with /api/buy
3. **JSON API**: Call the `/api/stats` endpoint
4. **Multiple Requests**: Simulate traffic with fetch/browser
5. **Client-Side Interactions**: Next.js client-side routing and state updates
6. **Container Health**: K8s health probes monitor `/api/health`
7. **Load Balancing**: Multiple replicas distribute traffic

## Architecture

```
┌─────────────────┐
│   Kubernetes    │
│     Service     │
│  (NodePort)     │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
┌───▼───┐ ┌──▼────┐
│ Pod 1 │ │ Pod 2 │
│       │ │       │
└───────┘ └───────┘
```

## Tech Stack

- **Next.js 14** - React framework with App Router
- **TypeScript** - Type-safe development
- **React** - UI components
- **Docker** - Containerization
- **Kubernetes** - Orchestration and scaling
- **Node.js 20** - Runtime (Alpine-based)

## Cleanup

```bash
# Remove Kubernetes resources
kubectl delete -f k8s/

# Remove Docker containers
docker ps | grep potion-shop-k8s | awk '{print $1}' | xargs docker stop
docker images | grep potion-shop-k8s | awk '{print $3}' | xargs docker rmi
```

---

*"It's dangerous to go alone! Take a potion!" - Benson, probably*

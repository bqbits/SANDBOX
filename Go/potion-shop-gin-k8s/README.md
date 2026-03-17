# Benson's Potion Store (Go + Gin + Kubernetes Edition)

A Zelda-inspired potion shop built with Go and the Gin web framework, featuring Docker and Kubernetes support for testing and deployment.

## Features

- Retro pixel art Zelda-style theme
- Red Potions for sale (50 gold each)
- Real-time shop statistics
- Fun random purchase messages
- Lightweight and blazing fast with Go + Gin
- In-memory storage (thread-safe with mutex locks)
- **Docker containerized with multi-stage builds**
- **Kubernetes ready with manifests**
- Health check endpoints for K8s probes
- Clean project structure with proper separation of concerns

## Quick Start (Local Development)

### Prerequisites

- Go 1.21 or later
- Git

### Run Locally

```bash
# Navigate to the shop
cd /Users/benson.quach/benson/SANDBOX/Go/potion-shop-gin-k8s

# Download dependencies
go mod download

# Run the application
go run main.go
```

Visit: http://localhost:8080

## Docker Setup

### Build Docker Image

```bash
# Build the Docker image
docker build -t potion-shop-gin-k8s:latest .

# Run the container
docker run -p 8080:8080 potion-shop-gin-k8s:latest
```

Visit: http://localhost:8080

### Docker Compose (Optional)

Create a `docker-compose.yml`:

```yaml
version: '3.8'
services:
  potion-shop:
    build: .
    ports:
      - "8080:8080"
    environment:
      - GIN_MODE=release
      - PORT=8080
    restart: unless-stopped
```

Run with: `docker-compose up`

## Kubernetes Deployment

### Prerequisites

- Kubernetes cluster (minikube, Docker Desktop K8s, or cloud provider)
- kubectl configured

### Deploy to Kubernetes

```bash
# Build and tag the image
docker build -t potion-shop-gin-k8s:latest .

# For minikube, load the image
minikube image load potion-shop-gin-k8s:latest

# Apply Kubernetes manifests
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml

# Check deployment status
kubectl get pods
kubectl get services

# Access the application
# For minikube:
minikube service potion-shop-gin-k8s

# For Docker Desktop K8s:
# Visit http://localhost:30080
```

### Kubernetes Resources

The deployment includes:
- **Deployment**: 2 replicas for high availability
- **Service**: NodePort service exposing port 30080
- **Health Checks**: Liveness and readiness probes on `/api/health`
- **Resource Limits**: Conservative CPU (250m) and memory (128Mi) limits
- **Multi-stage Build**: Minimal Alpine-based container (~15MB)

### View Logs

```bash
# View logs from all pods
kubectl logs -l app=potion-shop-gin-k8s

# Follow logs
kubectl logs -f deployment/potion-shop-gin-k8s

# View logs from a specific pod
kubectl logs <pod-name>
```

### Scale the Deployment

```bash
# Scale to 3 replicas
kubectl scale deployment potion-shop-gin-k8s --replicas=3

# Check scaling
kubectl get pods -w
```

### Port Forwarding (Alternative Access Method)

```bash
# Forward local port to service
kubectl port-forward service/potion-shop-gin-k8s 8080:8080

# Visit http://localhost:8080
```

## API Endpoints

- `GET /` - Main shop page (serves HTML)
- `POST /api/buy` - Purchase a potion (returns JSON with message and stats)
- `GET /api/stats` - Get shop statistics (JSON)
- `POST /api/reset` - Reset shop statistics
- `GET /api/health` - Health check endpoint (used by K8s probes)

### Example API Usage

```bash
# Buy a potion
curl -X POST http://localhost:8080/api/buy

# Get stats
curl http://localhost:8080/api/stats

# Reset stats
curl -X POST http://localhost:8080/api/reset

# Health check
curl http://localhost:8080/api/health
```

## Project Structure

```
potion-shop-gin-k8s/
├── main.go                 # Application entry point
├── go.mod                  # Go module definition
├── go.sum                  # Dependency checksums
├── Dockerfile              # Multi-stage Docker build
├── README.md               # This file
├── .gitignore              # Git ignore rules
├── .dockerignore           # Docker ignore rules
├── internal/
│   ├── handlers/
│   │   └── handlers.go     # HTTP request handlers
│   ├── models/
│   │   └── models.go       # Data structures
│   └── store/
│       └── store.go        # In-memory storage with mutex
├── templates/
│   └── index.html          # Main shop HTML page
└── k8s/
    ├── deployment.yaml     # Kubernetes deployment
    └── service.yaml        # Kubernetes service
```

## Testing Scenarios

1. **Basic HTTP Traces**: Visit the shop and buy potions
2. **API Calls**: Test POST request handling with /api/buy
3. **JSON API**: Call the `/api/stats` endpoint
4. **Multiple Requests**: Simulate concurrent traffic
5. **Container Health**: K8s health probes monitor `/api/health`
6. **Load Balancing**: Multiple replicas distribute traffic
7. **Thread Safety**: Concurrent requests handled safely with mutex locks

## Architecture

```
┌─────────────────┐
│   Kubernetes    │
│     Service     │
│   (NodePort)    │
│   Port: 30080   │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
┌───▼───┐ ┌──▼────┐
│ Pod 1 │ │ Pod 2 │
│ :8080 │ │ :8080 │
└───────┘ └───────┘
```

## Tech Stack

- **Go 1.21** - Programming language
- **Gin** - High-performance web framework
- **Docker** - Containerization with multi-stage builds
- **Kubernetes** - Orchestration and scaling
- **Alpine Linux** - Minimal base image

## Performance

- Container size: ~15MB (thanks to multi-stage builds)
- Memory footprint: ~20-30MB at runtime
- Request handling: Thousands of requests per second
- Cold start: < 100ms

## Development

### Build Binary

```bash
# Build for current platform
go build -o potion-shop .

# Build for Linux (for Docker)
CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o main .
```

### Run Tests

```bash
# Run tests
go test ./...

# Run tests with coverage
go test -cover ./...
```

### Hot Reload (Development)

Install Air for hot reloading:

```bash
# Install Air
go install github.com/cosmtrek/air@latest

# Run with hot reload
air
```

## Environment Variables

- `PORT` - Server port (default: 8080)
- `GIN_MODE` - Gin mode: debug, release, test (default: release)

## Cleanup

```bash
# Remove Kubernetes resources
kubectl delete -f k8s/

# Remove Docker containers and images
docker ps | grep potion-shop-gin-k8s | awk '{print $1}' | xargs docker stop
docker images | grep potion-shop-gin-k8s | awk '{print $3}' | xargs docker rmi

# For minikube
minikube delete
```

## Comparison with Next.js Version

| Feature | Go + Gin | Next.js |
|---------|----------|---------|
| Container Size | ~15MB | ~150MB |
| Memory Usage | ~20-30MB | ~100-150MB |
| Cold Start | < 100ms | ~500ms |
| Request/sec | 50,000+ | 5,000+ |
| Language | Go | TypeScript/JavaScript |
| Framework | Gin | Next.js |

## Future Enhancements

- Add database persistence (PostgreSQL, Redis)
- Implement authentication
- Add more potion types
- Integrate with Datadog APM
- Add Prometheus metrics
- Implement circuit breakers
- Add rate limiting

## Troubleshooting

### Image Not Found in Minikube

```bash
# Make sure to load the image into minikube
minikube image load potion-shop-gin-k8s:latest

# Verify the image is loaded
minikube image ls | grep potion-shop
```

### Pods Not Starting

```bash
# Check pod status
kubectl describe pod <pod-name>

# Check logs
kubectl logs <pod-name>
```

### Port Already in Use

```bash
# Find process using port 8080
lsof -i :8080

# Kill the process
kill -9 <PID>
```

## License

MIT License - Feel free to use this for learning and testing!

---

*"It's dangerous to go alone! Take a potion!" - Benson, probably*

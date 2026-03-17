# Benson's Potion Store - ASP.NET Core Edition (Kubernetes Ready)

A Zelda-inspired ASP.NET Core MVC potion shop with Docker and Kubernetes support. This is the .NET equivalent of the Node.js Next.js application, perfect for testing APM, distributed tracing, and container orchestration.

## Features

- Retro pixel art Zelda-style theme
- Red Potions for sale (50 gold each)
- Real-time shop statistics with AJAX updates
- Fun purchase messages
- Lightweight and perfect for APM testing
- Built with ASP.NET Core MVC and C#
- **Docker containerized with multi-stage build**
- **Kubernetes ready with manifests**
- Health check endpoints for K8s probes
- In-memory state management (Singleton pattern)

## Tech Stack

- **ASP.NET Core 8.0 MVC** - Web framework
- **C#** - Programming language
- **Razor** - View engine
- **JavaScript** - Client-side interactions (Fetch API)
- **Docker** - Containerization
- **Kubernetes** - Orchestration and scaling
- **.NET 8 Runtime** - Alpine-based for small image size

## Quick Start (Local Development)

### Prerequisites
- .NET 8 SDK installed
- Terminal/Command Prompt

### Run Locally

```bash
# Navigate to the project directory
cd /Users/benson.quach/benson/SANDBOX/.NET/potion-shop-k8s

# Restore dependencies
dotnet restore

# Run the application
dotnet run
```

Visit: http://localhost:5000

### Build and Run

```bash
# Build the project
dotnet build

# Run in release mode
dotnet run --configuration Release
```

## Docker Setup

### Build Docker Image

```bash
# Navigate to project directory
cd /Users/benson.quach/benson/SANDBOX/.NET/potion-shop-k8s

# Build the Docker image
docker build -t potion-shop-k8s-dotnet:latest .

# Run the container
docker run -p 5000:5000 potion-shop-k8s-dotnet:latest
```

Visit: http://localhost:5000

### Docker Image Details

The Dockerfile uses a multi-stage build:
- **Stage 1 (Build)**: Uses `mcr.microsoft.com/dotnet/sdk:8.0` to build the application
- **Stage 2 (Runtime)**: Uses `mcr.microsoft.com/dotnet/aspnet:8.0` for a smaller runtime image
- Runs as non-root user (`appuser`) for security
- Exposes port 5000
- Final image size: ~220MB (compared to Node.js ~300MB)

### Docker Compose (Optional)

Create a `docker-compose.yml`:

```yaml
version: '3.8'
services:
  potion-shop:
    build: .
    ports:
      - "5000:5000"
    environment:
      - ASPNETCORE_ENVIRONMENT=Production
      - ASPNETCORE_URLS=http://+:5000
    restart: unless-stopped
```

Run with: `docker-compose up`

## Kubernetes Deployment

### Prerequisites

- Kubernetes cluster (minikube, Docker Desktop K8s, or cloud provider)
- kubectl configured
- Docker installed

### Deploy to Kubernetes

```bash
# Build the Docker image
docker build -t potion-shop-k8s-dotnet:latest .

# For minikube, load the image into minikube
minikube image load potion-shop-k8s-dotnet:latest

# For Docker Desktop K8s, the image is already available

# Apply Kubernetes manifests
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml

# Check deployment status
kubectl get pods
kubectl get services

# Wait for pods to be ready
kubectl wait --for=condition=ready pod -l app=potion-shop-k8s-dotnet --timeout=60s
```

### Access the Application

#### For Minikube:
```bash
minikube service potion-shop-k8s-dotnet
```

#### For Docker Desktop Kubernetes:
Visit: http://localhost:30081

#### For Cloud Providers:
```bash
# Get the node IP
kubectl get nodes -o wide

# Access via NodePort
# http://<NODE_IP>:30081
```

### Kubernetes Resources

The deployment includes:
- **Deployment**: 2 replicas for high availability
- **Service**: NodePort service exposing port 30081
- **Health Checks**:
  - Liveness probe on `/api/health` (checks if app is running)
  - Readiness probe on `/api/health` (checks if app is ready to serve traffic)
- **Resource Limits**:
  - Requests: 256Mi memory, 250m CPU
  - Limits: 512Mi memory, 500m CPU

### View Logs

```bash
# View logs from all pods
kubectl logs -l app=potion-shop-k8s-dotnet

# Follow logs in real-time
kubectl logs -f deployment/potion-shop-k8s-dotnet

# View logs from a specific pod
kubectl logs <pod-name>
```

### Scale the Deployment

```bash
# Scale to 3 replicas
kubectl scale deployment potion-shop-k8s-dotnet --replicas=3

# Check scaling progress
kubectl get pods -w

# Scale back to 2
kubectl scale deployment potion-shop-k8s-dotnet --replicas=2
```

### Update the Deployment

```bash
# After making changes, rebuild the image
docker build -t potion-shop-k8s-dotnet:latest .

# Reload image (for minikube)
minikube image load potion-shop-k8s-dotnet:latest

# Restart the deployment to pick up new image
kubectl rollout restart deployment/potion-shop-k8s-dotnet

# Check rollout status
kubectl rollout status deployment/potion-shop-k8s-dotnet
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | Main shop page (MVC View) |
| GET | `/api/health` | Health check endpoint (for K8s probes) |
| GET | `/api/stats` | Get shop statistics (JSON) |
| POST | `/api/buy` | Purchase a potion (returns JSON with message and stats) |
| POST | `/api/reset` | Reset shop statistics |

### API Response Examples

#### GET /api/health
```json
{
  "status": "healthy",
  "shop": "open",
  "potionsAvailable": true
}
```

#### GET /api/stats
```json
{
  "potionsSold": 10,
  "goldEarned": 500,
  "goldPerPotion": 50
}
```

#### POST /api/buy
```json
{
  "success": true,
  "message": "You got a Red Potion! Your hearts are refilled!",
  "stats": {
    "potionsSold": 11,
    "goldEarned": 550
  }
}
```

## Testing Scenarios

1. **Basic HTTP Traces**: Visit the shop and buy potions
2. **API Calls**: Test POST request tracing with `/api/buy`
3. **JSON API**: Call the `/api/stats` endpoint
4. **Multiple Requests**: Simulate traffic with curl/browser
5. **MVC Pattern**: Test Controller → View → Client interactions
6. **Container Health**: K8s health probes monitor `/api/health`
7. **Load Balancing**: Multiple replicas distribute traffic
8. **Horizontal Scaling**: Scale replicas up/down to test load distribution

## Project Structure

```
potion-shop-k8s/
├── Controllers/
│   ├── ApiController.cs          # API endpoints
│   └── HomeController.cs         # MVC controller
├── Models/
│   ├── ShopStats.cs              # Shop statistics model
│   ├── BuyResponse.cs            # Buy endpoint response model
│   ├── StatsResponse.cs          # Stats endpoint response model
│   └── HealthResponse.cs         # Health check response model
├── Services/
│   ├── ShopStore.cs              # In-memory state management
│   └── MessageService.cs         # Purchase message generator
├── Views/
│   └── Home/
│       └── Index.cshtml          # Main shop page
├── wwwroot/
│   ├── css/
│   │   └── site.css              # Zelda-themed styles
│   └── images/
│       ├── banner.svg            # Store banner
│       └── red-potion.svg        # Potion image
├── k8s/
│   ├── deployment.yaml           # K8s deployment manifest
│   └── service.yaml              # K8s service manifest
├── Program.cs                    # Application entry point
├── Dockerfile                    # Multi-stage Docker build
├── .dockerignore                 # Docker ignore file
└── README.md                     # This file
```

## Architecture

```
┌─────────────────┐
│   Kubernetes    │
│     Service     │
│  (NodePort)     │
│   Port: 30081   │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
┌───▼───┐ ┌──▼────┐
│ Pod 1 │ │ Pod 2 │
│ :5000 │ │ :5000 │
└───────┘ └───────┘
```

## Comparison: .NET vs Node.js

| Feature | ASP.NET Core | Next.js (Node.js) |
|---------|-------------|-------------------|
| Runtime | .NET 8 | Node.js 20 |
| Language | C# | TypeScript |
| View Engine | Razor | React/JSX |
| State Management | Singleton Services | Module-level variables |
| Image Size | ~220MB | ~300MB |
| Cold Start | Faster | Slower |
| Port | 5000 | 3000 |
| NodePort | 30081 | 30080 |

## Monitoring & APM

### Using with Datadog APM

To add Datadog APM tracing:

1. Install Datadog tracer NuGet package:
```bash
dotnet add package Datadog.Trace
```

2. Update `Program.cs`:
```csharp
// Add at the top
using Datadog.Trace;

// Before building the app
builder.Services.AddSingleton<Tracer>(serviceProvider =>
{
    var settings = TracerSettings.FromDefaultSources();
    settings.ServiceName = "potion-shop-k8s-dotnet";
    settings.Environment = "kubernetes";
    return new Tracer(settings);
});
```

3. Update Kubernetes deployment environment variables:
```yaml
env:
- name: DD_AGENT_HOST
  valueFrom:
    fieldRef:
      fieldPath: status.hostIP
- name: DD_SERVICE
  value: "potion-shop-k8s-dotnet"
- name: DD_ENV
  value: "kubernetes"
- name: DD_VERSION
  value: "1.0.0"
```

### Using with Other APM Tools

The application is compatible with:
- **OpenTelemetry**: Add `OpenTelemetry.Instrumentation.AspNetCore`
- **Application Insights**: Add `Microsoft.ApplicationInsights.AspNetCore`
- **New Relic**: Add `NewRelic.Agent.Api`
- **Elastic APM**: Add `Elastic.Apm.NetCoreAll`

## Cleanup

### Remove Kubernetes Resources
```bash
kubectl delete -f k8s/deployment.yaml
kubectl delete -f k8s/service.yaml

# Or delete everything at once
kubectl delete -f k8s/
```

### Remove Docker Containers and Images
```bash
# Stop running containers
docker ps | grep potion-shop-k8s-dotnet | awk '{print $1}' | xargs docker stop

# Remove containers
docker ps -a | grep potion-shop-k8s-dotnet | awk '{print $1}' | xargs docker rm

# Remove image
docker rmi potion-shop-k8s-dotnet:latest
```

## Troubleshooting

### Container won't start
```bash
# Check container logs
docker logs <container-id>

# Check if port is already in use
lsof -i :5000
```

### Pods not ready
```bash
# Describe pod to see events
kubectl describe pod <pod-name>

# Check pod logs
kubectl logs <pod-name>

# Check health endpoint manually
kubectl port-forward <pod-name> 5000:5000
curl http://localhost:5000/api/health
```

### Image pull issues in Kubernetes
```bash
# For minikube, ensure image is loaded
minikube image ls | grep potion-shop

# If not present, reload
docker build -t potion-shop-k8s-dotnet:latest .
minikube image load potion-shop-k8s-dotnet:latest
```

## Development Notes

- **Thread Safety**: ShopStore uses locks for thread-safe in-memory state
- **State Persistence**: State is in-memory only - resets on pod restart
- **Production Ready**: For production, use a database (SQL Server, PostgreSQL, etc.)
- **Scaling**: Each pod has its own state - use Redis/database for shared state
- **Security**: Runs as non-root user in container
- **Performance**: Optimized with multi-stage Docker build

## License

This project is created for educational and testing purposes.

---

*"It's dangerous to go alone! Take a potion!" - Old Man, probably*

# Benson's Potion Store - Laravel Kubernetes Edition

A Zelda-inspired potion shop built with Laravel, containerized with Docker and ready for Kubernetes deployment. This is a single-page shop that displays 3 Red Potions for sale (50 gold each) with retro pixel art styling and fun purchase messages.

## Features

- Single-page shop interface with Zelda-inspired retro styling
- 3 Red Potions available for purchase at 50 gold each
- Session-based statistics tracking (potions sold, gold earned)
- AJAX functionality for seamless purchases without page reloads
- Random purchase messages for each transaction
- Statistics reset functionality
- Health check endpoint for Kubernetes probes
- **Docker containerized**
- **Kubernetes ready with manifests**
- High availability with 2 replicas

## Requirements

### Local Development
- PHP 8.1 or higher
- Composer
- SQLite (for session storage)

### Docker
- Docker installed and running

### Kubernetes
- Kubernetes cluster (minikube, Docker Desktop K8s, or cloud provider)
- kubectl configured

## Quick Start (Local Development)

```bash
# Navigate to the project
cd /Users/benson.quach/benson/SANDBOX/PHP/potion-shop-laravel-k8s

# Install dependencies
composer install

# Set up environment
cp .env.example .env
php artisan key:generate

# Create database and set permissions
touch database/database.sqlite
chmod -R 775 storage bootstrap/cache

# Start the server
php artisan serve
```

Visit: http://localhost:8000

## Docker Setup

### Build Docker Image

```bash
# Navigate to project directory
cd /Users/benson.quach/benson/SANDBOX/PHP/potion-shop-laravel-k8s

# Build the Docker image
docker build -t potion-shop-laravel-k8s:latest .

# Run the container
docker run -p 8080:80 potion-shop-laravel-k8s:latest
```

Visit: http://localhost:8080

### Docker Environment Variables

You can customize the container behavior with environment variables:

```bash
docker run -p 8080:80 \
  -e APP_ENV=production \
  -e APP_DEBUG=false \
  -e APP_KEY=base64:YOUR_KEY_HERE \
  potion-shop-laravel-k8s:latest
```

## Kubernetes Deployment

### Prerequisites

- Kubernetes cluster running (minikube, Docker Desktop K8s, or cloud provider)
- kubectl configured and connected to your cluster

### Deploy to Kubernetes

#### For Minikube

```bash
# Start minikube (if not already running)
minikube start

# Build the Docker image
docker build -t potion-shop-laravel-k8s:latest .

# Load the image into minikube
minikube image load potion-shop-laravel-k8s:latest

# Apply Kubernetes manifests
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml

# Check deployment status
kubectl get pods
kubectl get services

# Access the application
minikube service potion-shop-laravel-k8s

# Or get the URL
minikube service potion-shop-laravel-k8s --url
```

#### For Docker Desktop Kubernetes

```bash
# Build the Docker image
docker build -t potion-shop-laravel-k8s:latest .

# Apply Kubernetes manifests
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml

# Check deployment status
kubectl get pods
kubectl get services

# Access the application
# Visit http://localhost:30080
```

### Kubernetes Resources

The deployment includes:

- **Deployment** (`k8s/deployment.yaml`):
  - 2 replicas for high availability
  - Liveness probe on `/api/health`
  - Readiness probe on `/api/health`
  - Resource limits: CPU (200m-500m), Memory (256Mi-512Mi)

- **Service** (`k8s/service.yaml`):
  - NodePort service
  - Exposes port 30080 externally
  - Routes traffic to port 80 on pods

### Kubernetes Operations

#### View Logs

```bash
# View logs from all pods
kubectl logs -l app=potion-shop-laravel-k8s

# Follow logs from deployment
kubectl logs -f deployment/potion-shop-laravel-k8s

# View logs from specific pod
kubectl get pods
kubectl logs <pod-name>
```

#### Scale the Deployment

```bash
# Scale to 3 replicas
kubectl scale deployment potion-shop-laravel-k8s --replicas=3

# Check scaling
kubectl get pods -w

# Scale back to 2
kubectl scale deployment potion-shop-laravel-k8s --replicas=2
```

#### Check Health

```bash
# Check pod status
kubectl get pods

# Describe deployment
kubectl describe deployment potion-shop-laravel-k8s

# Check health endpoint directly
kubectl port-forward deployment/potion-shop-laravel-k8s 8080:80
curl http://localhost:8080/api/health
```

#### Restart Deployment

```bash
# Rolling restart
kubectl rollout restart deployment/potion-shop-laravel-k8s

# Check rollout status
kubectl rollout status deployment/potion-shop-laravel-k8s
```

### Cleanup Kubernetes Resources

```bash
# Remove all resources
kubectl delete -f k8s/

# Or delete individually
kubectl delete deployment potion-shop-laravel-k8s
kubectl delete service potion-shop-laravel-k8s

# Verify removal
kubectl get all
```

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
Health check endpoint (used by Kubernetes probes)
- **Response:**
  ```json
  {
    "status": "ok"
  }
  ```

## Testing Scenarios

1. **Basic HTTP Traces**: Visit the shop and buy potions
2. **API Calls**: Test POST request tracing with `/api/buy`
3. **JSON API**: Call the `/api/stats` endpoint
4. **Multiple Requests**: Simulate traffic with fetch/browser
5. **Container Health**: Kubernetes health probes monitor `/api/health`
6. **Load Balancing**: Multiple replicas distribute traffic
7. **Pod Restart**: Test pod resilience with `kubectl delete pod <pod-name>`
8. **Scaling**: Scale up/down and observe load distribution

## Architecture

```
┌──────────────────┐
│   Kubernetes     │
│     Service      │
│   (NodePort)     │
│   Port: 30080    │
└────────┬─────────┘
         │
    ┌────┴────┐
    │         │
┌───▼────┐ ┌─▼──────┐
│ Pod 1  │ │ Pod 2  │
│ Port80 │ │ Port80 │
└────────┘ └────────┘
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
potion-shop-laravel-k8s/
├── app/
│   └── Http/
│       └── Controllers/
│           └── ShopController.php      # Main controller
├── resources/
│   └── views/
│       └── shop.blade.php              # Main shop view
├── routes/
│   ├── web.php                         # Web routes
│   └── api.php                         # API routes
├── k8s/
│   ├── deployment.yaml                 # K8s deployment config
│   └── service.yaml                    # K8s service config
├── Dockerfile                          # Docker image definition
├── .dockerignore                       # Docker ignore rules
├── .env.example                        # Environment template
└── README.md                           # This file
```

## Technology Stack

- **Backend:** Laravel 11
- **Frontend:** Vanilla JavaScript with AJAX
- **Styling:** Inline CSS with Zelda-inspired retro theme
- **Containerization:** Docker with PHP 8.2 + Apache
- **Orchestration:** Kubernetes
- **Session Storage:** File-based sessions

## Design Features

- Retro pixel art style
- Green gradient background (#1a472a to #0d2818)
- Golden borders and accents
- Animated elements (floating banner, sparkling cards, rotating gold coins)
- Responsive grid layout for potion cards
- Heart beat animation for health indicators
- Pulse animation for purchase messages

## Notes

- Statistics are stored in PHP sessions (file-based)
- Each pod has its own session storage (sessions are not shared between pods)
- For production with multiple replicas, consider using Redis or database sessions
- The application uses CSRF protection for all POST requests
- Health probes ensure pods are ready before receiving traffic

## Troubleshooting

### Docker Issues

**Problem:** Permission errors in container
```bash
# Solution: Rebuild with proper permissions
docker build --no-cache -t potion-shop-laravel-k8s:latest .
```

**Problem:** Port already in use
```bash
# Solution: Use different port
docker run -p 8081:80 potion-shop-laravel-k8s:latest
```

### Kubernetes Issues

**Problem:** Pods not starting
```bash
# Check pod status and logs
kubectl describe pod <pod-name>
kubectl logs <pod-name>
```

**Problem:** Image pull errors
```bash
# For minikube, make sure image is loaded
minikube image load potion-shop-laravel-k8s:latest

# For Docker Desktop, make sure imagePullPolicy is Never in deployment.yaml
```

**Problem:** Can't access service
```bash
# Check service and endpoints
kubectl get service potion-shop-laravel-k8s
kubectl get endpoints potion-shop-laravel-k8s

# For minikube, use service command
minikube service potion-shop-laravel-k8s
```

## Credits

Inspired by the classic Zelda potion shops and based on the Next.js versions at:
- `/Users/benson.quach/benson/SANDBOX/Node.js/potion-shop`
- `/Users/benson.quach/benson/SANDBOX/Node.js/potion-shop-k8s`

---

*"It's dangerous to go alone! Take a potion!" - Benson, probably*

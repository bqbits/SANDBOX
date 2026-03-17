# Quick Start Guide

## Local Development (Fastest)

```bash
cd /Users/benson.quach/benson/SANDBOX/.NET/potion-shop-k8s
dotnet run
```
Visit: http://localhost:5000

## Docker (Recommended for Testing)

```bash
cd /Users/benson.quach/benson/SANDBOX/.NET/potion-shop-k8s

# Build
docker build -t potion-shop-k8s-dotnet:latest .

# Run
docker run -p 5000:5000 potion-shop-k8s-dotnet:latest
```
Visit: http://localhost:5000

## Kubernetes (Full Experience)

### For Minikube:
```bash
cd /Users/benson.quach/benson/SANDBOX/.NET/potion-shop-k8s

# Build and load image
docker build -t potion-shop-k8s-dotnet:latest .
minikube image load potion-shop-k8s-dotnet:latest

# Deploy
kubectl apply -f k8s/

# Access
minikube service potion-shop-k8s-dotnet
```

### For Docker Desktop Kubernetes:
```bash
cd /Users/benson.quach/benson/SANDBOX/.NET/potion-shop-k8s

# Build
docker build -t potion-shop-k8s-dotnet:latest .

# Deploy
kubectl apply -f k8s/

# Access
```
Visit: http://localhost:30081

## Test API Endpoints

```bash
# Health check
curl http://localhost:5000/api/health

# Get stats
curl http://localhost:5000/api/stats

# Buy a potion
curl -X POST http://localhost:5000/api/buy

# Reset stats
curl -X POST http://localhost:5000/api/reset
```

## Cleanup

### Stop local development:
Press `Ctrl+C`

### Stop Docker:
```bash
docker ps | grep potion-shop-k8s-dotnet | awk '{print $1}' | xargs docker stop
```

### Delete from Kubernetes:
```bash
kubectl delete -f k8s/
```

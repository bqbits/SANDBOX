# Potion Shop - Spring Boot K8s Edition

A Zelda-themed potion shop application built with Spring Boot 3.x, designed for Kubernetes deployment. This is a Spring Boot port of the Next.js potion shop application, featuring the same retro aesthetic and functionality.

## Features

- **Retro Zelda Theme**: Nostalgic pixel-art design with animated elements
- **REST API**: Fully functional backend with JSON endpoints
- **In-Memory Storage**: Simple state management without database dependencies
- **Kubernetes Ready**: Optimized for containerized deployment with health checks
- **Multi-Stage Docker Build**: Efficient container images using Maven and JDK 17
- **Production Ready**: Resource limits, non-root user execution, and health probes

## Tech Stack

- **Spring Boot 3.2.0** - Modern Java framework
- **Java 17** - LTS Java version
- **Maven** - Build and dependency management
- **Thymeleaf** - HTML templating engine
- **Docker** - Container runtime
- **Kubernetes** - Container orchestration

## Project Structure

```
potion-shop-springboot-k8s/
├── src/
│   ├── main/
│   │   ├── java/com/benson/potionshop/
│   │   │   ├── PotionShopApplication.java      # Main Spring Boot application
│   │   │   ├── controller/
│   │   │   │   └── ShopController.java         # REST API endpoints
│   │   │   ├── service/
│   │   │   │   ├── ShopService.java            # Business logic
│   │   │   │   └── MessageService.java         # Random message generation
│   │   │   └── model/
│   │   │       └── ShopStats.java              # Shop statistics model
│   │   └── resources/
│   │       ├── application.properties          # Spring Boot configuration
│   │       ├── static/
│   │       │   ├── css/style.css              # Retro Zelda styling
│   │       │   ├── js/shop.js                 # Frontend JavaScript
│   │       │   └── images/
│   │       │       ├── banner.svg             # Shop banner
│   │       │       └── red-potion.svg         # Potion sprite
│   │       └── templates/
│   │           └── index.html                 # Main page template
├── k8s/
│   ├── deployment.yaml                         # Kubernetes deployment
│   └── service.yaml                            # Kubernetes service
├── Dockerfile                                  # Multi-stage Docker build
├── .dockerignore                               # Docker ignore rules
├── pom.xml                                     # Maven dependencies
└── README.md                                   # This file
```

## API Endpoints

### REST API

- **POST /api/buy** - Purchase a potion (increments stats, returns random message)
  ```json
  Response: {
    "success": true,
    "message": "You got a Red Potion! Your hearts are refilled!",
    "stats": {
      "potions_sold": 1,
      "gold_earned": 50
    }
  }
  ```

- **GET /api/stats** - Get shop statistics
  ```json
  Response: {
    "potions_sold": 1,
    "gold_earned": 50,
    "gold_per_potion": 50
  }
  ```

- **POST /api/reset** - Reset statistics
  ```json
  Response: {
    "success": true,
    "message": "Shop statistics have been reset",
    "stats": {
      "potions_sold": 0,
      "gold_earned": 0
    }
  }
  ```

- **GET /api/health** - Health check (for K8s probes)
  ```json
  Response: {
    "status": "healthy",
    "shop": "open",
    "potions_available": true
  }
  ```

### Frontend

- **GET /** - Serve the potion shop HTML page

## Local Development

### Prerequisites

- Java 17 or higher
- Maven 3.9 or higher

### Run Locally

1. **Clone the repository**
   ```bash
   cd /Users/benson.quach/benson/SANDBOX/Java/potion-shop-springboot-k8s
   ```

2. **Build the application**
   ```bash
   mvn clean package
   ```

3. **Run the application**
   ```bash
   java -jar target/potion-shop-1.0.0.jar
   ```

4. **Access the application**
   - Open your browser to: http://localhost:8080
   - API health check: http://localhost:8080/api/health
   - API stats: http://localhost:8080/api/stats

### Development Mode

For hot-reload during development:
```bash
mvn spring-boot:run
```

## Docker Deployment

### Build Docker Image

```bash
cd /Users/benson.quach/benson/SANDBOX/Java/potion-shop-springboot-k8s
docker build -t potion-shop-springboot-k8s:latest .
```

The Dockerfile uses a multi-stage build:
- **Stage 1**: Maven build with `maven:3.9-eclipse-temurin-17`
- **Stage 2**: Runtime image with `eclipse-temurin:17-jre-alpine` (minimal size)

### Run Docker Container

```bash
docker run -d \
  --name potion-shop \
  -p 8080:8080 \
  potion-shop-springboot-k8s:latest
```

### Test the Container

```bash
# Check health
curl http://localhost:8080/api/health

# Get stats
curl http://localhost:8080/api/stats

# Buy a potion
curl -X POST http://localhost:8080/api/buy

# Open browser
open http://localhost:8080
```

### Stop and Remove Container

```bash
docker stop potion-shop
docker rm potion-shop
```

## Kubernetes Deployment

### Prerequisites

- Kubernetes cluster (minikube, kind, or cloud provider)
- kubectl CLI configured

### Deploy to Kubernetes

1. **Build the Docker image** (if using minikube/kind)
   ```bash
   # For minikube
   eval $(minikube docker-env)
   docker build -t potion-shop-springboot-k8s:latest .

   # For kind
   docker build -t potion-shop-springboot-k8s:latest .
   kind load docker-image potion-shop-springboot-k8s:latest
   ```

2. **Apply Kubernetes manifests**
   ```bash
   kubectl apply -f k8s/deployment.yaml
   kubectl apply -f k8s/service.yaml
   ```

3. **Verify deployment**
   ```bash
   # Check pods
   kubectl get pods -l app=potion-shop-springboot-k8s

   # Check deployment
   kubectl get deployment potion-shop-springboot-k8s

   # Check service
   kubectl get service potion-shop-springboot-k8s

   # View logs
   kubectl logs -l app=potion-shop-springboot-k8s
   ```

4. **Access the application**
   ```bash
   # For minikube
   minikube service potion-shop-springboot-k8s

   # For other clusters (NodePort 30080)
   kubectl get nodes -o wide
   # Then access: http://<NODE-IP>:30080
   ```

### Kubernetes Configuration Details

**Deployment (`k8s/deployment.yaml`)**:
- **Replicas**: 2 pods for high availability
- **Resources**:
  - Requests: 256Mi memory, 250m CPU
  - Limits: 512Mi memory, 500m CPU
- **Liveness Probe**: Checks `/api/health` every 10s (after 30s initial delay)
- **Readiness Probe**: Checks `/api/health` every 5s (after 10s initial delay)
- **Image Pull Policy**: IfNotPresent (for local development)

**Service (`k8s/service.yaml`)**:
- **Type**: NodePort
- **Port**: 8080 (internal)
- **NodePort**: 30080 (external access)

### Update Deployment

```bash
# Rebuild image
docker build -t potion-shop-springboot-k8s:latest .

# Reload image (for minikube/kind)
eval $(minikube docker-env)  # or kind load docker-image
docker build -t potion-shop-springboot-k8s:latest .

# Restart deployment
kubectl rollout restart deployment potion-shop-springboot-k8s

# Watch rollout status
kubectl rollout status deployment potion-shop-springboot-k8s
```

### Scale Deployment

```bash
# Scale up
kubectl scale deployment potion-shop-springboot-k8s --replicas=3

# Scale down
kubectl scale deployment potion-shop-springboot-k8s --replicas=1
```

### Clean Up

```bash
# Delete all resources
kubectl delete -f k8s/deployment.yaml
kubectl delete -f k8s/service.yaml

# Or delete by label
kubectl delete all -l app=potion-shop-springboot-k8s
```

## Monitoring and Debugging

### Check Application Logs

```bash
# View logs from all pods
kubectl logs -l app=potion-shop-springboot-k8s --tail=100

# Follow logs
kubectl logs -l app=potion-shop-springboot-k8s -f

# Logs from specific pod
kubectl logs <pod-name>
```

### Describe Resources

```bash
# Describe deployment
kubectl describe deployment potion-shop-springboot-k8s

# Describe pod
kubectl describe pod <pod-name>

# Describe service
kubectl describe service potion-shop-springboot-k8s
```

### Execute Commands in Pod

```bash
# Get shell access
kubectl exec -it <pod-name> -- /bin/sh

# Test health endpoint from inside pod
kubectl exec <pod-name> -- wget -qO- http://localhost:8080/api/health
```

### Port Forward (for testing)

```bash
# Forward local port 8080 to pod port 8080
kubectl port-forward deployment/potion-shop-springboot-k8s 8080:8080

# Access via: http://localhost:8080
```

## Business Logic

- **Red Potion Price**: 50 gold
- **Stats Tracked**: Total potions sold and gold earned
- **Purchase Messages**: 8 different Zelda-themed messages
- **Storage**: In-memory (resets on pod restart)

## Security Features

- **Non-root User**: Container runs as `spring` user
- **Resource Limits**: CPU and memory constraints
- **Health Checks**: Liveness and readiness probes
- **Minimal Image**: Alpine-based JRE (smaller attack surface)

## Performance

- **Startup Time**: ~30 seconds (configured in liveness probe)
- **Ready Time**: ~10 seconds (configured in readiness probe)
- **Image Size**: ~180-200 MB (Alpine JRE base)
- **Memory Usage**: ~256 MB under normal load

## Troubleshooting

### Pod CrashLoopBackOff

```bash
# Check logs
kubectl logs <pod-name>

# Common issues:
# - Health check failing (check /api/health endpoint)
# - Insufficient memory (increase resource limits)
# - Port already in use
```

### ImagePullBackOff

```bash
# For local clusters, ensure image is loaded
minikube image ls | grep potion-shop
kind load docker-image potion-shop-springboot-k8s:latest

# Or set imagePullPolicy: IfNotPresent
```

### Service Not Accessible

```bash
# Check service endpoints
kubectl get endpoints potion-shop-springboot-k8s

# Verify pods are running
kubectl get pods -l app=potion-shop-springboot-k8s

# Check NodePort
kubectl get service potion-shop-springboot-k8s -o yaml
```

## Differences from Next.js Version

| Feature | Next.js Version | Spring Boot Version |
|---------|----------------|---------------------|
| Runtime | Node.js 18+ | Java 17 (JRE) |
| Framework | Next.js 14 | Spring Boot 3.2 |
| Port | 3000 | 8080 |
| Build Tool | npm | Maven |
| Template Engine | React (JSX) | Thymeleaf |
| Image Size | ~400 MB | ~200 MB |
| Startup Time | ~5s | ~30s |

## License

MIT License - Feel free to use this project for learning and experimentation!

## Credits

- Inspired by The Legend of Zelda series
- Spring Boot port of the Next.js potion shop application
- Created by Benson Quach

---

**"It's dangerous to go alone! Take a potion!"** - Old Man, probably

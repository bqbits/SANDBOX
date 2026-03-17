# Benson's Potion Store (Kubernetes Edition - Rails)

A Zelda-inspired Ruby on Rails potion shop with Docker and Kubernetes support. This is the Rails implementation that mirrors the functionality of the Next.js version, perfect for testing containerized Rails applications and APM tools.

## Features

- Retro pixel art Zelda-style theme
- Red Potions for sale (50 gold each)
- Real-time shop statistics
- Fun purchase messages (8 randomized messages)
- Lightweight and perfect for testing
- Built with Ruby on Rails 8 and SQLite
- **Docker containerized with multi-stage build**
- **Kubernetes ready with manifests**
- Health check endpoints for K8s probes
- Production-optimized with Thruster HTTP server

## Quick Start (Local Development)

### Prerequisites

- Ruby 3.2.2 (use rbenv or similar)
- Bundler
- SQLite3

### Setup

```bash
# Navigate to the shop
cd /Users/benson.quach/benson/SANDBOX/Ruby/potion-shop-rails-k8s

# Install dependencies
bundle install

# Setup database
rails db:create db:migrate

# Run the development server
rails server
```

Visit: http://localhost:3000

## Docker Setup

### Build Docker Image

```bash
# Build the Docker image
docker build -t potion-shop-rails-k8s:latest .

# Run the container
docker run -p 3000:3000 -e SECRET_KEY_BASE=change_me_in_production potion-shop-rails-k8s:latest
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
      - RAILS_ENV=production
      - SECRET_KEY_BASE=change_me_in_production
      - RAILS_LOG_TO_STDOUT=true
      - RAILS_SERVE_STATIC_FILES=true
    restart: unless-stopped
```

Run with: `docker-compose up`

## Kubernetes Deployment

### Prerequisites

- Kubernetes cluster (minikube, Docker Desktop K8s, or cloud provider)
- kubectl configured
- Docker for building images

### Deploy to Kubernetes

```bash
# Build and load image (for local K8s like minikube)
docker build -t potion-shop-rails-k8s:latest .

# For minikube, load the image
minikube image load potion-shop-rails-k8s:latest

# Apply Kubernetes manifests
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml

# Check deployment status
kubectl get pods
kubectl get services

# Access the application
# For minikube:
minikube service potion-shop-rails-k8s

# For Docker Desktop K8s:
# Visit http://localhost:30080
```

### Kubernetes Resources

The deployment includes:
- **Deployment**: 2 replicas for high availability
- **Service**: NodePort service exposing port 30080
- **Health Checks**: Liveness and readiness probes on `/api/health`
- **Resource Limits**:
  - Requests: 256Mi memory, 250m CPU
  - Limits: 512Mi memory, 500m CPU
- **Environment Variables**: Pre-configured for containerized Rails

### View Logs

```bash
# View logs from all pods
kubectl logs -l app=potion-shop-rails-k8s

# Follow logs
kubectl logs -f deployment/potion-shop-rails-k8s

# View logs from a specific pod
kubectl logs <pod-name>
```

### Scale the Deployment

```bash
# Scale to 3 replicas
kubectl scale deployment potion-shop-rails-k8s --replicas=3

# Check scaling
kubectl get pods -w
```

### Update the Deployment

```bash
# Rebuild image
docker build -t potion-shop-rails-k8s:latest .

# Load to minikube (if using minikube)
minikube image load potion-shop-rails-k8s:latest

# Restart deployment to pick up new image
kubectl rollout restart deployment/potion-shop-rails-k8s

# Check rollout status
kubectl rollout status deployment/potion-shop-rails-k8s
```

## API Endpoints

- `GET /` - Main shop page with Zelda UI
- `POST /api/buy` - Purchase a potion (returns JSON with message and stats)
- `GET /api/stats` - Get shop statistics (JSON)
- `POST /api/reset` - Reset shop statistics
- `GET /api/health` - Health check endpoint (used by K8s probes)

### Example API Usage

```bash
# Get current stats
curl http://localhost:3000/api/stats

# Buy a potion
curl -X POST http://localhost:3000/api/buy

# Reset statistics
curl -X POST http://localhost:3000/api/reset

# Health check
curl http://localhost:3000/api/health
```

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
│ Rails │ │ Rails │
│SQLite │ │SQLite │
└───────┘ └───────┘
```

Note: Each pod has its own SQLite database. For production use with shared state, consider using PostgreSQL with a separate database pod.

## Tech Stack

- **Ruby on Rails 8** - Web framework
- **Ruby 3.2.2** - Programming language
- **SQLite3** - Database (containerized, per-pod)
- **Thruster** - Fast HTTP server for production
- **Docker** - Containerization with multi-stage builds
- **Kubernetes** - Orchestration and scaling

## Testing Scenarios

1. **Basic HTTP Traces**: Visit the shop and buy potions
2. **API Calls**: Test POST request tracing with /api/buy
3. **JSON API**: Call the `/api/stats` endpoint
4. **Multiple Requests**: Simulate traffic with curl/browser
5. **Health Probes**: K8s automatically monitors `/api/health`
6. **Load Balancing**: Multiple replicas distribute traffic
7. **Container Logs**: View logs via kubectl for debugging

## Production Considerations

### Database

The current setup uses SQLite, which means each pod has its own database. This is fine for testing but not ideal for production where you want shared state. Consider:

1. **PostgreSQL**: Update `config/database.yml` to use PostgreSQL
2. **Separate DB Pod**: Deploy PostgreSQL in Kubernetes
3. **Cloud Database**: Use managed database services (RDS, Cloud SQL, etc.)

### Secrets Management

The deployment includes a placeholder `SECRET_KEY_BASE`. For production:

```bash
# Generate a secure key
rails secret

# Create a Kubernetes secret
kubectl create secret generic rails-secrets \
  --from-literal=secret-key-base=<your-generated-key>

# Update deployment.yaml to use the secret:
env:
- name: SECRET_KEY_BASE
  valueFrom:
    secretKeyRef:
      name: rails-secrets
      key: secret-key-base
```

### File Storage

For uploaded files or persistent assets, consider:
- Kubernetes Persistent Volumes
- Cloud storage (S3, GCS, Azure Blob)
- Update `config/storage.yml` accordingly

### Monitoring and APM

This application is perfect for testing APM tools like Datadog:

```bash
# Add to Gemfile
gem 'ddtrace'

# Initialize in config/initializers/datadog.rb
Datadog.configure do |c|
  c.tracing.enabled = true
  c.service = 'potion-shop-rails-k8s'
  c.env = 'kubernetes'
  c.version = '1.0.0'
end

# Update deployment.yaml with Datadog env vars
env:
- name: DD_AGENT_HOST
  valueFrom:
    fieldRef:
      fieldPath: status.hostIP
```

## Cleanup

```bash
# Remove Kubernetes resources
kubectl delete -f k8s/

# Remove Docker containers
docker ps | grep potion-shop-rails-k8s | awk '{print $1}' | xargs docker stop
docker images | grep potion-shop-rails-k8s | awk '{print $3}' | xargs docker rmi
```

## Differences from Node.js Version

While this Rails version mirrors the functionality of the Next.js version, there are some key differences:

1. **Framework**: Rails 8 vs Next.js 14
2. **Language**: Ruby 3.2.2 vs Node.js 20
3. **Database**: SQLite (Rails) vs in-memory (Next.js)
4. **Server**: Thruster HTTP server vs Next.js standalone server
5. **Assets**: Rails asset pipeline vs Next.js static exports
6. **Port**: Both use port 3000 for consistency
7. **Health Endpoint**: `/api/health` (custom) vs `/api/health` (both support K8s)

Both versions provide equivalent functionality for testing containerized applications in Kubernetes!

## Development

### Project Structure

```
potion-shop-rails-k8s/
├── app/
│   ├── controllers/
│   │   ├── api/shop_controller.rb  # API endpoints
│   │   └── shop_controller.rb       # Main page
│   ├── models/
│   │   └── shop_stat.rb             # Statistics model
│   ├── views/
│   │   └── shop/
│   │       └── index.html.erb       # Main shop UI
│   └── assets/
│       ├── stylesheets/
│       │   └── application.css      # Zelda theme CSS
│       └── images/
│           ├── banner.svg           # Shop banner
│           └── red-potion.svg       # Potion icon
├── config/
│   ├── routes.rb                    # API and page routes
│   ├── database.yml                 # Database config
│   └── environments/
│       └── production.rb            # Production settings
├── db/
│   ├── migrate/                     # Database migrations
│   └── schema.rb                    # Database schema
├── k8s/
│   ├── deployment.yaml              # K8s deployment manifest
│   └── service.yaml                 # K8s service manifest
├── Dockerfile                       # Multi-stage production build
├── .dockerignore                    # Docker build exclusions
└── README.md                        # This file
```

### Adding New Features

1. **New API Endpoint**: Add to `app/controllers/api/shop_controller.rb` and update `config/routes.rb`
2. **UI Changes**: Modify `app/views/shop/index.html.erb` and `app/assets/stylesheets/application.css`
3. **Database Changes**: Generate migration with `rails generate migration`
4. **Testing**: Rebuild Docker image and test in Kubernetes

### Debugging

```bash
# Local development
rails console

# View routes
rails routes

# Check database
rails dbconsole

# In Kubernetes
kubectl exec -it <pod-name> -- rails console
kubectl describe pod <pod-name>
kubectl logs <pod-name>
```

---

*"It's dangerous to go alone! Take a potion!" - Benson, probably*

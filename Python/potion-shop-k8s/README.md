# Benson's Potion Shop - Kubernetes Deployment

This is a Kubernetes-ready version of Benson's Potion Shop Flask application with Datadog APM instrumentation for testing in a Kubernetes cluster.

## Prerequisites

- Kubernetes cluster (minikube, kind, Docker Desktop, or cloud provider)
- `kubectl` configured to access your cluster
- Docker installed for building images
- Datadog Agent installed in your Kubernetes cluster (in the `datadog` namespace)
- Datadog API key

## Project Structure

```
potion-shop-k8s/
├── app.py                          # Flask application
├── requirements.txt                # Python dependencies
├── Dockerfile                      # Multi-stage Docker build
├── .dockerignore                   # Docker build exclusions
├── README.md                       # This file
├── static/                         # Static assets
│   └── images/
│       └── potion.png
├── templates/                      # HTML templates
│   └── index.html
└── k8s/                           # Kubernetes manifests
    ├── 01-namespace.yaml          # Creates bensons-shop namespace
    ├── 02-datadog-secret.yaml.template  # Datadog API key (template)
    ├── 03-configmap.yaml          # Datadog APM configuration
    ├── 04-deployment.yaml         # Application deployment (2 replicas)
    └── 05-service.yaml            # ClusterIP service
```

## Quick Start

### 1. Build the Docker Image

```bash
cd /Users/benson.quach/benson/SANDBOX/Python/potion-shop-k8s

# Build the image
docker build -t bensons-potion-shop:latest .

# If using minikube, load the image into minikube
minikube image load bensons-potion-shop:latest

# If using kind, load the image into kind
kind load docker-image bensons-potion-shop:latest
```

### 2. Configure Datadog Secret (Optional)

If you need the Datadog API key as a secret (for standalone mode):

```bash
# Copy the template
cp k8s/02-datadog-secret.yaml.template k8s/02-datadog-secret.yaml

# Edit the file and replace YOUR_DATADOG_API_KEY with your actual key
# Then apply it
kubectl apply -f k8s/02-datadog-secret.yaml
```

**Note:** If using the Datadog Agent already installed in your cluster, you typically don't need this secret. The application will send traces to the Datadog Agent via the hostname specified in the ConfigMap.

### 3. Deploy to Kubernetes

```bash
# Apply all manifests in order
kubectl apply -f k8s/01-namespace.yaml
kubectl apply -f k8s/03-configmap.yaml
kubectl apply -f k8s/04-deployment.yaml
kubectl apply -f k8s/05-service.yaml
```

Or apply all at once:

```bash
kubectl apply -f k8s/
```

### 4. Verify Deployment

```bash
# Check pods are running
kubectl get pods -n bensons-shop

# Check deployment status
kubectl get deployment -n bensons-shop

# Check service
kubectl get service -n bensons-shop

# View pod logs
kubectl logs -n bensons-shop -l app=bensons-shop --tail=50

# Follow logs in real-time
kubectl logs -n bensons-shop -l app=bensons-shop -f
```

### 5. Access the Application

**Option A: Port Forward (Recommended for testing)**

```bash
kubectl port-forward -n bensons-shop service/bensons-shop 8080:80
```

Then access at: http://localhost:8080

**Option B: LoadBalancer Service (Cloud providers)**

Uncomment the LoadBalancer section in `k8s/05-service.yaml` and reapply:

```bash
kubectl apply -f k8s/05-service.yaml
kubectl get service bensons-shop-external -n bensons-shop
```

Wait for the EXTERNAL-IP to be assigned, then access the application at that IP.

## Datadog APM Configuration

The application is configured to send traces to the Datadog Agent at:
- **Hostname:** `datadog-agent.datadog.svc.cluster.local`
- **Port:** `8126`

### Configuration (in k8s/03-configmap.yaml)

```yaml
DD_SERVICE: "bensons-potion-shop"
DD_ENV: "kubernetes"
DD_VERSION: "1.0.0"
DD_TRACE_ENABLED: "true"
DD_LOGS_INJECTION: "true"
DD_TRACE_SAMPLE_RATE: "1.0"
DD_TRACE_AGENT_HOSTNAME: "datadog-agent.datadog.svc.cluster.local"
DD_AGENT_HOST: "datadog-agent.datadog.svc.cluster.local"
DD_TRACE_AGENT_PORT: "8126"
```

### Verify Datadog Traces

1. Generate traffic to your application:
```bash
# Port forward to access the app
kubectl port-forward -n bensons-shop service/bensons-shop 8080:80

# In another terminal, generate traffic
for i in {1..10}; do curl http://localhost:8080/; done
```

2. Check Datadog APM:
   - Navigate to https://app.datadoghq.com/apm/traces
   - Filter by service: `bensons-potion-shop`
   - Filter by env: `kubernetes`

3. Check application logs for Datadog initialization:
```bash
kubectl logs -n bensons-shop -l app=bensons-shop | grep -i datadog
```

## Testing Endpoints

The application has the following endpoints:

- **`GET /`** - Main page showing potion inventory
- **`GET /health`** - Health check endpoint (used by Kubernetes probes)

## Deployment Details

### Health Checks

The deployment includes three types of probes:

1. **Liveness Probe:** Checks if the app is responsive (restarts if failing)
   - Path: `/health`
   - Initial delay: 10s
   - Period: 30s

2. **Readiness Probe:** Checks if the app is ready to serve traffic
   - Path: `/health`
   - Initial delay: 5s
   - Period: 10s

3. **Startup Probe:** Checks if the app has started (for slow starts)
   - Path: `/health`
   - Initial delay: 5s
   - Period: 5s
   - Max failures: 30 (150s total)

### Resource Limits

```yaml
resources:
  requests:
    cpu: 100m
    memory: 128Mi
  limits:
    cpu: 500m
    memory: 512Mi
```

### Security

- Runs as non-root user (UID: 1000)
- Uses multi-stage Docker build for smaller image
- Minimal base image (python:3.13-slim)

## Troubleshooting

### Pods Not Starting

```bash
# Describe pod to see events
kubectl describe pod -n bensons-shop -l app=bensons-shop

# Check pod logs
kubectl logs -n bensons-shop -l app=bensons-shop --all-containers=true
```

### Image Pull Issues

If using local images with minikube/kind, ensure you loaded the image:

```bash
# For minikube
minikube image ls | grep bensons-potion-shop

# For kind
docker exec -it <kind-control-plane-container> crictl images | grep bensons-potion-shop
```

### Datadog Traces Not Appearing

1. Verify Datadog Agent is running:
```bash
kubectl get pods -n datadog
```

2. Check if agent service is accessible:
```bash
kubectl run -it --rm debug --image=busybox --restart=Never -- \
  nc -zv datadog-agent.datadog.svc.cluster.local 8126
```

3. Check application logs for trace submission:
```bash
kubectl logs -n bensons-shop -l app=bensons-shop | grep -i trace
```

4. Verify ConfigMap is mounted correctly:
```bash
kubectl exec -n bensons-shop -it <pod-name> -- env | grep DD_
```

### Health Check Failures

Check if the `/health` endpoint is responding:

```bash
kubectl exec -n bensons-shop -it <pod-name> -- \
  curl -v http://localhost:5000/health
```

## Scaling

### Manual Scaling

```bash
# Scale to 3 replicas
kubectl scale deployment bensons-shop -n bensons-shop --replicas=3

# Verify
kubectl get pods -n bensons-shop
```

### Auto-scaling (HPA)

Create a HorizontalPodAutoscaler:

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: bensons-shop-hpa
  namespace: bensons-shop
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: bensons-shop
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

Apply with:
```bash
kubectl apply -f k8s/06-hpa.yaml
```

## Cleanup

To remove the deployment:

```bash
# Delete all resources
kubectl delete -f k8s/

# Or delete the entire namespace
kubectl delete namespace bensons-shop
```

## Differences from Local Version

This Kubernetes version differs from the local `potion-shop` in:

1. **Environment:** Configured for `DD_ENV=kubernetes` instead of `DD_ENV=local`
2. **Agent Hostname:** Points to Kubernetes service instead of localhost
3. **Containerization:** Includes Dockerfile and .dockerignore
4. **Orchestration:** Kubernetes manifests for deployment, service, configmap
5. **Health Checks:** Added `/health` endpoint probes
6. **Scaling:** Supports horizontal pod autoscaling
7. **Security:** Non-root user, resource limits, security context

## Next Steps

- Configure Ingress for external access
- Set up HorizontalPodAutoscaler for automatic scaling
- Add PersistentVolume if data persistence is needed
- Configure network policies for security
- Set up monitoring and alerting in Datadog
- Integrate with CI/CD pipeline for automated deployments

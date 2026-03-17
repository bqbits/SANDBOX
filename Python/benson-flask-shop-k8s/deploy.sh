#!/bin/bash
# Deployment script for Benson's Potion Shop on Kubernetes

set -e  # Exit on error

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Benson's Potion Shop - K8s Deployment${NC}"
echo -e "${GREEN}========================================${NC}\n"

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

if ! command -v kubectl &> /dev/null; then
    echo -e "${RED}Error: kubectl is not installed${NC}"
    exit 1
fi

if ! command -v docker &> /dev/null; then
    echo -e "${RED}Error: docker is not installed${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Prerequisites OK${NC}\n"

# Build Docker image
echo -e "${YELLOW}Building Docker image...${NC}"
docker build -t bensons-potion-shop:latest .
echo -e "${GREEN}✓ Image built successfully${NC}\n"

# Detect Kubernetes environment
echo -e "${YELLOW}Detecting Kubernetes environment...${NC}"
K8S_CONTEXT=$(kubectl config current-context)
echo "Current context: $K8S_CONTEXT"

if [[ "$K8S_CONTEXT" == *"minikube"* ]]; then
    echo -e "${YELLOW}Detected minikube - loading image...${NC}"
    minikube image load bensons-potion-shop:latest
    echo -e "${GREEN}✓ Image loaded into minikube${NC}\n"
elif [[ "$K8S_CONTEXT" == *"kind"* ]]; then
    echo -e "${YELLOW}Detected kind - loading image...${NC}"
    kind load docker-image bensons-potion-shop:latest
    echo -e "${GREEN}✓ Image loaded into kind${NC}\n"
else
    echo -e "${YELLOW}Cloud or other K8s environment detected${NC}"
    echo -e "${YELLOW}You may need to push the image to a registry${NC}\n"
fi

# Apply Kubernetes manifests
echo -e "${YELLOW}Deploying to Kubernetes...${NC}"

echo "Applying namespace..."
kubectl apply -f k8s/01-namespace.yaml

echo "Applying configmap..."
kubectl apply -f k8s/03-configmap.yaml

echo "Applying deployment..."
kubectl apply -f k8s/04-deployment.yaml

echo "Applying service..."
kubectl apply -f k8s/05-service.yaml

# Apply HPA if it exists
if [ -f "k8s/06-hpa.yaml" ]; then
    echo "Applying HPA..."
    kubectl apply -f k8s/06-hpa.yaml
fi

echo -e "${GREEN}✓ Kubernetes resources applied${NC}\n"

# Wait for deployment to be ready
echo -e "${YELLOW}Waiting for deployment to be ready...${NC}"
kubectl wait --for=condition=available --timeout=120s \
    deployment/bensons-shop -n bensons-shop || true

# Show deployment status
echo -e "\n${GREEN}Deployment Status:${NC}"
kubectl get all -n bensons-shop

# Show logs
echo -e "\n${YELLOW}Recent logs:${NC}"
kubectl logs -n bensons-shop -l app=bensons-shop --tail=20 || true

# Instructions
echo -e "\n${GREEN}========================================${NC}"
echo -e "${GREEN}Deployment Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo -e "\nTo access the application:"
echo -e "  ${YELLOW}kubectl port-forward -n bensons-shop service/bensons-shop 8080:80${NC}"
echo -e "\nThen visit: ${GREEN}http://localhost:8080${NC}"
echo -e "\nTo view logs:"
echo -e "  ${YELLOW}kubectl logs -n bensons-shop -l app=bensons-shop -f${NC}"
echo -e "\nTo check pod status:"
echo -e "  ${YELLOW}kubectl get pods -n bensons-shop${NC}"
echo -e "\nTo delete deployment:"
echo -e "  ${YELLOW}kubectl delete namespace bensons-shop${NC}\n"

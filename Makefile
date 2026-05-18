# ===========================================
# Cloud-Native URL Shortener - Makefile
# ===========================================

.PHONY: help build run test clean docker-build docker-up docker-down k8s-deploy k8s-destroy terraform-init terraform-apply terraform-destroy load-test

# Default target
help:
	@echo "Cloud-Native URL Shortener - Available Commands:"
	@echo ""
	@echo "  Local Development:"
	@echo "    build           - Build the Rust API"
	@echo "    run             - Run the API locally"
	@echo "    test            - Run tests"
	@echo "    clean           - Clean build artifacts"
	@echo ""
	@echo "  Docker:"
	@echo "    docker-build    - Build Docker images"
	@echo "    docker-up       - Start with Docker Compose"
	@echo "    docker-down     - Stop Docker Compose"
	@echo "    docker-logs     - View Docker logs"
	@echo ""
	@echo "  Kubernetes:"
	@echo "    k8s-deploy      - Deploy to Kubernetes"
	@echo "    k8s-destroy     - Remove from Kubernetes"
	@echo "    k8s-logs        - View Kubernetes logs"
	@echo "    k8s-port-forward - Port forward to local"
	@echo ""
	@echo "  Terraform:"
	@echo "    terraform-init  - Initialize Terraform"
	@echo "    terraform-plan   - Plan Terraform changes"
	@echo "    terraform-apply  - Apply Terraform"
	@echo "    terraform-destroy - Destroy infrastructure"
	@echo ""
	@echo "  Monitoring:"
	@echo "    monitoring-up   - Start monitoring stack"
	@echo "    monitoring-down  - Stop monitoring stack"
	@echo ""
	@echo "  Testing:"
	@echo "    load-test       - Run load tests with k6"

# ===========================================
# Local Development
# ===========================================
build:
	cd api && cargo build --release

run:
	cd api && cargo run --release

test:
	cd api && cargo test

clean:
	cd api && cargo clean
	rm -rf target/

# ===========================================
# Docker
# ===========================================
docker-build:
	docker compose build

docker-up:
	docker compose up -d

docker-down:
	docker compose down

docker-logs:
	docker compose logs -f

docker-ps:
	docker compose ps

# ===========================================
# Kubernetes
# ===========================================
k8s-namespace:
	kubectl apply -f k8s/namespace.yaml

k8s-deploy: k8s-namespace
	kubectl apply -f k8s/configmap.yaml
	kubectl apply -f k8s/secret.yaml
	kubectl apply -f k8s/deployment.yaml
	kubectl apply -f k8s/service.yaml
	kubectl apply -f k8s/ingress.yaml
	kubectl apply -f k8s/hpa.yaml

k8s-destroy:
	kubectl delete -f k8s/hpa.yaml --ignore-not-found
	kubectl delete -f k8s/ingress.yaml --ignore-not-found
	kubectl delete -f k8s/service.yaml --ignore-not-found
	kubectl delete -f k8s/deployment.yaml --ignore-not-found
	kubectl delete -f k8s/secret.yaml --ignore-not-found
	kubectl delete -f k8s/configmap.yaml --ignore-not-found
	kubectl delete -f k8s/namespace.yaml --ignore-not-found

k8s-logs:
	kubectl logs -f -l app=url-shortener -n url-shortener

k8s-port-forward:
	kubectl port-forward svc/api-service 8080:8080 -n url-shortener

k8s-status:
	kubectl get all -n url-shortener

# ===========================================
# Terraform
# ===========================================
terraform-init:
	cd terraform && terraform init

terraform-plan:
	cd terraform && terraform plan -var-file=terraform.tfvars

terraform-apply:
	cd terraform && terraform apply -var-file=terraform.tfvars

terraform-destroy:
	cd terraform && terraform destroy -var-file=terraform.tfvars

# ===========================================
# Monitoring
# ===========================================
monitoring-up:
	docker compose up -d prometheus grafana loki

monitoring-down:
	docker compose down prometheus grafana loki

# ===========================================
# Load Testing
# ===========================================
load-test:
	k6 run scripts/load-test.js

load-test-cloud:
	k6 cloud scripts/load-test.js

# ===========================================
# Security
# ===========================================
security-scan:
	trivy fs .
	trivy image url-shortener-api:latest

# ===========================================
# Development Setup
# ===========================================
setup: docker-build docker-up
	@echo "Development environment ready!"
	@echo "API: http://localhost:8080"
	@echo "Prometheus: http://localhost:9090"
	@echo "Grafana: http://localhost:3000"

dev-setup:
	cp .env.example .env
	rustup install stable
	rustup default stable
	cd api && cargo build
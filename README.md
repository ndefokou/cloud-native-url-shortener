# Cloud-Native URL Shortener

A production-style cloud-native URL shortener demonstrating containerization, infrastructure as code, CI/CD automation, Kubernetes orchestration, observability, DevSecOps, and cloud deployment.

## 🏗️ Architecture

```text
                   ┌─────────────────┐
                   │     Users       │
                   └────────┬────────┘
                            │
                            ▼
                  ┌──────────────────┐
                  │  NGINX Ingress   │
                  │ TLS + Routing    │
                  └────────┬─────────┘
                           │
                 ┌─────────┴─────────┐
                 ▼                   ▼
        ┌────────────────┐   ┌────────────────┐
        │ URL API Pod 1  │   │ URL API Pod 2  │
        └────────┬───────┘   └────────┬───────┘
                 │                    │
                 └─────────┬──────────┘
                           ▼
                 ┌──────────────────┐
                 │ Redis/Postgres   │
                 └──────────────────┘
```

## 🛠️ Technology Stack

| Layer              | Tool               |
| ------------------ | ------------------ |
| Language           | Rust               |
| Framework          | Axum               |
| Database           | Redis              |
| Reverse Proxy      | Nginx              |
| Containers         | Docker             |
| Container Registry | Docker Hub (Sprint 1), GHCR (later) |
| CI/CD              | GitHub Actions     |
| IaC                | Terraform          |
| Orchestration      | Kubernetes         |
| Monitoring         | Prometheus         |
| Visualization      | Grafana            |
| Logging            | Loki               |
| GitOps             | ArgoCD             |
| Security           | Trivy              |
| Secrets            | Kubernetes Secrets |
| Local K8s          | Minikube/k3d       |

## 📁 Project Structure

```text
cloud-native-url-shortener/
├── api/                    # Rust API service
│   ├── src/
│   ├── Cargo.toml
│   └── Dockerfile
├── nginx/                  # Nginx reverse proxy
│   ├── nginx.conf
│   └── Dockerfile
├── k8s/                    # Kubernetes manifests
│   ├── namespace.yaml
│   ├── deployment.yaml
│   ├── service.yaml
│   ├── ingress.yaml
│   ├── hpa.yaml
│   ├── configmap.yaml
│   └── secret.yaml
├── terraform/              # Infrastructure as Code
│   ├── main.tf
│   ├── variables.tf
│   ├── outputs.tf
│   └── modules/
├── monitoring/             # Observability stack
│   ├── prometheus/
│   ├── grafana/
│   └── loki/
├── .github/                # CI/CD workflows
│   └── workflows/
│       ├── ci.yml              # Lint & test (every push)
│       ├── docker-publish.yml  # Push image to Docker Hub (main)
│       └── ci-cd-full.yml      # K8s deploy + GHCR (later sprints)
├── docs/
│   └── SPRINT-1.md         # Sprint 1 task guide
├── scripts/                # Utility scripts
├── docker-compose.yml      # Local development
└── README.md
```

## 🚀 API Endpoints

| Method | Route    | Purpose          |
| ------ | -------- | ---------------- |
| POST   | /shorten | Create short URL |
| GET    | /:code   | Redirect         |
| GET    | /health  | Health check     |

## 📋 Prerequisites

- Docker & Docker Compose
- Kubernetes (minikube, k3d, or cloud cluster)
- Rust (for local development)
- Terraform (for infrastructure deployment)
- kubectl

## 🔧 Local Development

### 1. Clone the repository

```bash
git clone <repository-url>
cd cloud-native-url-shortener
```

### 2. Run with Docker Compose

```bash
docker compose up
```

### 3. Test the API

```bash
# Create a short URL
curl -X POST http://localhost:8080/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'

# Use the short URL (redirects to original)
curl -L http://localhost:8080/<code>

# Health check
curl http://localhost:8080/health
```

## ☸️ Kubernetes Deployment

```bash
# Create namespace
kubectl apply -f k8s/namespace.yaml

# Deploy application
kubectl apply -f k8s/

# Check status
kubectl get pods -n url-shortener
```

## 📊 Monitoring

Access the monitoring dashboards:
- Prometheus: http://localhost:9090
- Grafana: http://localhost:3000

## 🔐 Security Features

- Non-root containers
- Image scanning with Trivy
- HTTPS with cert-manager
- RBAC configurations
- Resource limits
- Rate limiting

## 📈 CI/CD Pipeline (Sprint 1)

See [docs/SPRINT-1.md](docs/SPRINT-1.md) for the full task breakdown.

| Workflow | Trigger | What it does |
| -------- | ------- | ------------ |
| `ci.yml` | Every push / PR | `cargo fmt`, clippy, unit tests |
| `docker-publish.yml` | Push to `main` | Build image, tag with Git SHA, push to Docker Hub |

**Required GitHub secrets:** `DOCKERHUB_USERNAME`, `DOCKERHUB_TOKEN`

Later sprints use `ci-cd-full.yml` (Trivy, GHCR, Kubernetes deploy).

## 📝 License

MIT License
# Sprint 1 — Git & Docker Hub CI/CD

First presentation sprint: version control, containers, and automated image publishing.

## Branching strategy (S1-01)

| Branch    | Purpose                                      |
| --------- | -------------------------------------------- |
| `main`    | Production-ready code; protected, merge only   |
| `develop` | Integration branch for feature work          |
| `feature/*` | Short-lived branches per task or member    |

**Workflow:** branch from `develop` → open PR → CI runs lint & test → review → merge to `develop`. Release/demo merges go to `main` and trigger the Docker Hub publish workflow.

## Task checklist

| ID    | Task | Artifact in this repo |
| ----- | ---- | --------------------- |
| S1-01 | Initialise repo & branching strategy | `docs/SPRINT-1.md`, default branches |
| S1-02 | Multi-stage Dockerfile | `api/Dockerfile` |
| S1-03 | docker-compose for local dev | `docker-compose.yml` |
| S1-04 | Docker Hub repo + GitHub secrets | `DOCKERHUB_USERNAME`, `DOCKERHUB_TOKEN` in repo settings |
| S1-05 | CI: lint & test on every push | `.github/workflows/ci.yml` |
| S1-06 | CD: build, tag with Git SHA, push to Docker Hub on merge to `main` | `.github/workflows/docker-publish.yml` |
| S1-07 | Unit tests for API endpoints | `api/tests/api_tests.rs` |

## S1-04 — GitHub secrets

In **Settings → Secrets and variables → Actions**, add:

- `DOCKERHUB_USERNAME` — Docker Hub username (e.g. `mbunwevicki100`), not your GitHub name
- `DOCKERHUB_TOKEN` — access token with **Read, Write, Delete** (Read-only cannot push)

Create a repository on Docker Hub named `url-shortener` (or set `IMAGE_NAME` in the publish workflow).

## S1-03 — Local development

```bash
cp .env.example .env
docker compose up --build
```

API: http://localhost:8080 (or via nginx at http://localhost:80)
Prometheus: http://localhost:9090
Grafana: http://localhost:3000

## S1-06 — Test on a pull request

Open a PR targeting `main` or `develop`. The **CD — Publish to Docker Hub** workflow will:

- Build the Docker image (validates Dockerfile + secrets for username)
- **Not** push to Docker Hub (avoids polluting `latest` from unmerged code)

After merge to `main`, the same workflow pushes to Docker Hub.

## S1-06 — Image tags on Docker Hub

After a merge to `main`, the publish workflow pushes:

- `docker.io/<username>/url-shortener:<git-sha>`
- `docker.io/<username>/url-shortener:latest`

Pull example:

```bash
docker pull <username>/url-shortener:latest
```

## Later sprints

Kubernetes deploy, GHCR, Trivy image scan, and staging/production deploy jobs live in `.github/workflows/ci-cd-full.yml` for Sprint 2+.

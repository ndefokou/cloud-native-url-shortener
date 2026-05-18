# ===========================================
# Monitoring Module - Prometheus & Grafana
# ===========================================

# Install Prometheus Operator using Helm
resource "helm_release" "prometheus_operator" {
  name       = "prometheus-operator"
  namespace  = "monitoring"
  repository = "https://prometheus-community.github.io/helm-charts"
  chart      = "kube-prometheus-stack"
  version    = "52.0.0"

  create_namespace = true

  set {
    name  = "prometheus.prometheusSpec.retention"
    value = "7d"
  }

  set {
    name  = "prometheus.prometheusSpec.storageSpec.volumeClaimTemplate.spec.resources.requests.storage"
    value = "10Gi"
  }

  set {
    name  = "grafana.adminPassword"
    value = var.grafana_password
  }

  set {
    name  = "alertmanager.enabled"
    value = "true"
  }
}

# Install Loki for log aggregation
resource "helm_release" "loki" {
  name       = "loki"
  namespace  = "monitoring"
  repository = "https://grafana.github.io/helm-charts"
  chart      = "loki-stack"
  version    = "2.9.0"

  set {
    name  = "promtail.enabled"
    value = "true"
  }

  set {
    name  = "loki.persistence.enabled"
    value = "true"
  }

  set {
    name  = "loki.persistence.size"
    value = "5Gi"
  }

  depends_on = [helm_release.prometheus_operator]
}

# ===========================================
# Outputs
# ===========================================
output "grafana_url" {
  description = "URL for Grafana dashboard"
  value       = "http://grafana.monitoring.svc.cluster.local"
}

output "prometheus_url" {
  description = "URL for Prometheus"
  value       = "http://prometheus-operated.monitoring.svc.cluster.local:9090"
}
# ===========================================
# Kubernetes Module - DOKS Cluster
# ===========================================

resource "digitalocean_kubernetes_cluster" "main" {
  name    = var.cluster_name
  region  = var.region
  version = "1.28.2"

  # Use auto-scaling node pool
  node_pool {
    name       = "default"
    size       = var.node_size
    auto_scale = true
    min_nodes  = var.node_count
    max_nodes  = var.node_count * 2
  }

  vpc_uuid = var.vpc_id

  maintenance_policy {
    start_time  = "04:00"
    day         = "sunday"
  }

  auto_upgrade = true

  tags = [
    "Environment:${var.environment}",
    "Project:url-shortener"
  ]
}

# ===========================================
# Outputs
# ===========================================
output "cluster_id" {
  description = "ID of the Kubernetes cluster"
  value       = digitalocean_kubernetes_cluster.main.id
}

output "cluster_name" {
  description = "Name of the Kubernetes cluster"
  value       = digitalocean_kubernetes_cluster.main.name
}

output "cluster_endpoint" {
  description = "Endpoint for the Kubernetes API"
  value       = digitalocean_kubernetes_cluster.main.endpoint
}

output "cluster_token" {
  description = "Token for the Kubernetes cluster"
  value       = digitalocean_kubernetes_cluster.main.kube_config[0].token
  sensitive   = true
}

output "cluster_ca_certificate" {
  description = "CA certificate for the Kubernetes cluster"
  value       = digitalocean_kubernetes_cluster.main.kube_config[0].cluster_ca_certificate
  sensitive   = true
}

output "node_ids" {
  description = "IDs of the worker nodes"
  value       = digitalocean_kubernetes_cluster.main.node_pool[0].nodes[*].id
}
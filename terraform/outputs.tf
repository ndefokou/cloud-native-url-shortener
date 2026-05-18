# ===========================================
# Infrastructure Outputs
# ===========================================

# Kubernetes Cluster
output "cluster_id" {
  description = "ID of the Kubernetes cluster"
  value       = module.kubernetes.cluster_id
}

output "cluster_name" {
  description = "Name of the Kubernetes cluster"
  value       = module.kubernetes.cluster_name
}

output "cluster_endpoint" {
  description = "Endpoint for the Kubernetes API"
  value       = module.kubernetes.cluster_endpoint
}

output "cluster_token" {
  description = "Token for the Kubernetes cluster"
  value       = module.kubernetes.cluster_token
  sensitive   = true
}

output "cluster_ca_certificate" {
  description = "CA certificate for the Kubernetes cluster"
  value       = module.kubernetes.cluster_ca_certificate
  sensitive   = true
}

# Networking
output "vpc_id" {
  description = "ID of the VPC"
  value       = module.networking.vpc_id
}

output "vpc_cidr" {
  description = "CIDR block of the VPC"
  value       = module.networking.vpc_cidr
}

# Redis
output "redis_host" {
  description = "Redis connection host"
  value       = module.redis.redis_host
}

output "redis_port" {
  description = "Redis connection port"
  value       = module.redis.redis_port
}

# Domain
output "domain_name" {
  description = "Domain name for the application"
  value       = var.domain_name
}

output "short_url_domain" {
  description = "Short URL domain"
  value       = "short.${var.domain_name}"
}

# Monitoring
output "grafana_url" {
  description = "URL for Grafana dashboard"
  value       = module.monitoring.grafana_url
}

output "prometheus_url" {
  description = "URL for Prometheus"
  value       = module.monitoring.prometheus_url
}

# Connection Information
output "kubeconfig_command" {
  description = "Command to get kubeconfig"
  value       = "doctl kubernetes cluster kubeconfig save ${module.kubernetes.cluster_id}"
}

output "application_url" {
  description = "URL to access the application"
  value       = "https://short.${var.domain_name}"
}
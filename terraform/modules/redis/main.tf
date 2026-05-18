# ===========================================
# Redis Module - Managed Redis Instance
# ===========================================

resource "digitalocean_database_cluster" "redis" {
  name       = var.redis_name
  engine     = "redis"
  version    = "7"
  size       = var.redis_size
  region     = var.region
  node_count = 1

  private_network_uuid = var.vpc_id

  tags = [
    "Environment:${var.environment}",
    "Project:url-shortener"
  ]
}

# ===========================================
# Outputs
# ===========================================
output "redis_host" {
  description = "Redis connection host"
  value       = digitalocean_database_cluster.redis.host
}

output "redis_port" {
  description = "Redis connection port"
  value       = digitalocean_database_cluster.redis.port
}

output "redis_uri" {
  description = "Redis connection URI"
  value       = digitalocean_database_cluster.redis.uri
  sensitive   = true
}

output "redis_id" {
  description = "Redis cluster ID"
  value       = digitalocean_database_cluster.redis.id
}
# ===========================================
# Networking Module - VPC Configuration
# ===========================================

resource "digitalocean_vpc" "main" {
  name     = var.vpc_name
  region   = var.region
  ip_range = var.vpc_cidr

  tags = [
    "Environment:${var.environment}",
    "Project:url-shortener"
  ]
}

# ===========================================
# Outputs
# ===========================================
output "vpc_id" {
  description = "ID of the VPC"
  value       = digitalocean_vpc.main.id
}

output "vpc_cidr" {
  description = "CIDR block of the VPC"
  value       = digitalocean_vpc.main.ip_range
}

output "vpc_name" {
  description = "Name of the VPC"
  value       = digitalocean_vpc.main.name
}
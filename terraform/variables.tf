# ===========================================
# Project Variables
# ===========================================

variable "project_name" {
  description = "Name of the project"
  type        = string
  default     = "url-shortener"
}

variable "environment" {
  description = "Environment (development, staging, production)"
  type        = string
  default     = "production"
}

# ===========================================
# DigitalOcean Variables
# ===========================================

variable "do_token" {
  description = "DigitalOcean API token"
  type        = string
  sensitive   = true
}

variable "region" {
  description = "DigitalOcean region"
  type        = string
  default     = "nyc1"
}

# ===========================================
# Networking Variables
# ===========================================

variable "vpc_cidr" {
  description = "CIDR block for VPC"
  type        = string
  default     = "10.0.0.0/16"
}

# ===========================================
# Kubernetes Variables
# ===========================================

variable "node_count" {
  description = "Number of worker nodes"
  type        = number
  default     = 3
}

variable "node_size" {
  description = "Size of worker nodes"
  type        = string
  default     = "s-2vcpu-4gb"
}

# ===========================================
# Redis Variables
# ===========================================

variable "redis_size" {
  description = "Size of Redis instance"
  type        = string
  default     = "db-s-1vcpu-1gb"
}

# ===========================================
# Domain Variables
# ===========================================

variable "domain_name" {
  description = "Domain name for the application"
  type        = string
  default     = "example.com"
}

# ===========================================
# Monitoring Variables
# ===========================================

variable "enable_monitoring" {
  description = "Enable monitoring stack"
  type        = bool
  default     = true
}

variable "grafana_admin_password" {
  description = "Grafana admin password"
  type        = string
  sensitive   = true
  default     = "admin"
}

# ===========================================
# Backup Variables
# ===========================================

variable "enable_backups" {
  description = "Enable automated backups"
  type        = bool
  default     = true
}
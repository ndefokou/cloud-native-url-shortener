# ===========================================
# Monitoring Module Variables
# ===========================================

variable "cluster_id" {
  description = "ID of the Kubernetes cluster"
  type        = string
}

variable "environment" {
  description = "Environment name"
  type        = string
}

variable "grafana_password" {
  description = "Grafana admin password"
  type        = string
  sensitive   = true
  default     = "admin"
}
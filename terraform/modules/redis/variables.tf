# ===========================================
# Redis Module Variables
# ===========================================

variable "redis_name" {
  description = "Name of the Redis instance"
  type        = string
}

variable "region" {
  description = "DigitalOcean region"
  type        = string
}

variable "redis_size" {
  description = "Size of Redis instance"
  type        = string
}

variable "vpc_id" {
  description = "ID of the VPC"
  type        = string
}

variable "environment" {
  description = "Environment name"
  type        = string
}
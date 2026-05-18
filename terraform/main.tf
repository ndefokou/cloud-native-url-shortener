# ===========================================
# Terraform Configuration
# ===========================================
terraform {
  required_version = ">= 1.5.0"

  required_providers {
    digitalocean = {
      source  = "digitalocean/digitalocean"
      version = "~> 2.30"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "~> 2.23"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "~> 2.11"
    }
  }

  # Backend configuration for remote state
  # Uncomment and configure for production
  # backend "s3" {
  #   bucket         = "your-terraform-state-bucket"
  #   key            = "url-shortener/terraform.tfstate"
  #   region         = "us-east-1"
  #   encrypt        = true
  #   dynamodb_table = "terraform-locks"
  # }
}

# ===========================================
# Providers
# ===========================================
provider "digitalocean" {
  token = var.do_token
}

provider "kubernetes" {
  host                   = module.kubernetes.cluster_endpoint
  token                  = module.kubernetes.cluster_token
  cluster_ca_certificate = module.kubernetes.cluster_ca_certificate
}

provider "helm" {
  kubernetes {
    host                   = module.kubernetes.cluster_endpoint
    token                  = module.kubernetes.cluster_token
    cluster_ca_certificate = module.kubernetes.cluster_ca_certificate
  }
}

# ===========================================
# Modules
# ===========================================
module "networking" {
  source = "./modules/networking"

  region          = var.region
  vpc_name        = "${var.project_name}-vpc"
  vpc_cidr        = var.vpc_cidr
  environment     = var.environment
}

module "kubernetes" {
  source = "./modules/kubernetes"

  cluster_name    = "${var.project_name}-cluster"
  region          = var.region
  node_count      = var.node_count
  node_size       = var.node_size
  vpc_id          = module.networking.vpc_id
  environment     = var.environment
}

module "redis" {
  source = "./modules/redis"

  redis_name      = "${var.project_name}-redis"
  region          = var.region
  redis_size      = var.redis_size
  vpc_id          = module.networking.vpc_id
  environment     = var.environment
}

module "monitoring" {
  source = "./modules/monitoring"

  cluster_id      = module.kubernetes.cluster_id
  environment     = var.environment

  depends_on = [module.kubernetes]
}

# ===========================================
# DNS Configuration
# ===========================================
resource "digitalocean_domain" "main" {
  name       = var.domain_name
  project_id = digitalocean_project.main.id
}

resource "digitalocean_domain" "short" {
  name       = "short.${var.domain_name}"
  project_id = digitalocean_project.main.id
}

# ===========================================
# Project
# ===========================================
resource "digitalocean_project" "main" {
  name        = var.project_name
  description = "Cloud-native URL shortener project"
  environment = var.environment
  purpose     = "Web Application"
}

# ===========================================
# Firewall Rules
# ===========================================
resource "digitalocean_firewall" "kubernetes" {
  name = "${var.project_name}-k8s-firewall"

  droplet_ids = module.kubernetes.node_ids

  # Inbound rules
  inbound_rule {
    protocol         = "tcp"
    port_range       = "22"
    source_addresses = ["0.0.0.0/0"]
  }

  inbound_rule {
    protocol         = "tcp"
    port_range       = "80"
    source_addresses = ["0.0.0.0/0"]
  }

  inbound_rule {
    protocol         = "tcp"
    port_range       = "443"
    source_addresses = ["0.0.0.0/0"]
  }

  # Kubernetes API
  inbound_rule {
    protocol         = "tcp"
    port_range       = "6443"
    source_addresses = ["0.0.0.0/0"]
  }

  # Outbound rules
  outbound_rule {
    protocol              = "tcp"
    port_range            = "1-65535"
    destination_addresses = ["0.0.0.0/0"]
  }

  outbound_rule {
    protocol              = "udp"
    port_range            = "1-65535"
    destination_addresses = ["0.0.0.0/0"]
  }

  outbound_rule {
    protocol              = "icmp"
    destination_addresses = ["0.0.0.0/0"]
  }
}
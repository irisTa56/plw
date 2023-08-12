provider "google" {
  credentials = var.GCP_SERVICE_ACCOUNT_CREDENTIALS

  project = var.GCP_PROJECT_ID
  region  = var.region
  zone    = var.zone
}

provider "google-beta" {
  credentials = var.GCP_SERVICE_ACCOUNT_CREDENTIALS

  project = var.GCP_PROJECT_ID
  region  = var.region
  zone    = var.zone
}

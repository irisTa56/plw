module "service_accounts" {
  # "Service Account Admin" role is required (for typical use-cases).

  source  = "terraform-google-modules/service-accounts/google"
  version = "~> 4.0"

  project_id = var.GCP_PROJECT_ID
  names      = ["test-sa"]
}

module "cloud_run" {
  # "Cloud Run Admin" role is required to set IAM policies to the resource.
  # A service account to be attached to the resource is created by the `service-accounts` module.

  source  = "GoogleCloudPlatform/cloud-run/google"
  version = "~> 0.9.0"

  service_name = "test-hello"
  project_id   = var.GCP_PROJECT_ID
  location     = var.region
  image        = "gcr.io/cloudrun/hello"

  container_concurrency = 1
  members               = ["allUsers"]
  service_account_email = module.service_accounts.email
}

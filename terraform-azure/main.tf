resource "azurerm_resource_group" "rg" {
  name     = "test-rg"
  location = var.location
}

module "container_apps" {
  source  = "Azure/container-apps/azure"
  version = "~> 0.1.0"

  resource_group_name            = azurerm_resource_group.rg.name
  location                       = var.location
  container_app_environment_name = "test-cae"
  log_analytics_workspace_name   = "test-law"

  container_apps = {
    app = {
      name          = "test-app"
      revision_mode = "Single"
      max_replicas  = 1

      template = {
        containers = [
          {
            name   = "test-helloworld"
            image  = "mcr.microsoft.com/azuredocs/containerapps-helloworld"
            cpu    = 0.25
            memory = "0.5Gi"
          }
        ]
      }

      ingress = {
        target_port      = 80
        external_enabled = true
        traffic_weight = {
          latest_revision = true
          percentage      = 100
        }
      }
    }
  }
}

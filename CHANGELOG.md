# Changelog

All notable changes to this project are documented in this file.

This project follows [Keep a Changelog](https://keepachangelog.com/) conventions and adheres to [Semantic Versioning](https://semver.org/).

## [v0.1.0] - 2023-09-20

### Initial Release

Basic Python client for Argo Workflows with essential API operations.

#### Added Features

##### **Workflow Service**
- `create_workflow` - Submit workflows to Kubernetes
- `get_workflow` - Retrieve workflow details
- `list_workflows` - List workflows in namespace
- `delete_workflow` - Delete workflows

##### **Workflow Template Service**
- `create_workflow_template` - Create workflow templates
- `get_workflow_template` - Get template details
- `list_workflow_templates` - List all templates
- `delete_workflow_template` - Delete templates

##### **Archived Workflow Service**
- `list_archived_workflows` - List archived workflows
- `get_archived_workflow` - Retrieve archived workflow details

For additional details, please refer to the [release notes](https://github.com/gauravgahlot/argoflows/releases/tag/v0.1.0).

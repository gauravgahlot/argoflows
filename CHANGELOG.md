# Changelog

All notable changes to this project are documented in this file.

This project follows [Keep a Changelog](https://keepachangelog.com/) conventions and adheres to [Semantic Versioning](https://semver.org/).

## [v0.1.0] - 2025-01-18

Argoflows provides a Rust client library for interacting with Argo Workflows.

## [Unreleased]

#### Added Features

- Initial project setup with core functionality
- REST API endpoints for Argo Workflows operations:
  - `/api/v1/workflows` - List workflows
  - `/api/v1/workflow/{namespace}/{name}` - Get workflow details
  - `/api/v1/workflow/{namespace}/{name}/logs` - Get workflow logs
  - `/api/v1/workflow/{namespace}/{name}/terminate` - Terminate workflow
  - `/api/v1/workflow/{namespace}/{name}/retry` - Retry workflow
  - `/api/v1/workflow/{namespace}/{name}/suspend` - Suspend workflow
  - `/api/v1/workflow/{namespace}/{name}/resume` - Resume workflow
  - `/api/v1/workflow/{namespace}/{name}/resubmit` - Resubmit workflow
- Support for multiple Kubernetes clusters
- Workflow management capabilities including:
  - Listing workflows
  - Getting workflow details
  - Retrieving workflow logs
  - Workflow operations (terminate, retry, suspend, resume, resubmit)
- Basic authentication support
- Kubernetes configuration integration
- Error handling and logging

### Documentation
- Added README with API documentation
- Added installation and setup instructions
- Added configuration guide for Kubernetes integration

[Unreleased]: https://github.com/gauravgahlot/argoflows/commits/main
For additional details, please refer to the [release notes](https://github.com/gauravgahlot/argoflows/releases/tag/v0.1.0).

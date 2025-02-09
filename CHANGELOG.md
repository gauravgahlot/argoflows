# Changelog

All notable changes to this project are documented in this file.

This project follows [Keep a Changelog](https://keepachangelog.com/) conventions and adheres to [Semantic Versioning](https://semver.org/).

## [v0.1.0] - 2025-01-18

### Initial Release

This release introduces the core functionality of `argoflows`, providing comprehensive support for workflow orchestration through various services and endpoints.

#### **Added Features**

##### **Info Service**
- `get_info` – Retrieves general service information.
- `collect_event` – Collects events for processing.

##### **Workflow Service**
- `create_workflow` – Submits a new workflow for execution.
- `get_workflow` – Fetches details of a specific workflow.
- `list_workflows` – Retrieves a list of all workflows.
- `terminate_workflow` – Terminates an active workflow.
- `retry_workflow` – Retries a failed workflow.
- `suspend_workflow` – Suspends a running workflow.
- `resume_workflow` – Resumes a suspended workflow.
- `delete_workflow` – Deletes a specified workflow.

##### **Workflow Template Service**
- `create_workflow_template` – Creates a new workflow template.
- `get_workflow_template` – Retrieves details of a specific workflow template.
- `list_workflow_templates` – Lists all workflow templates.
- `delete_workflow_template` – Deletes a specified workflow template.
- `lint_workflow_template` – Validates a workflow template.
- `update_workflow_template` – Updates an existing workflow template.

##### **Archived Workflow Service**
- `delete_archived_workflow` – Removes an archived workflow.
- `get_archived_workflow` – Retrieves details of an archived workflow.
- `list_archived_workflow_label_keys` – Lists label keys associated with archived workflows.
- `list_archived_workflow_label_values` – Lists label values associated with archived workflows.
- `list_archived_workflows` – Retrieves a list of all archived workflows.
- `resubmit_archived_workflow` – Resubmits an archived workflow.
- `retry_archived_workflow` – Retries an archived workflow.

##### **Cluster Workflow Template Service**
- `create_cluster_workflow_template` – Creates a new cluster-wide workflow template.
- `delete_cluster_workflow_template` – Deletes a specified cluster workflow template.
- `get_cluster_workflow_template` – Retrieves details of a cluster workflow template.
- `lint_cluster_workflow_template` – Validates a cluster workflow template.
- `list_cluster_workflow_templates` – Lists all cluster workflow templates.
- `update_cluster_workflow_template` – Updates an existing cluster workflow template.

##### **Cron Workflow Service**
- `create_cron_workflow` – Creates a new cron workflow.
- `delete_cron_workflow` – Deletes a specified cron workflow.
- `get_cron_workflow` – Retrieves details of a cron workflow.
- `lint_cron_workflow` – Validates a cron workflow.
- `list_cron_workflows` – Lists all cron workflows.
- `resume_cron_workflow` – Resumes a suspended cron workflow.
- `suspend_cron_workflow` – Suspends an active cron workflow.
- `update_cron_workflow` – Updates an existing cron workflow.

##### **Event Source Service**
- `create_event_source` – Creates a new event source.
- `delete_event_source` – Deletes a specified event source.
- `event_sources_logs` – Retrieves logs related to event sources.
- `get_event_source` – Fetches details of an event source.
- `list_event_sources` – Lists all event sources.
- `update_event_source` – Updates an existing event source.
- `watch_event_sources` – Monitors changes in event sources.

##### **Event Service**
- `list_workflow_event_bindings` – Lists all workflow event bindings.
- `receive_event` – Receives an event for processing.

For additional details, please refer to the [release notes](https://github.com/gauravgahlot/argoflows/releases/tag/v0.1.0).

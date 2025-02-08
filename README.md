<div align="center">
  <h1><code>argoflows</code></h1>

  <p>
    Argoflows lets you seamlessly integrate and manage 
    <a href="https://argo-workflows.readthedocs.io/en/latest/">
      <strong>Argo</strong> Work<strong>flows</strong>
    </a>directly from your Rust applications.
  </p>

  <p>
    <a href="https://github.com/gauravgahlot/argoflows/actions?query=workflow%3ACI"><img src="https://github.com/gauravgahlot/argoflows/actions/workflows/ci.yaml/badge.svg" alt="build status" /></a>
    <a href="https://docs.rs/argoflows"><img src="https://docs.rs/argoflows/badge.svg" alt="Documentation Status" /></a>
  </p>
</div>

## Supported API(s)

| API                              | Supported | Endpoint                              | Comment    |
| -------------------------------- | --------- | ------------------------------------- | ---------- |
| `ArchivedWorkflowService`        |           | `delete_archived_workflow`            |            |
|                                  |           | `get_archived_workflow`               |            |
|                                  |           | `list_archived_workflow_label_keys`   |            |
|                                  |           | `list_archived_workflow_label_values` |            |
|                                  |           | `list_archived_workflows`             |            |
|                                  |           | `resubmit_archived_workflow`          |            |
|                                  |           | `retry_archived_workflow`             |            |
|                                  |           |                                       |            |
| `ClusterWorkflowTemplateService` |           | `create_cluster_workflow_template`    |            |
|                                  |           | `delete_cluster_workflow_template`    |            |
|                                  |           | `get_cluster_workflow_template`       |            |
|                                  |           | `lint_cluster_workflow_template`      |            |
|                                  |           | `list_cluster_workflow_templates`     |            |
|                                  |           | `update_cluster_workflow_template`    |            |
|                                  |           |                                       |            |
| `CronWorkflowService`            |           | `create_cron_workflow`                |            |
|                                  |           | `delete_cron_workflow`                |            |
|                                  |           | `get_cron_workflow`                   |            |
|                                  |           | `lint_cron_workflow`                  |            |
|                                  |           | `list_cron_workflows`                 |            |
|                                  |           | `resume_cron_workflow`                |            |
|                                  |           | `suspend_cron_workflow`               |            |
|                                  |           | `update_cron_workflow`                |            |
|                                  |           |                                       |            |
| `EventSourceService`             |           | `create__event_source`                |            |
|                                  |           | `delete_event_source`                 |            |
|                                  |           | `event_sources_logs`                  |            |
|                                  |           | `get_event_source`                    |            |
|                                  |           | `list_event_sources`                  |            |
|                                  |           | `update_event_source`                 |            |
|                                  |           | `watch_event_sources`                 |            |
|                                  |           |                                       |            |
| `EventService`                   |           | `list_workflow_event_bindings`        |            |
|                                  |           | `receive_event`                       |            |
|                                  |           |                                       |            |
| `InfoService`                    | ✅        | `collect_event`                       |            |
|                                  | ✅        | `get_info`                            |            |
|                                  | ✅        | `get_user_info`                       |            |
|                                  | ✅        | `get_version`                         |            |
|                                  |           |                                       |            |
| `SensorService`                  |           | `create_sensor`                       |            |
|                                  |           | `delete_sensor`                       |            |
|                                  |           | `get_sensor`                          |            |
|                                  |           | `list_sensors`                        |            |
|                                  |           | `sensors_logs`                        |            |
|                                  |           | `update_sensor`                       |            |
|                                  |           | `watch_sensors`                       |            |
|                                  |           |                                       |            |
| `WorkflowService`                | ✅        | `create_workflow`                     |            |
|                                  | ✅        | `delete_workflow`                     |            |
|                                  | ✅        | `get_workflow`                        |            |
|                                  | ✅        | `lint_workflow`                       |            |
|                                  | ✅        | `list_workflows`                      |            |
|                                  | ❌        | `pod_logs`                            | DEPRECATED |
|                                  | ✅        | `resubmit_workflow`                   |            |
|                                  | ✅        | `resume_workflow`                     |            |
|                                  | ✅        | `retry_workflow`                      |            |
|                                  |           | `set_workflow`                        |            |
|                                  |           | `stop_workflow`                       |            |
|                                  | ✅        | `submit_workflow`                     |            |
|                                  |           | `suspend_workflow`                    |            |
|                                  |           | `terminate_workflow`                  |            |
|                                  |           | `watch_events`                        |            |
|                                  |           | `watch_workflows`                     |            |
|                                  |           | `workflow_logs`                       |            |
|                                  |           |                                       |            |
| `WorkflowTemplateService`        | ✅        | `create_workflow_template`            |            |
|                                  | ✅        | `delete_workflow_template`            |            |
|                                  | ✅        | `get_workflow_template`               |            |
|                                  | ✅        | `lint_workflow_template`              |            |
|                                  | ✅        | `list_workflow_templates`             |            |
|                                  | ✅        | `update_workflow_template`            |            |
|                                  |           |                                       |            |
| `ArtifactService`                |           | `get_artifact_file`                   |            |
|                                  |           | `get_input_artifact`                  |            |
|                                  |           | `get_input_artifact_by_uid`           |            |
|                                  |           | `get_ouput_artifact`                  |            |
|                                  |           | `get_output_artifact_by_uid`          |            |

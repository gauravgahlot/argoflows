use argoflows::api::workflow_template;
use argoflows::config::Config;
use argoflows::types::workflow_template::CreateRequest;

fn main() {
    let token = std::env::var("ARGO_TOKEN").expect("the ARGO_TOKEN env variable must be set");

    let cfg = Config::builder()
        .bearer_token(&token)
        .danger_accept_invalid_certs(true)
        .build();
    let cfg = cfg.expect("failed to create client config");

    let tmpl = r#"{
        "apiVersion": "argoproj.io/v1alpha1",
        "kind": "WorkflowTemplate",
        "metadata": {
            "name": "wftmpl-sample",
            "namespace": "argoflows"
        },
        "spec": {
            "arguments": {
                "parameters": [
                    {
                        "name": "message",
                        "value": "hello world"
                    }
                ]
            },
            "templates": [
                {
                    "name": "print-message",
                    "inputs": {
                        "parameters": [
                            {
                                "name": "message"
                            }
                        ]
                    },
                    "container": {
                        "image": "busybox",
                        "command": [
                            "echo"
                        ],
                        "args": [
                            "{{inputs.parameters.message}}"
                        ]
                    }
                }
            ]
        }
    }"#;

    let tmpl = serde_json::from_str(&tmpl).expect("failed to parse workflow template");
    let req = CreateRequest {
        namespace: Some(String::from("argoflows")),
        template: Some(Box::new(tmpl)),
        create_options: None,
    };

    match workflow_template::create_workflow_template(&cfg, "argoflows", req) {
        Ok(t) => println!(
            "Successfully created '{}' workflow template in '{}'",
            t.metadata.name.unwrap(),
            t.metadata.namespace.unwrap()
        ),
        Err(e) => eprintln!("failed to create workflow template: {:?}", e),
    }

    match workflow_template::list_workflow_templates(&cfg, "argoflows", None, None) {
        Ok(r) => println!("Found {} workflow templates", r.items.len()),
        Err(e) => eprintln!("failed to list workflow templates: {:?}", e),
    }

    match workflow_template::get_workflow_template(&cfg, "argoflows", "wftmpl-sample", None) {
        Ok(r) => {
            println!(
                "Found '{}' workflow template in '{}'",
                r.metadata.name.unwrap(),
                r.metadata.namespace.unwrap()
            )
        }
        Err(e) => eprintln!("failed to get workflow template: {:?}", e),
    }

    match workflow_template::delete_workflow_template(&cfg, "argoflows", "wftmpl-sample", None) {
        Ok(_) => println!("successfully deleted workflow template"),
        Err(e) => eprintln!("failed to delete workflow template: {:?}", e),
    }
}

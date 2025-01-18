use argoflows::api::workflow;
use argoflows::config::Config;
use argoflows::types::workflow::CreateRequest;

fn main() {
    let token = std::env::var("ARGO_TOKEN").expect("the ARGO_TOKEN env variable must be set");

    let cfg = Config::builder()
        .bearer_token(&token)
        .danger_accept_invalid_certs(true)
        .build();
    let cfg = cfg.expect("failed to create client config");

    let wf = r#"{
        "apiVersion": "argoproj.io/v1alpha1",
        "kind": "Workflow",
        "metadata": {
            "name": "wf-sample",
            "namespace": "argoflows"
        },
        "spec": {
            "entrypoint": "hello-world",
            "templates": [
                {
                    "name": "hello-world",
                    "container": {
                        "image": "busybox",
                        "command": [
                            "echo"
                        ],
                        "args": [
                            "hello world"
                        ]
                    }
                }
            ]
        }
    }"#;

    let wf = serde_json::from_str(&wf).expect("failed to parse workflow template");
    let req = CreateRequest {
        namespace: Some(String::from("argoflows")),
        workflow: Some(Box::new(wf)),
        create_options: None,
        ..Default::default()
    };

    match workflow::create_workflow(&cfg, "argoflows", req) {
        Ok(t) => println!(
            "Successfully created '{}' workflow in '{}'",
            t.metadata.name.unwrap(),
            t.metadata.namespace.unwrap()
        ),
        Err(e) => eprintln!("failed to create workflow: {:?}", e),
    }

    match workflow::list_workflows(&cfg, "argoflows", None, None, None) {
        Ok(r) => println!("Found {} workflows", r.items.len()),
        Err(e) => eprintln!("failed to list workflow: {:?}", e),
    }

    match workflow::get_workflow(&cfg, "argoflows", "wf-sample", None, None) {
        Ok(r) => {
            println!(
                "Found '{}' workflow  in '{}'",
                r.metadata.name.unwrap(),
                r.metadata.namespace.unwrap()
            )
        }
        Err(e) => eprintln!("failed to get workflow: {:?}", e),
    }

    match workflow::delete_workflow(&cfg, "argoflows", "wf-sample", None, false) {
        Ok(_) => println!("successfully deleted workflow"),
        Err(e) => eprintln!("failed to delete workflow: {:?}", e),
    }
}

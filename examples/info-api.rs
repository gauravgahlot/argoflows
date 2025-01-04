use argoflows::api::info;
use argoflows::config::Config;

fn main() {
    let token = std::env::var("ARGO_TOKEN").expect("the ARGO_TOKEN env variable must be set");

    let cfg = Config::new(token);
    match info::get_version(&cfg) {
        Ok(v) => println!("{:?}\n", v),
        Err(e) => eprintln!("failed to get version: {:?}", e),
    }
}

use shiplift::Docker;
use std::env;

#[tokio::main]
async fn main() {
    let docker = Docker::from_env().unwrap();
    let volumes = docker.volumes();

    let volume_name = env::args()
        .nth(1)
        .expect("You need to specify an volume name");

    if let Err(e) = volumes.get(&volume_name).delete().await {
        eprintln!("Error: {}", e)
    }
}

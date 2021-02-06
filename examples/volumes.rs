use shiplift::Docker;

#[tokio::main]
async fn main() {
    let docker = Docker::from_env().unwrap();
    let volumes = docker.volumes();

    match volumes.list().await {
        Ok(volumes) => {
            for v in volumes {
                println!("volume -> {:#?}", v)
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

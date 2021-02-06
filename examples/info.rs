use shiplift::Docker;

#[tokio::main]
async fn main() {
    let docker = Docker::from_env().unwrap();

    match docker.info().await {
        Ok(info) => println!("info {:?}", info),
        Err(e) => eprintln!("Error: {}", e),
    }
}

use shiplift::Docker;

#[tokio::main]
async fn main() {
    let docker = Docker::from_env().unwrap();
    match docker.version().await {
        Ok(ver) => println!("version -> {:#?}", ver),
        Err(e) => eprintln!("Error: {}", e),
    }
}

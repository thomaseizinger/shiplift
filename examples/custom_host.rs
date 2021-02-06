use shiplift::Docker;

#[tokio::main]
async fn main() {
    let docker = Docker::new("http://yourhost").unwrap();
    match docker.ping().await {
        Ok(pong) => println!("Ping: {}", pong),
        Err(e) => eprintln!("Error: {}", e),
    }
}

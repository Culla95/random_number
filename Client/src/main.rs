use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("localhost:8881")
        .await
        .expect("Couldn't connect to the server");

    loop {
        let mut random_number = rand::thread_rng().gen_range(100..1000000000).to_string();
        random_number.push_str("\n");
        let _escribir = stream.write(random_number.as_bytes());
        stream
            .write(random_number.as_bytes())
            .await
            .expect("Failed to write number");
        sleep(Duration::from_nanos(1));
    }
}

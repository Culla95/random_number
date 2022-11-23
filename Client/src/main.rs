use rand::Rng;
use std::io::{stdin, stdout, Write};
use std::sync::Arc;
use std::time::Duration;
use std::{io, thread::sleep};
use tokio::io::BufWriter;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("localhost:8881")
        .await
        .expect("Couldn't connect to the server");
    let mut timer = tokio::time::interval(Duration::from_nanos(2000));
    let mut buffer = BufWriter::new(stream);
    loop {
        timer.tick().await;
        let mut random_number = rand::thread_rng().gen_range(100..1000000000).to_string();
        random_number.push_str("\n");
        buffer
            .write(random_number.as_bytes())
            .await
            .expect("Failed to write number");
    }
}

/*
#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("localhost:8881")
        .await
        .expect("Couldn't connect to the server");
    let mut stream = Arc::new(tokio::sync::Mutex::new(stream));
    tokio::join!(send_numbers(stream.clone()), read_line(stream.clone()));


}


async fn send_numbers(mut stream: Arc<Mutex<TcpStream>>) {
    let mut timer = tokio::time::interval(Duration::from_nanos(2000));
    let mut buffer = BufWriter::new(stream.lock().await);
    tokio::spawn(async move{

        loop {
            timer.tick().await;
            let mut random_number = rand::thread_rng().gen_range(100..1000000000).to_string();
            random_number.push_str("\n");
            stream
                .lock().await.write(random_number.as_bytes())
                .await
                .expect("Failed to write number");

        }
    });

}

async fn read_line (mut stream: Arc<Mutex<TcpStream>>){
    loop{
        let mut user_input = String::new();
    let stdin = stdin();
    stdin.read_line(&mut user_input);
    if user_input=="finish\n"{
        stream.lock().await.shutdown().await;
        break;
    }

    }

}
*/

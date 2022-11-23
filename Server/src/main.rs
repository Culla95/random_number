use std::collections::HashSet;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::fs::OpenOptions;
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufWriter};
use tokio::time::{self};

#[tokio::main]
async fn main() {
    let total_unique_numbers = Arc::new(Mutex::new(HashSet::<u32>::new()));
    let connected_clients: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
    let tried_numbers: Arc<Mutex<HashSet<u32>>> = Arc::new(Mutex::new(HashSet::<u32>::new()));
    let new_unique_numbers: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let repeated_numbers: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

    tokio::join!(
        listen(connected_clients.clone(), tried_numbers.clone()),
        data_printing(
            connected_clients.clone(),
            total_unique_numbers.clone(),
            new_unique_numbers.clone(),
            repeated_numbers.clone()
        ),
        update_file(
            tried_numbers.clone(),
            total_unique_numbers.clone(),
            new_unique_numbers.clone(),
            repeated_numbers.clone()
        )
    );
}

async fn process_socket(
    connected_clients: Arc<Mutex<u8>>,
    tried_numbers: Arc<Mutex<HashSet<u32>>>,
    mut socket: tokio::net::TcpStream,
) {
    let (sread, _swrite) = socket.split();
    let mut reader = BufReader::new(sread);
    loop {
        let mut line_read = String::new();
        let n_bytes_read = reader.read_line(&mut line_read).await.unwrap();
        if n_bytes_read == 0 {
            *connected_clients.lock().unwrap() -= 1;
            socket.shutdown().await.unwrap();
            break;
        }
        let input = line_read.trim_end();
        match input.parse::<u32>() {
            Ok(rand_number) => {
                if rand_number < 1000000000 {
                    tried_numbers.lock().unwrap().insert(rand_number);
                } else {
                    *connected_clients.lock().unwrap() -= 1;
                    break;
                }
            }
            Err(_) => {
                *connected_clients.lock().unwrap() -= 1;
                break;
            }
        }
    }
}

async fn data_printing(
    connected_clients: Arc<Mutex<u8>>,
    total_unique_numbers: Arc<Mutex<HashSet<u32>>>,
    new_unique_numbers: Arc<Mutex<u32>>,
    repeated_numbers: Arc<Mutex<u32>>,
) {
    let mut timer = time::interval(Duration::from_secs(10));
    loop {
        timer.tick().await;
        let total_unique_numbers = total_unique_numbers.clone();
        let total = total_unique_numbers.lock().unwrap().len();
        let connected_clients = connected_clients.clone();
        let n_connected = connected_clients.lock().unwrap();
        {
            let mut new_unique_numbers = new_unique_numbers.lock().unwrap();
            let mut repeated_numbers = repeated_numbers.lock().unwrap();
            println!("Number of clients connected: {}", n_connected);
            println!("Number of new unique numbers: {}", new_unique_numbers);
            println!("Number of repeated numbers: {}", repeated_numbers);
            println!("Total number of new unique numbers: {}", total);
            println!("-----------------------------------------------");
            *new_unique_numbers = 0;
            *repeated_numbers = 0;
        }
    }
}

async fn listen(connected_clients: Arc<Mutex<u8>>, tried_numbers: Arc<Mutex<HashSet<u32>>>) {
    let listener = tokio::net::TcpListener::bind("localhost:8881")
        .await
        .unwrap();
    *connected_clients.lock().unwrap() = 0;
    tokio::spawn(async move {
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let connected_clients = connected_clients.clone();
            let tried_numbers = tried_numbers.clone();

            if *connected_clients.lock().unwrap() >= 5 {
                socket.shutdown().await;
            } else {
                tokio::spawn(async move {
                    *connected_clients.lock().unwrap() += 1;
                    process_socket(connected_clients, tried_numbers, socket).await;
                });
            }
        }
    });
}

async fn update_file(
    tried_numbers: Arc<Mutex<HashSet<u32>>>,
    total_unique_numbers: Arc<Mutex<HashSet<u32>>>,
    new_unique_numbers: Arc<Mutex<u32>>,
    repeated_numbers: Arc<Mutex<u32>>,
) {
    /*let Ok(output_file) = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("uniques.log").await else {
            panic!("Failed to open the file")
        };
    let total_unique_numbers = total_unique_numbers.clone();
    let new_unique_numbers = new_unique_numbers.clone();
    let repeated_numbers = repeated_numbers.clone();
    let mut timer = time::interval(Duration::from_micros(100));
    let mut writer = tokio::io::BufWriter::new(output_file);

    while let Some(number) = rx_numbers.recv().await {
        if !total_unique_numbers.lock().unwrap().contains(&number) {
            *new_unique_numbers.lock().unwrap() += 1;
            total_unique_numbers.lock().unwrap().insert(number);
            let mut number= number.to_string();
            number.push_str("/n");
            writer.write_all(number.as_bytes()).await.expect("Failed to write into file");
            writer.flush().await.expect("Failed to flush writer buffer")
        } else {
            *repeated_numbers.lock().unwrap() += 1;
        }
    }*/
    let Ok(output_file) = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true) 
        .open("uniques.log").await else {
            panic!("Failed to open the file")
        };
    let mut writer = BufWriter::new(output_file);
    let tried_numbers = tried_numbers.clone();
    let total_unique_numbers = total_unique_numbers.clone();
    let new_unique_numbers = new_unique_numbers.clone();
    let repeated_numbers = repeated_numbers.clone();
    let mut timer = time::interval(Duration::from_micros(100));
    loop {
        timer.tick().await;
        if tried_numbers.lock().unwrap().len() > 0 {
            let mut output: String = String::new();
            for number in tried_numbers.lock().unwrap().iter() {
                if !total_unique_numbers.lock().unwrap().contains(&number) {
                    *new_unique_numbers.lock().unwrap() += 1;
                    total_unique_numbers.lock().unwrap().insert(*number);
                    output.push_str(&number.to_string());
                    output.push_str("\n");
                } else {
                    *repeated_numbers.lock().unwrap() += 1;
                }
            }
            tried_numbers.lock().unwrap().clear();
            writer.write(output.as_bytes()).await; // utilizar buffer
        }
    }
}

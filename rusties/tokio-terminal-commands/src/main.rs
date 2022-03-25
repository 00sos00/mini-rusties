use tokio::spawn;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};


#[tokio::main]
async fn main() {
    let (tx, rx) = flume::bounded(1);

    spawn(async move {
        let mut reader = BufReader::new(stdin());

        loop {
            let mut string = String::new();
            reader.read_line(&mut string).await.unwrap();

            tx.send_async(string).await.unwrap();
        }
    });

    while let Ok(string) = rx.recv() {
        println!("{}", string.trim());
    }
}
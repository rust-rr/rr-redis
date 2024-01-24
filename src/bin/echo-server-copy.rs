use tokio::{
    io::{self},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (mut rd, mut wt) = socket.split();

            if io::copy(&mut rd, &mut wt).await.is_err() {
                eprintln!("Failed to copy");
            }
        });
    }
}

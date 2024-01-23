use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wt) = io::split(stream);

    tokio::spawn(async move {
        wt.write_all(b"hello\r\n").await?;
        wt.write_all(b"world\r\n").await?;

        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];
    loop {
        let n = rd.read(&mut buf[..]).await?;
        if n == 0 {
            break;
        }
        println!("Got {:?}", &buf[..n]);
    }

    Ok(())
}

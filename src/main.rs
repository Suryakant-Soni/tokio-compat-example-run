use futures::AsyncReadExt;
// use futures::AsyncReadExt;
// use futures::AsyncWriteExt;
// use futures::io::{AsyncReadExt, AsyncWriteExt};
use rand::Rng;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
// use tokio::task::spawn_blocking;
use tokio::time::{Duration, sleep};
use tokio_util::compat::TokioAsyncReadCompatExt;
#[tokio::main]
// async fn main() -> futures_util::io::Result<()> {
//     let file = NamedTempFile::new()?;
//     // this compat_file variable implements futures_io::AsyncWrite also along with default
//     // implmentation of tokio::io::AsyncWrite which is being implemented by the File objec of tokio::fs::fie
//     let mut compat_file = OpenOptions::new()
//         .read(true)
//         .write(true)
//         .open(file)
//         .await?
//         .compat_write();

//     //write some content using Futures implementation methods
//     compat_file.write_all(b"Hello world").await?;
//     compat_file.write_vectored(buf)

//     Ok(())
// }
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;

    // 🔹 Start an emitter as a client in the background
    tokio::spawn(async {
        // sleep(Duration::from_secs(1)).await; // Wait a bit before connecting
        if let Ok(mut client) = TcpStream::connect("127.0.0.1:8081").await {
            println!("Emitter connected as a client!");

            loop {
                let random_number: u32 = rand::rng().random_range(1..=100);
                let data = format!("{}\n", random_number);

                if let Err(e) = client.write_all(data.as_bytes()).await {
                    eprintln!("Emitter failed to send data: {}", e);
                    break;
                }
                println!("Emitter sent: {}", random_number);
                sleep(Duration::from_secs(1)).await;
            }
        }
    });

    while let Ok((stream, _addr)) = listener.accept().await {
        tokio::spawn(async move {
            // TcpStream implement AsyncRead and AsyncWrite
            // converts into a compatiblity layer's Compat object which then is compatible to both tokio::io::Async.... and future::io::Async... functions
            let mut compat_stream = stream.compat();
            let mut buffer = vec![0; 1024];
            loop {
                match compat_stream.read(&mut buffer).await {
                    Ok(0) => {
                        println!(
                            "Client disconnected: {}",
                            compat_stream.get_ref().local_addr().unwrap().ip()
                        );
                    }
                    Ok(n) => println!("Received {} bytes: {:?}", n, &buffer[..n]),
                    Err(e) => eprintln!("Failed to read: {}", e),
                }
            }
        });
    }
    Ok(())
}

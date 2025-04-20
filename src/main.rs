// use futures::AsyncReadExt;
// use rand::Rng;
// use tokio::io::AsyncWriteExt;
// use tokio::net::{TcpListener, TcpStream};
// use tokio::time::{Duration, sleep};
// use tokio_util::compat::TokioAsyncReadCompatExt;
// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:8081").await?;

//     // 🔹 Start an emitter as a client in the background
//     tokio::spawn(async {
//         loop {
//             match TcpStream::connect("127.0.0.1:8081").await {
//                 Ok(mut client) => {
//                     println!("Emitter connected as a client!");

//                     loop {
//                         let random_number: u32 = rand::rng().random_range(1..=100);
//                         let data = format!("{}\n", random_number);

//                         if let Err(e) = client.write_all(data.as_bytes()).await {
//                             eprintln!("Emitter failed to send data: {}", e);
//                             break;
//                         }
//                         println!("Emitter sent: {}", random_number);
//                         sleep(Duration::from_secs(1)).await;
//                     }
//                 }
//                 Err(e) => eprintln!("emitter faild to connect : {}", e),
//             }
//             println!("reconnecting in 3 seconds");
//             sleep(Duration::from_secs(3)).await;
//         }
//     });

//     while let Ok((stream, _addr)) = listener.accept().await {
//         tokio::spawn(async move {
//             // TcpStream implement AsyncRead and AsyncWrite
//             // converts into a compatiblity layer's Compat object which then is compatible to both tokio::io::Async.... and future::io::Async... functions
//             let mut compat_stream = stream.compat();
//             let mut buffer = vec![0; 1024];
//             loop {
//                 match compat_stream.read(&mut buffer).await {
//                     Ok(0) => {
//                         println!(
//                             "Client disconnected: {}",
//                             compat_stream.get_ref().local_addr().unwrap().ip()
//                         );
//                         break;
//                     }
//                     Ok(n) => println!("Received {} bytes: {:?}", n, &buffer[..n]),
//                     Err(e) => eprintln!("Failed to read: {}", e),
//                 }
//             }
//         });
//     }
//     Ok(())
// }


//! ## Bridging Tokio and Futures I/O with `compat()`
//!
//! The [`compat()`](TokioAsyncReadCompatExt::compat) function provides a compatibility layer
//! that allows types implementing [`tokio::io::AsyncRead`] or [`tokio::io::AsyncWrite`]
//! to be used as their [`futures::io::AsyncRead`] or [`futures::io::AsyncWrite`] counterparts — and vice versa.
//!
//! This is especially useful when working with libraries that expect I/O types from one ecosystem
//! (usually `futures`) but you’re using types from the other (usually `tokio`).
//!
//! ## Compatibility Overview
//!
//! | If the inner type implements...       | Then `Compat<T>` implements...            |
//! |--------------------------------------|-------------------------------------------|
//! | [`tokio::io::AsyncRead`]             | [`futures::io::AsyncRead`]                |
//! | [`futures::io::AsyncRead`]           | [`tokio::io::AsyncRead`]                  |
//! | [`tokio::io::AsyncWrite`]            | [`futures::io::AsyncWrite`]               |
//! | [`futures::io::AsyncWrite`]          | [`tokio::io::AsyncWrite`]                 |
//!
//! ## Feature Flag
//!
//! This functionality is available through the `compat` feature flag:
//!
//! ```toml
//! tokio-util = { version = "...", features = ["compat"] }
//! ```
//!
//! ## Example 1: Tokio → Futures (`AsyncRead`)
//!
//! This example demonstrates sending data over a [`tokio::net::TcpStream`] and using
//! [`futures::io::AsyncReadExt::read`] to read it after adapting the stream via `compat()`.
//!
//! ```no_run
//! use tokio::net::{TcpListener, TcpStream};
//! use tokio::io::AsyncWriteExt;
//! use tokio_util::compat::TokioAsyncReadCompatExt;
//! use futures::io::AsyncReadExt;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let listener = TcpListener::bind("127.0.0.1:8081").await?;
//!
//!     tokio::spawn(async {
//!         let mut client = TcpStream::connect("127.0.0.1:8081").await.unwrap();
//!         client.write_all(b"Hello World").await.unwrap();
//!     });
//!
//!     let (stream, _) = listener.accept().await?;
//!
//!     // Adapt `tokio::TcpStream` to be used with `futures::io::AsyncReadExt`
//!     let mut compat_stream = stream.compat();
//!     let mut buffer = [0; 20];
//!     let n = compat_stream.read(&mut buffer).await?;
//!     println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Example 2: Futures → Tokio (`AsyncRead`)
//!
//! The reverse is also possible: you can take a [`futures::io::AsyncRead`] (e.g. a cursor)
//! and adapt it to be used with [`tokio::io::AsyncReadExt::read_to_end`].
//!
//! ```no_run
//! use futures::io::Cursor;
//! use tokio_util::compat::FuturesAsyncReadCompatExt;
//! use tokio::io::AsyncReadExt;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let reader = Cursor::new(b"Hello from futures");
//!     let mut compat_reader = reader.compat();
//!     let mut buf = Vec::new();
//!     compat_reader.read_to_end(&mut buf).await?;
//!     println!("Received: {}", String::from_utf8_lossy(&buf));
//!     Ok(())
//! }
//! ```
//!
//! ## Common Use Cases
//!
//! - Using `tokio` sockets with `async-tungstenite`, `async-compression`, or `futures-rs`-based libraries
//! - Bridging I/O interfaces between mixed-ecosystem libraries
//! - Avoiding rewrites or duplication of I/O code in async environments
//!
//! ## See Also
//!
//! - [`Compat`](Compat) type
//! - [`TokioAsyncReadCompatExt`](TokioAsyncReadCompatExt)
//! - [`FuturesAsyncReadCompatExt`](FuturesAsyncReadCompatExt)
//! - [`tokio::io` module](tokio::io)
//! - [`futures::io` module](futures::io)

// use futures::io::AsyncReadExt;
// use tokio::io::AsyncWriteExt;
// use tokio::net::{TcpListener, TcpStream};
// use tokio_util::compat::TokioAsyncReadCompatExt;
//
// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     // simulate a local server using TCP listener
//     let listener = TcpListener::bind("127.0.0.1:8081").await?;
//
//     // spawn a TCPStream that connects and then send data
//     tokio::spawn(async {
//         let mut client = TcpStream::connect("127.0.0.1:8081").await.unwrap();
//         let tx_buffer = b"Hello World";
//         client.write_all(tx_buffer).await.unwrap();
//     });
//
//     // since TcpStream already implements tokio::io::AsyncRead, the compat::new which is called here
//     // gives the object which implements futures_io::AsyncRead
//     let (stream, _) = listener.accept().await?;
//
//     // convert `TcpStream` to `futures::io::AsyncReadExt` using `compat()`
//     let mut compat_stream = stream.compat();
//     let mut buffer = [0; 20];
//
//     let n = compat_stream.read(&mut buffer).await?;
//     println!("Received {}", String::from_utf8_lossy(&buffer[..n]));
//
//     Ok(())
// }

use futures::io::Cursor;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let reader = Cursor::new(b"Hello from futures");
    let mut compat_reader = reader.compat();
    let mut buf = Vec::new();
    compat_reader.read_to_end(&mut buf).await?;
    println!("Received: {}", String::from_utf8_lossy(&buf));
    Ok(())
}

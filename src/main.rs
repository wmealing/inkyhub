use async_std::os::unix::net::UnixListener;
use async_std::os::unix::net::UnixStream;
use async_std::prelude::*;


use async_std::io;

use async_std::task;



async fn handle_client(mut stream: UnixStream) -> io::Result<String> {
    println!("handle client done");
    let mut buf = String::new();
    stream.read_to_string(&mut buf).await?;
    println!("BUF: {}", buf);
    Ok(buf.into())
}

fn main() -> std::io::Result<()> {


    let mut clients = Vec::new();

    task::block_on(async {
        
        let listener = UnixListener::bind("/tmp/rust-uds.sock").await?;
        println!("Listening on {:?}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            let client_result = task::spawn(async {
                let x = handle_client(stream).await.unwrap();
                x
            });

            let file_path = client_result.await;
            clients.push(file_path);
            println!("Now clients is: {} long", clients.len());
        }


        Ok(())

    })
    
}

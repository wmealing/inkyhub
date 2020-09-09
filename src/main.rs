use async_std::fs;
use async_std::os::unix::net::UnixListener;
use async_std::os::unix::net::UnixStream;
use async_std::prelude::*;
use async_std::task;
use async_std::path::PathBuf;

async fn write_msg(pa: PathBuf) -> std::io::Result<()> {
    println!("Writing msg {:?}", pa);

    let r = fs::write(pa, b"Hello world!").await?;
    Ok(r)
    
}

async fn broadcast_to(clients: Vec<async_std::fs::DirEntry>) ->    std::io::Result<String> {
    println!("BROADCASTING TO!");

    for client in &clients {
        let p = client.clone();
        write_msg(p.path()).await;
    }

    Ok("Yeah".into())
}

async fn get_client_dirs() -> std::io::Result<Vec<async_std::fs::DirEntry>> {
    let mut clients = Vec::new();

    let mut dir = fs::read_dir("/home/wmealing/inky/sensors/").await?;

    while let Some(res) = dir.next().await {
        let entry = res?;
        clients.push(entry);
    }

    Ok(clients)
}

async fn process_stream(mut st: UnixStream) -> std::io::Result<()> {
    println!("Processing stream");

    let mut response = Vec::new();
    let out = st.read_to_end(&mut response).await?;
    println!("OUT: {}", out);
    println!("DONE");
    Ok(())
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    //    let mut clients = Vec::new();

    let listener = UnixListener::bind("/home/wmealing/inky/input").await?;

    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let mut stream = stream?;
        stream.write_all(b"Ready> ").await?;
        task::block_on(async {
            let _a = process_stream(stream).await;
        });

        match get_client_dirs().await {
            Ok(clients) => {
                match task::block_on(broadcast_to(clients)) {
                    Ok(_a) => { println!("Broadcast done") },
                    Err(_e) => { println!("Error writing..") },
                }
            },
            Err(_e) => println!("ERROR"),
        }
    }

    Ok(())
}

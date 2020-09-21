use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use smol::{io, Async, Timer};

const GARBAGE_PAUSE_DUR_SECS: u64 = 5;
const GPD: Duration = Duration::from_secs(GARBAGE_PAUSE_DUR_SECS);
const GARBAGE: &[u8] = b"garbage\n";

async fn tarpit(stream: Async<TcpStream>) -> io::Result<()> {
    loop {
        Timer::after(GPD).await;
        let mut reader = io::BufReader::new(GARBAGE);
        io::copy(&mut reader, &mut &stream).await?;
    }
}

fn main() -> io::Result<()> {

    smol::block_on(async {
        // Create a listener.
        let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 8080))?;
        println!("Listening on {}", listener.get_ref().local_addr()?);

        // Accept clients in a loop.
        loop {
            let (stream, peer_addr) = listener.accept().await?;
            println!("Accepted client: {}", peer_addr);
            smol::spawn(tarpit(stream)).detach();
        }
    })

}

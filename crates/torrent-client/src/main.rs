//! Command-line entry point. Async orchestration belongs here; protocol rules
//! belong in `torrent-core`.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Torrente — a BitTorrent client built for learning");
    //println!("Core: {}", torrent_core::crate_purpose());
    Ok(())
}

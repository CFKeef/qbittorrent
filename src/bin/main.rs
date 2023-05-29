use qbittorrent as qbit;
use tokio;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _api = qbit::Api::new("admin", "adminadmin", "http://localhost:8080")
        .await
        .unwrap();

    let torrents = _api.get_torrent_list().await;
    dbg! {&torrents};
    torrents.unwrap();
}

use std::sync::Arc;
use std::time::Duration;

use clap::Parser;
use tg_ws_proxy_rs::check::run_check;
use tg_ws_proxy_rs::config::Config;
use tg_ws_proxy_rs::pool::WsPool;
use tg_ws_proxy_rs::proxy::handle_client;
use tg_ws_proxy_rs::ws_client::{
    TgWsStream, WsConnectResult, connect_cf_worker_ws_for_dc, connect_cf_ws_for_dc, connect_ws,
    connect_ws_for_dc,
};
use tokio::net::{TcpListener, TcpStream};

#[test]
fn old_ws_client_public_signatures_still_compile() {
    let _connect = connect_ws(
        "127.0.0.1",
        "kws2.web.telegram.org",
        false,
        Duration::from_millis(1),
    );
    let _direct_dc = connect_ws_for_dc("127.0.0.1", 2, false, false, Duration::from_millis(1));
    let cf_domains = ["example.net".to_string()];
    let _cf_dc = connect_cf_ws_for_dc(2, &cf_domains, false, false, Duration::from_millis(1));
    let _worker = connect_cf_worker_ws_for_dc(
        "worker.example.dev",
        "149.154.167.51",
        2,
        false,
        false,
        Duration::from_millis(1),
    );
}

#[test]
fn old_check_and_pool_public_signatures_still_compile() {
    let config = Config::try_parse_from(["tg-ws-proxy", "--check"]).unwrap();
    let _check = run_check(&config);
    let _pool = WsPool::new(0, Duration::from_secs(55));
}

#[tokio::test]
async fn old_handle_client_public_signature_still_compiles() {
    let config = Config::try_parse_from(["tg-ws-proxy"]).unwrap();
    let pool = Arc::new(WsPool::new(0, Duration::from_secs(55)));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let client = TcpStream::connect(addr);
    let accepted = listener.accept();
    let (client, accepted) = tokio::join!(client, accepted);
    let _client = client.unwrap();
    let (server, peer) = accepted.unwrap();

    let _future = handle_client(server, peer, config, pool);
}

#[allow(dead_code)]
fn connected_variant_payload_stays_unboxed(result: WsConnectResult) -> Option<TgWsStream> {
    match result {
        WsConnectResult::Connected(ws) => Some(ws),
        _ => None,
    }
}

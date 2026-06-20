use tg_ws_proxy_rs::default_domains::{deobfuscate, fetch_default_domains};

#[test]
fn deobfuscate_public_api_still_decodes_known_domain() {
    assert_eq!(deobfuscate("virkgj.com"), Some("pclead.co.uk".to_string()));
}

#[test]
fn fetch_default_domains_public_api_keeps_zero_arg_signature() {
    let _future = fetch_default_domains();
}

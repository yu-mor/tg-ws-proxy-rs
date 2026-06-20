use super::*;

#[test]
fn deobfuscate_known_pair() {
    assert_eq!(deobfuscate("virkgj.com"), Some("pclead.co.uk".to_string()));
}

#[test]
fn deobfuscate_rejects_non_com() {
    assert!(deobfuscate("example.org").is_none());
    assert!(deobfuscate("nocomhere").is_none());
    assert!(deobfuscate("").is_none());
}

#[test]
fn parse_domain_list_skips_blank_lines_and_comments() {
    let domains = parse_domain_list("# header\nvirkgj.com\n\n# comment\nvmmzovy.com\n");

    assert_eq!(domains.len(), 2);
    assert!(domains.iter().all(|domain| domain.ends_with(".co.uk")));
}

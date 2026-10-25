use english::english;

fn main() {
    let ok = english!(The dog barks. I 'm happy! She doesn 't care.);
    assert_eq!(ok.to_string(), "The dog barks. I'm happy! She doesn't care.");

    let _ = english!(He don 't care);
}

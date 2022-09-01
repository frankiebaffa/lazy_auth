use {
    base64::{encode_config, URL_SAFE_NO_PAD},
    orion::aead::SecretKey,
};
fn main() {
    let secret_key = SecretKey::default();
    let bytes = secret_key.unprotected_as_bytes();
    let base64 = encode_config(bytes, URL_SAFE_NO_PAD);
    println!("{}", base64);
}

use lazy_auth::LAClient;
fn main() {
    const SECRET: &'static str = "8fhjjdljksjkf89d9";
    let pin = std::env::args().nth(1).unwrap();
    if LAClient::validate(&pin, SECRET, None)
        .unwrap()
    {
        println!("Authenticated :)");
    } else {
        println!("Not authenticated :(");
    }
}

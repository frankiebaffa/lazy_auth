use lazy_auth::LAClient;
fn main() {
    const SECRET: &'static str = "8fhjjdljksjkf89d9";
    const APP_NAME: &'static str = "LAClientTestApp";
    const APP_INFO: &'static str = "SomeInfoOverHere";
    let data_url = LAClient::pair(
        APP_NAME, APP_INFO, SECRET, None
    ).unwrap().unwrap();
    println!("{}", data_url);
}

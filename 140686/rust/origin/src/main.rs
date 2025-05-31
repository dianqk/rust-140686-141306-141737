use reqwest::{blocking::Client, Method};

fn main() {
    Client::new().request(Method::GET, "http://localhost:8888/hello").send().ok();
}

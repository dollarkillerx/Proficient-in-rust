use sphttp::*;
use futures::executor::block_on;

fn main() {
    // httpserver::httpserver();
    block_on(httpclient::client());
}

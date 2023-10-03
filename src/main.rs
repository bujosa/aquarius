use aquarius::api::App;
use poem::{listener::TcpListener, Server};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(App::new())
        .await
}

use tokio::sync::oneshot;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Response;
    }
}

struct Response {
    content: oneshot::Sender<i32>,
}

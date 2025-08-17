use std::sync::OnceLock;
use tokio::runtime::Runtime;

pub fn get_or_init_runtime() -> &'static Runtime {
    static RT: OnceLock<Runtime> = OnceLock::new();

    RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to build tokio runtime")
    })
}

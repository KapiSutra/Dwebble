use std::sync::OnceLock;

#[unsafe(no_mangle)]
pub extern "C" fn tokio_mt_spawn(work: extern "C" fn() -> bool) {
    static RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

    let rt = RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .build()
            .expect("Failed to build tokio runtime")
    });

    rt.spawn(async move {
        work();
        true
    });
}

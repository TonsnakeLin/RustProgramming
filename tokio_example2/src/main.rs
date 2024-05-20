use tokio::runtime::{Builder, Runtime};
use std::sync::Arc;

fn new_multi_thread_runtime() -> Arc<Runtime> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("linpin-thread")
        .thread_stack_size(16 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();
    Arc::new(runtime)
}

fn main() {
    let rt = new_multi_thread_runtime();
    rt.block_on(async {
        println!("this is executed by block on")
    });
}

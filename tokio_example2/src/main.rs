use tokio::runtime::{Builder, Runtime};
use std::sync::Arc;
use std::thread as StdThread;
use std::time::{Duration, SystemTime, Instant};
use chrono::Local;

#[allow(dead_code)]
fn new_arc_multi_thread_runtime() -> Arc<Runtime> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("linpin-thread")
        .thread_stack_size(16 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();
    Arc::new(runtime)
}


#[allow(dead_code)]
fn new_multi_thread_runtime() -> Runtime {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("linpin-thread")
        .thread_stack_size(16 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();
    runtime
}

fn mew_current_thread_runtime() -> Runtime {
    let runtime = Builder::new_current_thread()
        .worker_threads(1)
        .thread_name("linpin-curr-thread")
        .enable_all()
        .build()
        .unwrap();
    runtime
}


fn main() {
    println!("main thread [{:?}], current_time [{:?}], current_instant [{:?}], current_timestamp [{:?}]", 
            StdThread::current(), SystemTime::now(), Instant::now(), Local::now());
    // let rt = new_arc_multi_thread_runtime();
    let rt = new_multi_thread_runtime();
    rt.block_on(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("thread [{:?}], current_time [{:?}], this is executed by block on in multi-thread-runtime", 
                StdThread::current(), SystemTime::now())
    });

    let rt_cur = mew_current_thread_runtime();
    rt_cur.block_on(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("thread [{:?}], current_time [{:?}], this is executed by block on in current-thread-runtime", 
                StdThread::current(), SystemTime::now())
    });

    println!("
    
    ");

    let _enter_guard = rt.enter();
    rt.spawn(tokio::task::spawn(async {
        tokio::time::sleep(Duration::from_millis(500)).await;
        println!("thread [{:?}], task1 is executed", std::thread::current());
    }));
    drop(_enter_guard);

    rt.spawn(async {
        println!("thread [{:?}], task2 is executed", std::thread::current());
    });

    std::thread::sleep(Duration::from_secs(2));
}

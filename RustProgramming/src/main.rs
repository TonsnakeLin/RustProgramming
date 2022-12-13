#![allow(dead_code, unused_variables, unused_imports, non_snake_case)]
use core::time;
use std::sync::{Arc, Mutex, Condvar};
use std::sync::mpsc::channel;
use std::time::Instant;
use std::thread;

use futures::executor::block_on;

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
    time::Duration,
};

///////////////////////////////////////////////////////////////////////////////////
use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        sync::mpsc::{sync_channel, Receiver, SyncSender},
    },
};


fn wait_thread_started() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        println!("notifier: enter lock");
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        println!("notifier: notify condvar");
        cvar.notify_one();
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    println!("waiter: enter lock");
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("waiter: enter wait");
        started = cvar.wait(started).unwrap();
    }
}

fn thread_wake_up_time() {
    let (sender, receiver) = channel();
    let pair: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2:  Arc<(Mutex<bool>, Condvar)> = Arc::clone(&pair);
    let mut vec2 = Vec::new();
    let mut vec1 = Vec::new();
    let one_second = time::Duration::from_secs(1);

    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;        
        for _i in 0..100 {
            println!("waiter: lock");
            let mut flag = lock.lock().unwrap();
            println!("enter condvar wait");
            *flag = true;
            flag = cvar.wait(flag).unwrap();
            let now = Instant::now();
            drop(flag);
            
            println!("after time:{:?}", now);
            sender.send(now).unwrap();
            println!("waiter: unlock");
        }
    });    
    thread::sleep(one_second);
    let (lock, cvar) = &*pair;
    for _i in 0..100 {
        println!("notifier: lock");
        let flag = lock.lock().unwrap();
        // We notify the condvar that the value has changed.
        let now = Instant::now();        
        cvar.notify_one();
        drop(flag);
        println!("before time:{:?}", now);
        vec1.push(now);  
        let now2 = receiver.recv().unwrap();
        vec2.push(now2);
        println!("notifier: unlock");
        thread::sleep(one_second);
    }

    let len = std::cmp::min(vec1.len(), vec2.len());
    let mut vec3 = Vec::new();
    let mut sum = 0;
    for _i in 0..len {
        let t1 = vec1.get(_i).unwrap();
        let t2 = vec2.get(_i).unwrap();
        let delta = t2.duration_since(t1.clone()).as_micros();
        sum += delta;
        vec3.push(delta);
    }
    // println!("vec1: {:?}", vec1);
    // println!("vec2: {:?}", vec2);
    
    println!("avg: {:?}", sum as f64/len as f64);
    vec3.sort();
    println!("vec3: {:?}", vec3);


    let p80 = (len as f64 * 0.8) as usize;
    let p95 = (len as f64 * 0.95) as usize;
    
    println!("p80: {:?}, p95: {:?}, max: {:?}", vec3.get(p80), vec3.get(p95), vec3.get(len - 1));
}

fn thread_wake_up_time2() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        for _i in 0..2 {
            thread::sleep(time::Duration::from_secs(1));            
            println!("notifier: enter lock");
            let mut started = lock.lock().unwrap();
            *started = true;
            // We notify the condvar that the value has changed.
            println!("notifier: notify condvar");
            cvar.notify_one();
            let now = Instant::now();
            println!("before time:{:?}", now);
        }        
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    println!("waiter: enter lock");
    for _i in 0..2 {
        let mut started = lock.lock().unwrap();    
        println!("waiter: enter wait");
        started = cvar.wait(started).unwrap();
        let now = Instant::now();
        println!("after time:{:?}", now);
        drop(started);
    }  
}

fn thread_wake_up_time3() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        for _i in 0..2 {
                       
            println!("waiter: enter lock");
            let mut started = lock.lock().unwrap();
            *started = true;
            // We notify the condvar that the value has changed.
            println!("waiter: enter wait");
            started = cvar.wait(started).unwrap();
            drop(started);
            let now = Instant::now();
            println!("after time:{:?}", now);
        }        
    });

    thread::sleep(time::Duration::from_secs(1)); 
    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    println!("notify: enter lock");
    for _i in 0..2 {
        let started = lock.lock().unwrap();    
        println!("notify: enter wait");
        let now = Instant::now();
        println!("before time:{:?}", now);
        cvar.notify_one();
               
        drop(started);
        thread::sleep(time::Duration::from_secs(1)); 
    }    
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////////////
async fn learn_song() {
    println!("learn song");
}

async fn sing_song() {
    println!("sing song");
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    learn_song().await; 
    sing_song().await;
    println!("learn_and_sing end");
}

async fn song_or_dance()->((), ()) {
    let f1 = learn_and_sing();
    let f2 = dance();
    println!("begin to join");
    futures::join!(f1, f2)
}

////////////////////////////////////////////////////////////////////////////////
/// ///////////////////////////////////////////////////////////////////////////
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// Shared state between the future and the waiting thread
struct SharedState {
    /// Whether or not the sleep time has elapsed
    completed: bool,

    /// The waker for the task that `TimerFuture` is running on.
    /// The thread can use this after setting `completed = true` to tell
    /// `TimerFuture`'s task to wake up, see that `completed = true`, and
    /// move forward.
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        let handle = thread::current();
        println!("TimerFuture::poll, [{:?}]", handle);
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // Set waker so that the thread can wake up the current task
            // when the timer has completed, ensuring that the future is polled
            // again and sees that `completed = true`.
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    /// Create a new `TimerFuture` which will complete after the provided
    /// timeout.
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        let handle = thread::current();
        println!("TimerFuture::new, [{:?}]", handle);
        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            println!("TimerFuture thread begin");
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                println!("TimerFuture thread completed");
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}


/// Task executor that receives tasks off of a channel and runs them.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

/// A future that can reschedule itself to be polled by an `Executor`.
struct Task {
    /// In-progress future that should be pushed to completion.
    ///
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread,
    /// so we need to use the `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead.
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// Handle to place the task itself back onto the task queue.
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}


impl Executor {
    fn run(&self) {
        let handle = thread::current();
        println!("Executor thread [{:?}]", handle);
        while let Ok(task) = self.ready_queue.recv() {
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // Create a `LocalWaker` from the task itself
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>` is a type alias for
                // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                // We can get a `Pin<&mut dyn Future + Send + 'static>`
                // from it by calling the `Pin::as_mut` method.
                println!("Executor begin to run poll, [{:?}]", handle);
                if future.as_mut().poll(context).is_pending() {
                    // We're not done processing the future, so put it
                    // back in its task to be run again in the future.
                    println!("Executor ruturned pending after polling, [{:?}]", handle);
                    *future_slot = Some(future);
                } else {
                    println!("executor ruturned ready after polling, [{:?}]", handle);
                }                
            }
        }
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    // Maximum number of tasks to allow queueing in the channel at once.
    // This is just to make `sync_channel` happy, and wouldn't be present in
    // a real executor.
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}



fn main() {

    // block_on(song_or_dance());

    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(async {
        println!("howdy!");
        let handle = thread::current();
        println!("Executing future thread {:?}", handle);
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();

    println!("main thread end");
}

#![allow(dead_code, unused_variables, unused_imports, non_snake_case)]
use core::time;
use std::sync::{Arc, Mutex, Condvar};
use std::sync::mpsc::channel;
use std::time::Instant;
use std::thread;

use futures::executor::block_on;


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
}

async fn song_or_dance() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2)
}

fn main() {
    block_on(song_or_dance());
    println!("main thread end");
}

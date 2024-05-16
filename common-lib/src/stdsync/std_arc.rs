use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::OnceCell;

#[allow(dead_code)]
pub struct LoadedFile {
    content: Arc<[u8]>,
}
#[allow(dead_code)]
pub enum CacheKvFile {
    Mem(Arc<OnceCell<LoadedFile>>),
    Fs(Arc<PathBuf>),
}
pub fn test_arc_strong_count() {
    let data = 0;
    let arc1 = Arc::new(&data);
    
    let arc2 = Arc::clone(&arc1);
    let _ = Arc::clone(&arc1);
    println!("arc1 strong_count: {}", Arc::strong_count(&arc1));
    println!("arc2 strong_count: {}", Arc::strong_count(&arc2));
    #[warn(unused_variables)]
    let _ = Arc::downgrade(&arc2);
    println!("arc1 strong_count: {}", Arc::strong_count(&arc1));
    println!("arc2 strong_count: {}", Arc::strong_count(&arc2));
    println!("arc2 weak_count: {}", Arc::weak_count(&arc2));

    let d1 = Arc::try_unwrap(arc1);
    println!("d1:{}, arc1 strong_count: {}", d1.unwrap(), Arc::strong_count(&arc2))
}

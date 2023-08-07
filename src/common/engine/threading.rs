use std::{thread::{Thread, JoinHandle}, any::Any, sync::{atomic::AtomicBool, Arc, Mutex}, ops::IndexMut};


pub struct Threader {
    handle: Option<JoinHandle<i32>>,
    alive: Arc<AtomicBool>,
}

impl Threader {
    pub fn new() -> Threader{
        Threader { handle: Option::None , alive: Arc::new(AtomicBool::new(false))}
    }

    pub fn start<F>(&mut self, fun: F) where F: 'static + Send + FnMut() -> i32{
        self.alive.store(true, std::sync::atomic::Ordering::SeqCst);

        let alive = self.alive.clone();

        self.handle = Some(std::thread::spawn(move ||
            {
            let mut fun = fun;
            fun()
            }
        ));

    }
    
    pub fn stop(&mut self) -> i32{
        self.handle.take().expect("failed to stop thread.").join().expect("failed to join thread.")
    }

    pub fn isAlive(&mut self) -> bool {
        self.alive.load(std::sync::atomic::Ordering::SeqCst)
    }

}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct DictEntry<K,V> where K: Clone,{
    key: K,
    value: V,
}

pub struct Dict<K,V> where K: Clone {
    data: Vec<DictEntry<K,V>>,
}

pub trait ThreadingDict {
    fn get(&self, key: usize) -> Option<Arc<Mutex<Threader>>>;
    fn insert(&mut self, key: usize, value: Arc<Mutex<Threader>>);
    fn get_or_insert(&mut self, key: usize, insert_case: Arc<Mutex<Threader>>) -> Option<Arc<Mutex<Threader>>>;
    fn new() -> Dict<usize, Arc<Mutex<Threader>>>;
}

impl ThreadingDict for Dict<usize, Arc<Mutex<Threader>>>{
    fn get(&self, key: usize) -> Option<Arc<Mutex<Threader>>> {
        for entry in &self.data{
            if(entry.key == key){
                return Some(entry.value.clone());
            }
        }
        return None;
    }

    fn insert(&mut self, key: usize, value: Arc<Mutex<Threader>>){
        for entry in &mut self.data {
            if(entry.key == key){
                let temp = entry.value.clone();
                entry.value = value.clone();
                
            }
        }
        self.data.push(DictEntry { key: key, value: value });
    }

    fn get_or_insert(&mut self, key: usize, insert_case: Arc<Mutex<Threader>>) -> Option<Arc<Mutex<Threader>>> {
        if(self.get(key).is_none()){
            self.insert(key, insert_case);
        }
        return self.get(key);
    }
    fn new() -> Dict<usize, Arc<Mutex<Threader>>>{
        Dict {data: Vec::<DictEntry<usize, Arc<Mutex<Threader>>>>::new()}
    }
}
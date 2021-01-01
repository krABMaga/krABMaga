use criterion::*;
use DBdashmap::DBashMap;

use dashmap::DashMap;
use std::sync::Arc;
use hashbrown::HashMap;

fn bench_ourmap(c: &mut Criterion){
    let mut group = c.benchmark_group(format!("Field"));

    let mut h =  HashMap::new();
    insert_hash(&mut h, 0 ,100_000_000);
    
    assert_eq!(h.len(),100_000_000);
    group
        .bench_function(
            BenchmarkId::new("get",100_000_000), 
            |b| { b.iter(|| h.get(&1_000_000)) }
        );
    group.finish();
}



criterion_group!(benches, bench_ourmap);
criterion_main!(benches);



fn insert(  m: &DBashMap<usize,usize>,start_n:usize,end_n:usize){
    for i in start_n..end_n{
        m.insert(i,i);
    }
}

fn insert_hash(  m: &mut HashMap<usize,usize>,start_n:usize,end_n:usize){
    for i in start_n..end_n{
        m.insert(i,i);
    }
}

fn read(  m: &HashMap<usize,usize>,start_n:usize,end_n:usize){
    for i in start_n..end_n{
        let n = m.get(&i).unwrap();
        
    }
}

fn read_dashmap(  m: &DBashMap<usize,usize>,start_n:usize,end_n:usize){
    for i in start_n..end_n{
        let n = m.get(&i).unwrap();
    }
}

fn insert_mutex(  m: Arc<std::sync::Mutex<HashMap<usize,usize>>>,start_n:usize,end_n:usize){
    let mut m = m.lock().unwrap();
    for i in start_n..end_n{
        m.insert(i,i);
    }
}

fn read_mutex(  m: Arc<std::sync::Mutex<HashMap<usize,usize>>>,start_n:usize,end_n:usize){
    let m = m.lock().unwrap();
    for i in start_n..end_n{
        let n = m.get(&i).unwrap();
    }
}

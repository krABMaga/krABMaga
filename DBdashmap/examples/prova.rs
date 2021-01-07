extern crate DBdashmap;

use DBdashmap::DBashMap;
use std::sync::Arc;
use hashbrown::HashMap;



fn main(){
    // println!("name;num_op;num_thread;write_time;read_time;update_time");
    
    // for (n,threads) in [ (200_000_000,4), (200_000_000,8), (200_000_000,16), (200_000_000,32),
    //                     (500_000_000,4), (500_000_000,8), (500_000_000,16), (500_000_000,32),
    //                     (800_000_000,4), (800_000_000,8), (800_000_000,16), (800_000_000,32),
    //                     (1_000_000_000,4), (1_000_000_000,8), (1_000_000_000,16), (1_000_000_000,32),
    //                     ].iter(){
    
    //     if *threads > 8{
    //         continue;
    //     }
    //     let (seq_insert,seq_read) = seq( n/100);
    //     let (single_insert,single_read) = par_single_hashmap( (n/100) /threads,  *threads );
    //     let (sharded_insert,sharded_read,sharded_update) = par_sharded_hashmap( (n/100) /threads, *threads );

    //     let (seq_insert,seq_read) = ( to_f64_sec(seq_insert),to_f64_sec(seq_read) ) ;
    //     let (single_insert,single_read) = ( to_f64_sec(single_insert),to_f64_sec(single_read)) ;
    //     let (sharded_insert,sharded_read,sharded_update) =  (to_f64_sec(sharded_insert),to_f64_sec(sharded_read),to_f64_sec(sharded_update) ) ;


    //     println!(
    //     "Sequential_Hashmap;{};1;{:.4};{:.4};0\n\
    //     Single_Hashmap;{};{};{:.4};{:.4};0\n\
    //     Sharded_HashMap;{};{};{:.4};{:.4};{:.6}", 
    //     n/100,seq_insert,seq_read,
    //     n/100,threads,single_insert,single_read,
    //     n/100,threads,sharded_insert,sharded_read,sharded_update);
    // }
        let mut d = DBashMap::new();
        let mut h = HashMap::new();

        insert(&d, 0 ,100_000);
        d.update();
        insert_hash(&mut h, 0 ,100_000);


        assert_eq!(d.r_len(),100_000);
        assert_eq!(h.len(),100_000);

        let start_d= std::time::Instant::now();
        d.get(&9999);
        let d_time = start_d.elapsed();

        let start_h = std::time::Instant::now();
        h.get(&9999);
        let h_time = start_h.elapsed();

        println!("d:{:?} h:{:?}",d_time,h_time);
   
}

fn to_f64_sec( d: std::time::Duration ) -> f64{
    d.as_nanos() as f64 * 1e-9
}


fn seq(n_base: usize) -> (std::time::Duration, std::time::Duration){
    let mut m1 = HashMap::new();
    let start_insert = std::time::Instant::now();
    insert_hash(&mut m1,0,n_base);
    let time_insert = start_insert.elapsed();
    let start_read = std::time::Instant::now();
    read(&m1,0,n_base);
    let time_read = start_read.elapsed();
    (time_insert,time_read)   
}

fn par_single_hashmap(n_base: usize,n_thread:usize) -> (std::time::Duration,std::time::Duration) {
    let mut m1 = Arc::new(std::sync::Mutex::new(HashMap::new()));

    let mut handles = vec![];
   // let n_base = 5000000;

    let start_insert = std::time::Instant::now();
    for i in 0..n_thread{
        let local_m1 = Arc::clone(&m1);
        let handle = std::thread::spawn( move ||{
         
            insert_mutex(local_m1,n_base*i, n_base*(i+1) );
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    let time_insert = start_insert.elapsed();

  

    let mut handles = vec![];
    let start_read = std::time::Instant::now();
    for i in 0..n_thread{
        let local_m1 = Arc::clone(&m1);
        let handle = std::thread::spawn( move ||{
            
            read_mutex(local_m1,n_base*i, n_base*(i+1));
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    let time_read = start_read.elapsed();


    (time_insert,time_read)
    
}

fn par_sharded_hashmap(n_base: usize,n_thread:usize) -> (std::time::Duration,std::time::Duration,std::time::Duration) {
    let mut m1 = Arc::new(DBashMap::new());
    let mut handles = vec![];
  // let n_base = 5000000;

    let start_insert = std::time::Instant::now();
    for i in 0..n_thread{
        let local_m1 = Arc::clone(&m1);
        let handle = std::thread::spawn( move ||{
         
            insert(&*local_m1,n_base*i, n_base*(i+1) );
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    let time_insert = start_insert.elapsed();

    let start_update = std::time::Instant::now();
    Arc::get_mut(&mut m1).unwrap().update();
    let time_update = start_update.elapsed();

    let mut handles = vec![];
    let start_read = std::time::Instant::now();
    for i in 0..n_thread{
        let local_m1 = Arc::clone(&m1);
        let handle = std::thread::spawn( move ||{
            read_dashmap(&*local_m1,n_base*i, n_base*(i+1));
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    let time_read = start_read.elapsed();


    (time_insert,time_read,time_update)
    
}


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

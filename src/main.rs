use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;
use std::time::Instant;

pub fn is_bit_set(number: u64, k: u64, bit: u64) -> bool {
    //stole from stackoverflow
    (number & (1 << k)) >> k == bit
}

pub fn get_nos(to_sort: &Vec<u64>, writer: &mut Vec<u64>, pos: usize, bit: u64) -> usize {
    writer.clear();
    let mut len = 0;
    for j in to_sort.iter() {
        if is_bit_set(*j, pos as u64, bit) {
            writer.push(*j);
            len += 1;
        }
    }

    len
}

fn main() {
    let no_list: Vec<_> = {
        let mut string = String::new();
        let _ =std::io::stdin().read_line(&mut string);
        string
            .split_whitespace()
            .map(|c| u64::from_str_radix(c, 10).unwrap())
            .collect()
    };

    let no_of_bits = std::mem::size_of::<u64>() * 8;

    let vec_size = no_list.len();
    let mut _first = Vec::with_capacity(vec_size);
    let mut _second = Vec::with_capacity(vec_size);

    let mut rw_no =  Arc::new(RwLock::new(no_list));
    let first =  Arc::new(Mutex::new(_first));
    let second = Arc::new(Mutex::new(_second));

    let time = Instant::now();
    for pos in 0..no_of_bits {
        let first_clone = Arc::clone(&first);
        let second_clone = Arc::clone(&second);

        let rw_no_clone = Arc::clone(&rw_no);
        let a = thread::spawn(move || {
            let no = rw_no_clone.read().unwrap();
            let mut first = first_clone.lock().unwrap();
            get_nos(&no, &mut first, pos, 0)
        });

        let rw_no_clone = Arc::clone(&rw_no);
        let b = thread::spawn(move || {
            let no = rw_no_clone.read().unwrap();
            let mut second = second_clone.lock().unwrap();
            get_nos(&no, &mut second, pos, 1)
        });
        let a = a.join().unwrap();
        let b = b.join().unwrap();

        let first = first.lock().unwrap();
        let second = second.lock().unwrap();

        

        //(*Arc::get_mut(&mut rw_no)).clear();
        let  no_list_temp = match Arc::get_mut(&mut rw_no){
            Some(e) => e,
            None => panic!("i dont know what happened"),
        };
        let mut no_list = no_list_temp.write().unwrap(); 

        let first = match a {
            0 => &[],
            a => first.chunks_exact(a).next().unwrap(),
        };
        let second = match b {
            0 => &[],
            a => second.chunks_exact(a).next().unwrap(),
        };

        no_list.clear();
        (no_list).extend_from_slice(&first);
        (no_list).extend_from_slice(&second);
    }
    dbg!(time.elapsed().as_millis());
    //dbg!(rw_no.read().unwrap());
}

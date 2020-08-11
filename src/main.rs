fn is_bit_set(number: u64, k: u64, bit: u64) -> bool {
    //stole from stackoverflow
    (number & ( 1 << k)) >> k == bit
}

fn get_nos(to_sort : &Vec::<u64> , writer : &mut Vec::<u64> , pos: usize, bit: u64) -> usize {
    writer.clear();
    let mut len = 0;
    for j in to_sort.iter(){
       if is_bit_set(*j, pos as u64, bit){
            writer.push(*j);
            len += 1;
       }
    }
    
    len
}

fn main() {
    let mut no : Vec<_> = {
        let mut string = String::new();
        std::io::stdin().read_line(&mut string);
        string.split_whitespace().map(|c| u64::from_str_radix(c, 10).unwrap()).collect()
    };

    let no_of_bits = std::mem::size_of::<u64>() * 8;

    let vec_size = no.len();
    let mut first = Vec::with_capacity(vec_size);
    let mut second = Vec::with_capacity(vec_size);
    for pos in 0..no_of_bits {
        let a = get_nos(&no, &mut first, pos, 0);
        let b = get_nos(&no, &mut second, pos, 1);

        no.clear();
        let first = match a {
            0 => &[],
            a => first.chunks_exact(a).next().unwrap(),
        };
        let second = match b {
            0 => &[],
            a => second.chunks_exact(a).next().unwrap(),
        };
        no.extend_from_slice(&first);    
        no.extend_from_slice(&second);    
    }

    dbg!(no);
}

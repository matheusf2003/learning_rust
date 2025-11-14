use std::io;

fn main() {
    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();
    let mut vec_sequencia = String::new();
    io::stdin().read_line(&mut vec_sequencia).unwrap();
    let vec_sequencia: Vec<i32> = vec_sequencia
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("{}", escadinha(&vec_sequencia, 0));
}

fn escadinha(sequencia: &Vec<i32>, indice: u32) -> u32 {
    if sequencia.len() - 1 == indice as usize {
        return 1;
    }
    let dif = sequencia[(indice + 1) as usize] - sequencia[indice as usize];
    let mut i = indice+1;

    loop {
        if sequencia.len() - 1 == i as usize {
            return 1;
        }
        if sequencia[(i+1) as usize] - sequencia[i as usize] != dif {
            return escadinha(sequencia, i) + 1;
        }
        i += 1;
    }
}

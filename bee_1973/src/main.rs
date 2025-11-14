use std::{clone::Clone, io};

fn main() {
    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();
    let mut vec_carneiros = String::new();
    io::stdin().read_line(&mut vec_carneiros).unwrap();
    let vec_carneiros: Vec<u32> = vec_carneiros
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    let total_carneiros: u64 = vec_carneiros.iter().map(|&x| x as u64).sum();
    let carneiros_roubados = rouba_carneiro(vec_carneiros.clone(), total_carneiros);
    let mut estrelas_roubadas: u32 = 0;
    for i in 0..vec_carneiros.len() {
        if vec_carneiros[i] != carneiros_roubados.1[i] {
            estrelas_roubadas += 1;
        }
    }
    println!("{} {}", estrelas_roubadas, carneiros_roubados.0);
}

fn rouba_carneiro(mut vec_carneiros: Vec<u32>, mut total_carneiros: u64) -> (u64, Vec<u32>) {
    let mut i: i32 = 0;
    loop {
        if i < 0 || i as usize >= vec_carneiros.len() {
            break;
        }
        if vec_carneiros[i as usize] == 0 {
            i -= 1;
        } else if vec_carneiros[i as usize] % 2 == 0 {
            vec_carneiros[i as usize] -= 1;
            total_carneiros -= 1;
            i -= 1;
        } else {
            vec_carneiros[i as usize] -= 1;
            total_carneiros -= 1;
            i += 1;
        }
    }
    (total_carneiros, vec_carneiros)
}
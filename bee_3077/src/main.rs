// A primeira linha da entrada contém um inteiro N (1 ≤ N ≤ 103) 
// seguido por um inteiro K (1 ≤ K ≤ 106), que são o número de 
// trabalhadores a quantidade de etapas, respectivamente. A próxima 
// linha contém os valores S1, S2, … , SN, onde 0 ≤ Si ≤ 103 para 1 ≤ i ≤ N.
use std::{io, collections::VecDeque};

fn main() {
    let (n, k) = {
        let mut infos = String::new();

        io::stdin().read_line(&mut infos).unwrap();
        let infos: Vec<usize> = infos.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
        (infos[0], infos[1])
    };
    let mut deque_pequis = String::new();
    io::stdin().read_line(&mut deque_pequis).unwrap();
    let mut deque_pequis: VecDeque<usize> = deque_pequis.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
    let mut pequis_trab: Vec<usize> = vec![0usize; n];

    let resto_k = k % n;
    let giros = k/n;
    for _ in 0..n as usize{
        for i in 0..n as usize{
            pequis_trab[i] += deque_pequis[i]*giros;
        }
        roda_deque(&mut deque_pequis);
    }

    for _ in 0..resto_k as usize{
        for i in 0..n as usize{
            pequis_trab[i] += deque_pequis[i];
        }
        roda_deque(&mut deque_pequis);
    }

    //println!("{}", pequis_trab.map(|n| n.to_string()).join(" "));
    let mut saida_str = String::new();
    for i in 0..n as usize{
        if i != 0{
            saida_str.push(' ');
        }
        saida_str.push_str(&pequis_trab[i].to_string());
    }
    println!("{}", saida_str);

}

fn roda_deque(deque:&mut VecDeque<usize>) {
    let card = deque.pop_back().unwrap();
    deque.push_front(card);
}

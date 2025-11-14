use std::io;

fn main() {
    // N é o número de mármores
    // Q é o número de consultas
    // consulta deque.front -> deque.back
    let mut case = 1u16;
    loop {
        let (n, q) = {
            let mut infos = String::new();
            io::stdin().read_line(&mut infos).unwrap();
            let infos: Vec<u16> = infos
                .split_whitespace()
                .map(|n| n.parse::<u16>().unwrap())
                .collect();
            match (infos[0], infos[1]) {
                (0, 0) => break,
                (n, q) => (n, q),
            }
        };
        let mut vec_marmore: Vec<u16> = Vec::new();
        for _ in 0..n {
            let mut marmore = String::new();
            io::stdin().read_line(&mut marmore).unwrap();
            vec_marmore.push(marmore.trim().parse().unwrap());
        }
        vec_marmore.sort();
        let mut vec_consultas: Vec<u16> = Vec::new();
        for _ in 0..q {
            let mut consulta = String::new();
            io::stdin().read_line(&mut consulta).unwrap();
            vec_consultas.push(consulta.trim().parse().unwrap());
        }
        let consultas = vec_consultas.into_iter();
        println!("CASE# {}:", case);
        case += 1;
        for marmore_analise in consultas {
            let mut n_consulta = 1u16;
            for marmore in vec_marmore.iter() {
                if marmore_analise == *marmore {
                    break;
                }
                n_consulta += 1;
            }
            if n_consulta == (vec_marmore.len() + 1) as u16 {
                println!("{} not found", marmore_analise);
            } else {
                println!("{0} found at {1}", marmore_analise, n_consulta);
            }
        }
    }
}

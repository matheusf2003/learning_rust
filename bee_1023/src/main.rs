use std::io;

struct Casa {
    pessoas: usize,
    consumo: usize,
    consumo_medio: usize,
}
struct Casas {
    vetor: Vec<Casa>
}

impl Casas {
    fn tirar_media(&self) -> f64 {
        let mut soma_consumo = 0usize;
        let mut soma_pessoas = 0usize;
        for c in 0..self.vetor.len() {
            soma_consumo += self.vetor[c].consumo;
            soma_pessoas += self.vetor[c].pessoas;
        }
        soma_consumo as f64 / soma_pessoas as f64
    }
    fn concatena_consumo(&mut self) {
        let mut comp_casa: &Casa;
        for c in (0..self.vetor.len()-1).rev() {
            comp_casa = &self.vetor[c+1];
            if comp_casa.consumo_medio == self.vetor[c].consumo_medio {
                self.vetor[c].pessoas += comp_casa.pessoas;
                self.vetor.remove(c+1);
            }
        }
    }
}
fn main() {
    let mut cidade = 1usize;
    loop {
        let mut instancias = String::new();
        let instancias: usize = match io::stdin().read_line(&mut instancias) {
            Ok(_) => instancias.trim().parse().unwrap(),
            _ => 0,
        };
        if instancias == 0 {
            break;
        }
        let mut casas: Casas = Casas { vetor: Vec::new() };
        for _n_instancia in 1..=instancias {
            let (x, y): (usize, usize) = {
                let mut entrada = String::new();
                io::stdin().read_line(&mut entrada).unwrap();
                let mut entrada = entrada.split_whitespace().map(|n| n.parse::<usize>().unwrap());
                (entrada.next().unwrap(), entrada.next().unwrap())
            };
            casas.vetor.push(Casa { pessoas: x, consumo: y, consumo_medio: y/x});
        }
        casas.vetor.sort_by(|a, b| a.consumo_medio.cmp(&b.consumo_medio));
        let media = casas.tirar_media();
        casas.concatena_consumo();
        let mut saida = String::new();
        for c in 0..casas.vetor.len() {
            saida.push_str(&format!("{}-{} ", casas.vetor[c].pessoas, casas.vetor[c].consumo_medio));
        }
        if cidade != 1 {
            print!("\n");
        }
        println!("Cidade# {}:", cidade);
        cidade+=1;
        println!("{}", saida.trim());
        let media = format!("{:.5}", media);
        println!("Consumo medio: {} m3.", &media[..get_float_dot(&media)+3]);
    }

}

fn get_float_dot(s: &str) -> usize {
    let mut count = 0usize;
    for c in s.chars() {
        match c {
            '.' => return count,
            _ => count += 1,
        }
    }
    count
}
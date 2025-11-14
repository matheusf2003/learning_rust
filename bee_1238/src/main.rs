use std::{io};

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .unwrap();
    let n: usize = n.trim().parse().unwrap_or(0);

    for _ in 0..n {
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .unwrap();
        let (s1, s2) = separa_palavras(entrada.trim());
        let saida = combinar(s1, s2);
        println!("{}", saida);
    }
}

fn separa_palavras(s: &str) -> (&str, &str) {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[..i], &s[i+1..]);
        }
    }
    (&s[..], &s[..])
}

fn combinar(s1: &str, s2: &str) -> String {
    let mut resultado = String::new();

    let mut chars1 = s1.chars();
    let mut chars2 = s2.chars();

    loop {
        match (chars1.next(), chars2.next()) {
            (Some(c1), Some(c2)) => {
                resultado.push(c1);
                resultado.push(c2);
            }
            (Some(c1), None) => {
                resultado.push(c1);
                resultado.extend(chars1);
                break;
            }
            (None, Some(c2)) => {
                resultado.push(c2);
                resultado.extend(chars2);
                break;
            }
            (None, None) => break
        }
    }
    
    resultado
}
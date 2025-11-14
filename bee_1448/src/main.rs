use std::io;

fn read_to_char_line() -> io::Result<Vec<char>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.chars().collect())
}

enum Primeiro {
    Time1,
    Time2,
    Nenhum,
}

fn main() {
    let mut instancias = String::new();
    let instancias: usize = match io::stdin().read_line(&mut instancias) {
        Ok(_) => instancias.trim().parse().unwrap(),
        _ => 0,
    };
    for i in 1..=instancias {
        let referencia = read_to_char_line().unwrap();
        let time1 = read_to_char_line().unwrap();
        let time2 = read_to_char_line().unwrap();

        let mut p_time1 = 0usize;
        let mut p_time2 = 0usize;
        let mut desempate = Primeiro::Nenhum;
        let menor_len = {
            if referencia.len() < time1.len() {
                if referencia.len() < time2.len() {
                    referencia.len()
                } else {
                    time2.len()
                }
            } else {
                if time1.len() < time2.len() {
                    time1.len()
                } else {
                    time2.len()
                }
            }
        };
        for c in 0..menor_len {
            if referencia[c] == time1[c] && referencia[c] != time2[c] {
                p_time1 += 1;
                match desempate {
                    Primeiro::Nenhum => desempate = Primeiro::Time1,
                    _ => {}
                };
            } else if referencia[c] == time2[c] && referencia[c] != time1[c] {
                p_time2 += 1;
                match desempate {
                    Primeiro::Nenhum => desempate = Primeiro::Time2,
                    _ => {}
                };
            }
        }
        println!("Instancia {}", i);
        if p_time1 == p_time2 {
            match desempate {
                Primeiro::Nenhum => println!("empate"),
                Primeiro::Time1 => println!("time 1"),
                Primeiro::Time2 => println!("time 2"),
            };
        } else {
            println!("time {}", if p_time1 > p_time2 { 1 } else { 2 });
        }
        print!("\n");
    }
}

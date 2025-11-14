use std::io;

fn main() {
    loop {
        //  n -> número de palavras do conto de Machado
        //  l -> número máximo de linhas por página
        //  c -> número máximo de caracteres por linha. 
        let (_n, l, c) = {
            let mut infos = String::new();

            match io::stdin().read_line(&mut infos) {
                Ok(0) => break, // EOF
                Ok(_) => (),
                _ => break,
            };
            let infos: Vec<u16> = infos.split_whitespace().map(|n| n.parse::<u16>().unwrap()).collect();
            (infos[0], infos[1], infos[2])
        };
        let mut conto = String::new();
        io::stdin().read_line(&mut conto).unwrap();
        let mut caracteres_linha = 0u16;
        let mut n_linhas = 1u16;
        for palavra in conto.trim().split_whitespace() {
            if caracteres_linha + palavra.len() as u16 <= c {
                caracteres_linha += palavra.len() as u16 +1;
            } else {
                n_linhas += 1;
                caracteres_linha = palavra.len() as u16 +1;
            }
        }
        let n_pag = if n_linhas % l > 0 { n_linhas / l + 1 } else {n_linhas / l};
        println!("{}", n_pag);
    }
}

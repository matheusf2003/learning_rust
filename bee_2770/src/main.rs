use std::io;
fn main() {
    loop {
        let mut infos = String::new();
        match io::stdin().read_line(&mut infos) {
            Ok(0) => break, // EOF
            Ok(_) => (),
            _ => break,
        };
        let (x, y, m): (u32, u32, u32) = {
            let mut nums = infos
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap());
            let (x, y, m) = (
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            );
            (x.max(y), x.min(y), m)
        };
        for _ in 0..m {
            let mut dimen = String::new();
            io::stdin().read_line(&mut dimen).unwrap();
            let (x_i, y_i): (u32, u32) = {
                let mut nums = dimen
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap());
                let (x_i, y_i) = (nums.next().unwrap(), nums.next().unwrap());
                (x_i.max(y_i), x_i.min(y_i))
            };
            if (x_i <= x) && (y_i <= y) {
                println!("Sim");
            } else {
                println!("Nao");
            }
        }
    }
}

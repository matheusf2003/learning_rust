use std::io;

fn main() {
    loop {
        let mut palavra1 = String::new();
        let mut palavra2 = String::new();

        match io::stdin().read_line(&mut palavra1) {
            Ok(0) => break,
            Ok(_) => (),
            Err(_) => break,
        };
        io::stdin().read_line(&mut palavra2).unwrap();

        println!("{}", encontra_maior_substring(palavra1.trim_end_matches(&['\n', '\r'][..]), palavra2.trim_end_matches(&['\n', '\r'][..])));
    }
}

fn encontra_maior_substring(s1: &str, s2: &str) -> u8 {
    let bytes1 = s1.as_bytes();
    let bytes2 = s2.as_bytes();

    let n = bytes1.len();
    let m = bytes2.len();

    if n == 0 || m == 0 {
        return 0;
    }

    let mut prev = vec![0u16; m + 1];
    let mut curr = vec![0u16; m + 1];
    let mut max_len = 0u16;

    for i in 1..=n {
        for j in 1..=m {
            if bytes1[i - 1] == bytes2[j - 1] {
                curr[j] = prev[j - 1] + 1;
                if curr[j] > max_len {
                    max_len = curr[j];
                }
            } else {
                curr[j] = 0;
            }
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    max_len.min(u8::MAX as u16) as u8
}

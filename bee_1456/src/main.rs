use std::{io, vec};

struct Buffer<T> {
    vec: Vec<T>,
    ponteiro: usize,
}

impl Buffer<u8> {
    fn new(str_buffer: &mut String) -> Self {
        Self {
            vec: str_buffer.trim().bytes().collect(),
            ponteiro: 0,
        }
    }

    fn consome(&mut self) -> u8 {
        if self.ponteiro == self.vec.len() {
            return 0u8;
        }
        let byte = self.vec[self.ponteiro];
        self.ponteiro += 1;
        byte
    }
}

impl Buffer<char> {
    fn new(str_buffer: &mut String) -> Self {
        Self {
            vec: str_buffer.trim().chars().collect(),
            ponteiro: 0,
        }
    }

    fn consome(&mut self) -> Option<char> {
        if self.ponteiro == self.vec.len() {
            return None;
        }
        let char_c = self.vec[self.ponteiro];
        self.ponteiro += 1;
        Some(char_c)
    }
}

struct BrainFuck {
    fita: Vec<u8>,
    ponteiro: u16,
    pilha_inicio_loop: Vec<usize>,
    instrucoes: Buffer<char>,
    buffer_entrada: Buffer<u8>,
}

impl BrainFuck {
    fn exec_prox_instrucao(&mut self) -> Result<(), ()> {
        match self.instrucoes.consome() {
            Some('>') => self.incrementa_ponteiro(),
            Some('<') => self.decrementa_ponteiro(),
            Some('+') => self.incrementa_byte(),
            Some('-') => self.decrementa_byte(),
            Some('.') => self.print_byte_as_char(),
            Some(',') => self.le_buffer(),
            Some('[') => {
                if self.fita[self.ponteiro as usize] == 0 {
                    self.encontra_final_loop();
                } else {
                    self.pilha_inicio_loop.push(self.instrucoes.ponteiro);
                }
            }
            Some(']') => {
                if self.fita[self.ponteiro as usize] == 0 {
                    self.pilha_inicio_loop.pop();
                } else {
                    self.instrucoes.ponteiro = *self.pilha_inicio_loop.last().unwrap();
                }
            }
            Some(_) => return Ok(()),
            _ => return Err(()),
        };
        Ok(())
    }

    fn encontra_final_loop(&mut self) {
        let mut count: u16 = 0;
        loop {
            match self.instrucoes.consome() {
                Some('[') => {
                    count += 1;
                }
                Some(']') => {
                    if count == 0 {
                        break;
                    } else {
                        count -= 1;
                    }
                }
                Some(_) => {
                    continue;
                }
                _ => panic!("loop incompleto"),
            }
        }
    }

    fn incrementa_ponteiro(&mut self) {
        if self.ponteiro < 29999 {
            self.ponteiro += 1;
        } else {
            self.ponteiro = 0
        }
    }

    fn decrementa_ponteiro(&mut self) {
        if self.ponteiro > 0 {
            self.ponteiro -= 1;
        } else {
            self.ponteiro = 29999
        }
    }

    fn incrementa_byte(&mut self) {
        self.fita[self.ponteiro as usize] += 1;
    }

    fn decrementa_byte(&mut self) {
        self.fita[self.ponteiro as usize] -= 1;
    }

    fn print_byte_as_char(&self) {
        print!("{}", self.fita[self.ponteiro as usize] as char);
    }

    fn le_buffer(&mut self) {
        self.fita[self.ponteiro as usize] = self.buffer_entrada.consome();
    }
}

fn main() {
    let mut instancia = String::new();
    let instancia: usize = match io::stdin().read_line(&mut instancia) {
        Ok(_) => instancia.trim().parse().unwrap(),
        _ => 0,
    };
    for n_instancia in 1..=instancia {
        let _ = io::stdin().read_line(&mut String::new());
        let mut fita_entrada = String::new();
        io::stdin().read_line(&mut fita_entrada).unwrap();
        let mut instrucoes = String::new();
        io::stdin().read_line(&mut instrucoes).unwrap();
        let mut backtrace = BrainFuck {
            fita: vec![0; 30000],
            buffer_entrada: Buffer::<u8>::new(&mut fita_entrada),
            instrucoes: Buffer::<char>::new(&mut instrucoes),
            ponteiro: 0,
            pilha_inicio_loop: Vec::new(),
        };
        println!("Instancia {}", n_instancia);
        loop {
            match backtrace.exec_prox_instrucao() {
                Ok(_) => continue,
                Err(_) => break,
            };
        }
        print!("\n\n");
    }
}

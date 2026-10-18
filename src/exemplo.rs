use std::io::{self, Write};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum Expressao {
    Numero(i64),
    Negacao(Box<Expressao>),
    Adicao(Box<Expressao>, Box<Expressao>),
    Subtracao(Box<Expressao>, Box<Expressao>),
    Multiplicacao(Box<Expressao>, Box<Expressao>),
    Divisao(Box<Expressao>, Box<Expressao>),
    Resto(Box<Expressao>, Box<Expressao>),
}

use Expressao::*;

impl Expressao {
    fn avaliar(&self) -> Option<i64> {
        match self {
            Numero(x) => Some(*x),
            Negacao(e) => e.avaliar().map(|x| -x),
            Adicao(esq, dir) => Some(l.avaliar()?.checked_add(r.avaliar()?)?),
            Subtracao(esq, dir) => Some(l.avaliar()?.checked_sub(r.avaliar()?)?),
            Multiplicacao(esq, dir) => Some(l.avaliar()?.checked_mul(r.avaliar()?)?),
            Divisao(esq, dir) => {
                let divisor = r.avaliar()?;
                if divisor == 0 {
                    None
                } else {
                    Some(l.avaliar()?.checked_div(divisor)?)
                }
            }
            Resto(esq, dir) => {
                let divisor = r.avaliar()?;
                if divisor == 0 {
                    None
                } else {
                    Some(l.avaliar()?.checked_rem(divisor)?)
                }
            }
        }
    }

    fn imprimir(&self) {
        fn aux(e: &Expressao, outer_prec: u8, ta_dir: bool) {
            let (s, prec, assoc_esq) = match e {
                Numero(x) => {
                    print!("{}", x);
                    return;
                }
                Negacao(_) => ("-", 3, false),
                Multiplicacao(_, _) | Divisao(_, _) | Resto(_, _) => ("*", 2, true),
                Adicao(_, _) | Subtracao(_, _) => ("+", 1, true),
            };

            let (prec, precisa_par) = match e {
                Negacao(sub) => {
                    print!("-");
                    let sub_prec = Expressao::precedencia(sub);
                    if sub_prec < prec {
                        print!("(");
                        aux(sub, 0, false);
                        print!(")");
                    } else {
                        aux(sub, prec, false);
                    }
                    return;
                }
                Numero(_) => (3, false),
                _ => {
                    let this_prec = Expressao::precedencia(e);
                    let need = this_prec < outer_prec
                        || (this_prec == outer_prec && is_right && !assoc_left);
                    (this_prec, need)
                }
            };

            if need_parens {
                print!("(");
            }
            match e {
                Adicao(l, r) => {
                    aux(l, prec, false);
                    print!(" + ");
                    aux(r, prec, true);
                }
                Subtracao(l, r) => {
                    aux(l, prec, false);
                    print!(" - ");
                    aux(r, prec + 1, true);
                }
                Multiplicacao(l, r) => {
                    aux(l, prec, false);
                    print!(" * ");
                    aux(r, prec, true);
                }
                Divisao(l, r) => {
                    aux(l, prec, false);
                    print!(" / ");
                    aux(r, prec + 1, true);
                }
                Resto(l, r) => {
                    aux(l, prec, false);
                    print!(" % ");
                    aux(r, prec + 1, true);
                }
                _ => {}
            }
            if need_parens {
                print!(")");
            }
        }

        aux(self, 0, false);
        println!();
    }

    fn precedencia(e: &Expressao) -> u8 {
        match e {
            Numero(_) => 3,
            Negacao(_) => 3,
            Multiplicacao(_, _) | Divisao(_, _) | Resto(_, _) => 2,
            Adicao(_, _) | Subtracao(_, _) => 1,
        }
    }

    fn imprimir_arvore(&self) {
        fn aux(expr: &Expressao, prefixo: &str, is_tail: bool) {
            print!("{}", prefixo);
            if is_tail {
                print!("└── ");
            } else {
                print!("├── ");
            }
            match expr {
                Numero(x) => println!("{}", x),
                Negacao(x) => {
                    println!("- (Neg)");
                    aux(x, &format!("{}    ", prefixo), true);
                }
                Adicao(l, r) => {
                    println!("+");
                    aux(l, &format!("{}│   ", prefixo), false);
                    aux(r, &format!("{}    ", prefixo), true);
                }
                Subtracao(l, r) => {
                    println!("-");
                    aux(l, &format!("{}│   ", prefixo), false);
                    aux(r, &format!("{}    ", prefixo), true);
                }
                Multiplicacao(l, r) => {
                    println!("*");
                    aux(l, &format!("{}│   ", prefixo), false);
                    aux(r, &format!("{}    ", prefixo), true);
                }
                Divisao(l, r) => {
                    println!("/");
                    aux(l, &format!("{}│   ", prefixo), false);
                    aux(r, &format!("{}    ", prefixo), true);
                }
                Resto(l, r) => {
                    println!("%");
                    aux(l, &format!("{}│   ", prefixo), false);
                    aux(r, &format!("{}    ", prefixo), true);
                }
            }
        }
        aux(self, "", true);
    }
}

// Parser simples: expressão totalmente parentizada ou simples (sem variáveis)
fn parse_expressao(s: &str) -> Result<Expressao, String> {
    let mut parser = Parser::new(s);
    let res = parser.parse_expr();
    match res {
        Ok(expr) => {
            parser.consume_whitespace();
            if parser.next().is_some() {
                Err("Entrada inválida após o fim da expressão".to_string())
            } else {
                Ok(expr)
            }
        }
        Err(e) => Err(e),
    }
}

/// Parser recursivo descent para expressões aritméticas
struct Parser<'a> {
    input: Peekable<Chars<'a>>,
}
impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
        Parser { input: s.chars().peekable() }
    }

    fn parse_expr(&mut self) -> Result<Expressao, String> {
        self.parse_add_sub()
    }
    fn parse_add_sub(&mut self) -> Result<Expressao, String> {
        let mut node = self.parse_mul_div_rem()?;

        loop {
            self.consume_whitespace();
            if self.peek() == Some(&'+') {
                self.next();
                let rhs = self.parse_mul_div_rem()?;
                node = Adicao(Box::new(node), Box::new(rhs));
            } else if self.peek() == Some(&'-') {
                self.next();
                let rhs = self.parse_mul_div_rem()?;
                node = Subtracao(Box::new(node), Box::new(rhs));
            } else {
                break;
            }
        }
        Ok(node)
    }
    fn parse_mul_div_rem(&mut self) -> Result<Expressao, String> {
        let mut node = self.parse_unary()?;

        loop {
            self.consume_whitespace();
            match self.peek() {
                Some(&'*') => {
                    self.next();
                    let rhs = self.parse_unary()?;
                    node = Multiplicacao(Box::new(node), Box::new(rhs));
                }
                Some(&'/') => {
                    self.next();
                    let rhs = self.parse_unary()?;
                    node = Divisao(Box::new(node), Box::new(rhs));
                }
                Some(&'%') => {
                    self.next();
                    let rhs = self.parse_unary()?;
                    node = Resto(Box::new(node), Box::new(rhs));
                }
                _ => break,
            }
        }
        Ok(node)
    }
    fn parse_unary(&mut self) -> Result<Expressao, String> {
        self.consume_whitespace();
        match self.peek() {
            Some(&'-') => {
                self.next();
                Ok(Negacao(Box::new(self.parse_unary()?)))
            }
            _ => self.parse_primary(),
        }
    }
    fn parse_primary(&mut self) -> Result<Expressao, String> {
        self.consume_whitespace();
        match self.peek() {
            Some(&'(') => {
                self.next();
                let expr = self.parse_expr()?;
                self.consume_whitespace();
                if self.next() == Some(')') {
                    Ok(expr)
                } else {
                    Err("Esperado ')'".to_string())
                }
            }
            Some(c) if c.is_digit(10) => self.parse_number(),
            _ => Err(format!("Token inesperado: {:?}", self.peek())),
        }
    }
    fn parse_number(&mut self) -> Result<Expressao, String> {
        self.consume_whitespace();
        let mut num = String::new();
        while let Some(&c) = self.peek() {
            if c.is_digit(10) {
                num.push(c);
                self.next();
            } else {
                break;
            }
        }
        if num.is_empty() {
            Err("Esperado número".to_string())
        } else {
            num.parse::<i64>()
                .map(Numero)
                .map_err(|_| "Número inválido".to_string())
        }
    }
    fn consume_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }
    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }
    fn next(&mut self) -> Option<char> {
        self.input.next()
    }
}

fn main() {
    print!("Digite uma expressão: ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();

    match parse_expressao(buffer) {
        Ok(expr) => {
            println!("\nForma legível:");
            expr.imprimir();

            println!("\nÁrvore sintática:");
            expr.imprimir_arvore();

            match expr.avaliar() {
                Some(v) => println!("\nResultado: {}", v),
                None => println!("\nErro ao avaliar a expressão (por exemplo: divisão por zero ou overflow)."),
            }
        }
        Err(e) => {
            println!("Erro ao parsear: {}", e);
        }
    }
}
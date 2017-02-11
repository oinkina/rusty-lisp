use std::io;
use std::iter::Peekable;

// DEFINE DATA TYPE
enum AST {
    Num(i64),
    Word(String),
    Nil,
    Cons { head: Box<AST>, tail: Box<AST> }
}

fn parse_sexpr(iter : &mut Peekable<std::str::Chars>) -> AST {
    //TODO make iter_peek nicer
//    let iter_peek = match iter.peek() { // match on what next char would be; don't "consume" yet
//        None     => panic!(),
 //       Some(&x) => x, // peek returns Some(&x); let iter_peek = x
//    };

    match *(iter.peek().unwrap()){
        ')'       => panic!("invalid s-expr: unexpected ')'"),
        '('       => { iter.next(); parse_cons(iter) },
        '0'...'9' =>                parse_num(iter),
        _         =>                parse_word(iter),
    }
}

fn parse_num(iter : &mut Peekable<std::str::Chars>) -> AST {
    let mut decimal = String::new();
    while let Some(n@'0'...'9') = iter.next() {
        decimal.push(n);
    }

    AST::Num(decimal.parse::<i64>().unwrap()) // Num : i64 -> AST
}

fn parse_word(iter : &mut Peekable<std::str::Chars>) -> AST {
    let mut word = String::new();
    loop { // loops as long as there are chars and no break-ing char
        let s = match iter.next() {
            None       => break,
            Some(' ')  => break,
            Some('\n') => break,
            Some('\t') => break,
            Some('(')  => break,
            Some(')')  => break,
            Some(s@_)  => s
        };
        word.push(s);
    }

    AST::Word(word) // Word : String -> AST
}

fn parse_cons(iter : &mut Peekable<std::str::Chars>) -> AST {
    unimplemented!();
//    let iter_peek = match iter.peek() { // match on what next char would be; don't "consume" yet
//        None     => panic!(),
//        Some(&x) => x, // peek returns Some(&x); let iter_peek = x
//    };
//
//    match iter.peek().unwrap() {
//        None => panic!("unexpected end of input"), //TODO fill in other panic msgs
//        Some(')') => AST::Nil,
//        Some(head@_) => { parse_sexpr(iter); parse_cons(iter) },
//    }
}

fn print_ast(tree : AST) {
    unimplemented!();
}


fn main() {
    // INPUT: I -> String
    println!("What string do you want to parse?");
    let mut sexpr = String::new();
    io::stdin().read_line(&mut sexpr);

    // PARSE: String -> AST
    let sexpr = sexpr; // make sexpr immutable, now that we will only iter
                       // todo: turn it into a str; consider simpler iter
    let mut iter = sexpr.chars().peekable(); // create iter
    let tree : AST = parse_sexpr(&mut iter);

    // EVALUATE: AST -> AST
    //TODO

    // OUTPUT: AST -> String -> O
    print_ast(tree);
}

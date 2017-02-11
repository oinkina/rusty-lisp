use std::io;
use std::iter::Peekable;


// 0. DEFINE DATA TYPE
#[derive(Debug)]
enum AST {
    Num(i64),
    Word(String),
    Nil,
    Cons { head: Box<AST>, tail: Box<AST> }
}


// 2. PARSE FUNCTIONS

fn strip_ws(iter : &mut Peekable<std::str::Chars>) {
    loop {
        match iter.peek() {
            Some(&' ')  => iter.next(),
            Some(&'\n') => iter.next(),
            Some(&'\t') => iter.next(),
            _           => break,
        };
    };
}

fn parse_sexpr(iter : &mut Peekable<std::str::Chars>) -> AST {
    println!("parse_sexpr entry");
    strip_ws(iter);
    let result = match *(iter.peek().unwrap()){
        ')'       => panic!("invalid s-expr: unexpected ')'"),
        '('       => { iter.next(); parse_cons(iter) },
        '0'...'9' =>                parse_num(iter),
        _         =>                parse_word(iter),
    };
    println!("parse_sexpr exit: {:?}", result);
    result
}

fn parse_num(iter : &mut Peekable<std::str::Chars>) -> AST {
    let mut decimal = String::new();
    while let Some(&n @ '0'...'9') = iter.peek() {
        decimal.push(n);
        iter.next();
    };

    AST::Num(decimal.parse::<i64>().unwrap()) // Num : i64 -> AST
}

fn parse_word(iter : &mut Peekable<std::str::Chars>) -> AST {
    println!("parse_word entry");
    let mut word = String::new();
    loop { // loops as long as there are chars and no break-ing char
        let s = match iter.peek() {
            None        => break,
            Some(&' ')  => break,
            Some(&'\n') => break,
            Some(&'\t') => break,
            Some(&'(')  => break,
            Some(&')')  => break,
            Some(&s)    => s
        };
        word.push(s);
        iter.next();
    }

    println!("parse_word exit: {:?}",word);
    AST::Word(word) // Word : String -> AST
}

fn parse_cons(iter : &mut Peekable<std::str::Chars>) -> AST {
    strip_ws(iter);
    println!("parse_cons entry: {:?}",iter.peek());
    let result = match *(iter.peek().unwrap()) {
        ')' => {iter.next(); AST::Nil},
        _   => { let head =Box::new(parse_sexpr(iter));
                 let tail = Box::new(parse_cons(iter));
                 AST::Cons { head : head, tail : tail }},
    };
    println!("parse_cons exit: {:?}",result);
    result
}


// 3. EVALUATE FUNCTIONS
// TODO


// 4. FORMAT FUNCTIONS
fn format_ast (tree : AST) -> String { // Show
   // match against each case; recurse for every node in AST::Cons 
   match tree {
       AST::Nil => format!("[]"),
       AST::Num(n@_) => format!("{}", n),
       AST::Word(w@_) => format!("{}", w),
       AST::Cons{head:h@_, tail:t@_} => format!("( {0} : {1} )", format_ast(*h), format_ast(*t)),
       // note that Rust compiler accepts that all cases are considered
   }
}


// MAIN //
fn main() {
    // 1. INPUT: I -> String
    println!("Please input an S-Expression:");
    let mut sexpr = String::new();
    io::stdin().read_line(&mut sexpr)
        .expect("failed to read line");

    // 2. PARSE: String -> AST
    let sexpr = sexpr; // make sexpr immutable, now that we will only iter
                       // todo: turn it into a str; consider simpler iter
    let mut iter = sexpr.chars().peekable(); // create iter
    let tree : AST = parse_sexpr(&mut iter);

    // 3. EVALUATE: AST -> AST
    //TODO

    // 4. OUTPUT: AST -> String -> O
    println!("{}", format_ast(tree));
}

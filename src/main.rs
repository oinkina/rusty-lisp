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
            Some(&ws) => match ws {
                ' '|'\n'|'\t'  => iter.next(),
                _              => break,
            }, None   => break,
        }; };
}

fn parse_sexpr(iter : &mut Peekable<std::str::Chars>) -> AST {
    strip_ws(iter);
    match *(iter.peek().unwrap()){
        ')'       => panic!("invalid s-expr: unexpected ')'"),
        '('       => { iter.next(); parse_cons(iter) },
        '0'...'9' =>                parse_num(iter),
        _         =>                parse_word(iter),
    }
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
    let mut word = String::new();
    loop { // loops as long as there are chars and no break-ing char
        let s = match iter.peek() {
            None        => break,
            Some(&s)    => match s {
                ' '|'\n'|'\t'|'('|')'=> break,
                _                    => s,
            } };
        word.push(s);
        iter.next();
    }

    AST::Word(word) // Word : String -> AST
}

fn parse_cons(iter : &mut Peekable<std::str::Chars>) -> AST {
    strip_ws(iter);
    match *(iter.peek().unwrap()) {
        ')' => {iter.next(); AST::Nil},
        _   => { let head =Box::new(parse_sexpr(iter));
                 let tail = Box::new(parse_cons(iter));
                 AST::Cons { head : head, tail : tail }},
    }
}


// 3A. EVALUATE FUNCTIONS
fn eval_tree(tree : AST) -> AST {
    match eval_flatten_heads(tree) {
        AST::Cons{head:h, tail:t} => match *h {
            AST::Word(w) => {
                match w.as_ref() {
                    "+" => unimplemented!(), //TODO
                    _ => panic!("undefined function")
                }
            },
            _ => panic!("tried to apply a non-word"),
        },
        x => x
    }
}
fn eval_flatten_heads(tree : AST) -> AST {
    match tree {
        AST::Cons{head:h, tail:t} => AST::Cons{
            head: Box::new(eval_tree(*h)),
            tail: Box::new(eval_flatten_heads(*t))},
        _ => tree,
    }
}

// 3B. KEYWORD IMPLEMENTATIONS
// TODO


// 4. FORMAT FUNCTIONS
fn format_ast (tree : AST) -> String { // Show
   // match against each case; recurse for every node in AST::Cons 
   match tree {
       AST::Nil => format!("[]"),
       AST::Num(n) => format!("{}", n),
       AST::Word(w) => format!("{}", w),
       AST::Cons{head:h, tail:t} => format!("( {0} : {1} )", format_ast(*h), format_ast(*t)),
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
    let mut iter = sexpr.chars().peekable(); // create iter
    let tree : AST = parse_sexpr(&mut iter);

    // 3. EVALUATE: AST -> AST
    let tree = eval_tree(tree);

    // 4. OUTPUT: AST -> String -> O
    println!("{:?}", tree);
    println!("{}", format_ast(tree));
}

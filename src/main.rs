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
                    "+"|"plus" |"add" => AST::Num(eval_add(*t)),
                    "*"|"times"|"mul" => AST::Num(eval_mul(*t)),
                    "list"            => *t,
                    "first"           => eval_first(*t),
                    "tail"            => eval_tail(*t),
                    _                 => panic!("expected a keyword; got undefined function '{}'", w),
                }
            },
            err => panic!("expected word; got {:?}", err),
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
fn eval_add(tree : AST) -> i64 {
    match tree {
        AST::Nil                  => 0,
        AST::Cons{head:h, tail:t} => match *h {
            AST::Num(n) => n + eval_add(*t),
            x           => panic!("expected a number; got {:?}", x),},
        _                         => panic!("ill-formed expr"),
    }
}

fn eval_mul(tree : AST) -> i64 {
    match tree {
        AST::Nil                       => 1,
        AST::Cons {head : h, tail : t} => match *h {
            AST::Num(n) => n * eval_mul(*t),
            _           => panic!("isn't num"),
        },
        _                              => panic!("ill formed"),
    }
}

fn eval_first(tree : AST) -> AST {
    match tree {
        AST::Cons { head: h, tail: _ } => match *h {
            AST::Cons { head: h2, tail: _ } => *h2,
            _                              => panic!("not a list"),
        },
        _                             => panic!("not a list"),
    }
}
fn eval_tail(tree : AST) -> AST {
    match tree {
        AST::Cons { head: h, tail: _ } => match *h {
            AST::Cons { head: _, tail: t} => *t,
            _                             => panic!("not a list"),
        },
        _                              => panic!("not a list"),
    }
}



// 4. FORMAT FUNCTIONS
fn format_ast (tree : AST) -> String { // Show
   // match against each case; recurse for every node in AST::Cons 
   match tree {
       AST::Nil                  => format!("[]"),
       AST::Num(n)               => format!("{}", n),
       AST::Word(w)              => format!("{}", w),
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

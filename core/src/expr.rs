//! Measurement-model expressions: parsing, evaluation and exact symbolic
//! differentiation.
//!
//! Sensitivity coefficients c_i = ∂f/∂x_i [GUM 5.1.3] and the higher-order
//! derivatives needed for the GUM 5.1.2 note are obtained by symbolic
//! differentiation, not by finite differences, so they carry no truncation
//! error.
//!
//! Grammar (usual precedence, `^` right-associative and binding tighter than
//! unary minus, so `-x^2 = -(x^2)`):
//!
//! ```text
//! expr    := term (('+' | '-') term)*
//! term    := unary (('*' | '/') unary)*
//! unary   := ('+' | '-') unary | power
//! power   := primary (('^' | '**') unary)?
//! primary := number | name | name '(' expr (',' expr)* ')' | '(' expr ')'
//! ```
//!
//! Names may contain Unicode letters, digits and `_` (e.g. `ρ_a`, `δm_R`).
//! Functions: sqrt, exp, ln, log10, sin, cos, tan, asin, acos, atan, sinh,
//! cosh, tanh, abs, pow(x, y). The name `log` is rejected as ambiguous.
//! Constant: `pi` / `π`.

use crate::error::UncError;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Func {
    Sqrt,
    Exp,
    Ln,
    Log10,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Abs,
    /// Sign function; appears only as the derivative of `abs`.
    Sign,
}

impl Func {
    fn from_name(name: &str) -> Option<Func> {
        Some(match name {
            "sqrt" => Func::Sqrt,
            "exp" => Func::Exp,
            "ln" => Func::Ln,
            "log10" => Func::Log10,
            "sin" => Func::Sin,
            "cos" => Func::Cos,
            "tan" => Func::Tan,
            "asin" => Func::Asin,
            "acos" => Func::Acos,
            "atan" => Func::Atan,
            "sinh" => Func::Sinh,
            "cosh" => Func::Cosh,
            "tanh" => Func::Tanh,
            "abs" => Func::Abs,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            Func::Sqrt => "sqrt",
            Func::Exp => "exp",
            Func::Ln => "ln",
            Func::Log10 => "log10",
            Func::Sin => "sin",
            Func::Cos => "cos",
            Func::Tan => "tan",
            Func::Asin => "asin",
            Func::Acos => "acos",
            Func::Atan => "atan",
            Func::Sinh => "sinh",
            Func::Cosh => "cosh",
            Func::Tanh => "tanh",
            Func::Abs => "abs",
            Func::Sign => "sign",
        }
    }

    #[inline]
    pub fn apply(self, x: f64) -> f64 {
        match self {
            Func::Sqrt => x.sqrt(),
            Func::Exp => x.exp(),
            Func::Ln => x.ln(),
            Func::Log10 => x.log10(),
            Func::Sin => x.sin(),
            Func::Cos => x.cos(),
            Func::Tan => x.tan(),
            Func::Asin => x.asin(),
            Func::Acos => x.acos(),
            Func::Atan => x.atan(),
            Func::Sinh => x.sinh(),
            Func::Cosh => x.cosh(),
            Func::Tanh => x.tanh(),
            Func::Abs => x.abs(),
            Func::Sign => {
                if x > 0.0 {
                    1.0
                } else if x < 0.0 {
                    -1.0
                } else {
                    0.0
                }
            }
        }
    }
}

/// Expression tree. Variables are indices into the input-quantity list.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Var(usize),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Call(Func, Box<Expr>),
    /// Four-quadrant inverse tangent atan2(y, x) in (−π, π].
    Atan2(Box<Expr>, Box<Expr>),
}

// ---------------------------------------------------------------------------
// Simplifying constructors (keep derivative trees small).
// ---------------------------------------------------------------------------

fn num(v: f64) -> Expr {
    Expr::Num(v)
}

fn is_num(e: &Expr, v: f64) -> bool {
    matches!(e, Expr::Num(x) if *x == v)
}

fn neg(a: Expr) -> Expr {
    match a {
        Expr::Num(x) => num(-x),
        Expr::Neg(inner) => *inner,
        other => Expr::Neg(Box::new(other)),
    }
}

fn add(a: Expr, b: Expr) -> Expr {
    match (&a, &b) {
        (Expr::Num(x), Expr::Num(y)) => num(x + y),
        _ if is_num(&a, 0.0) => b,
        _ if is_num(&b, 0.0) => a,
        (_, Expr::Neg(inner)) => sub(a, (**inner).clone()),
        _ => Expr::Add(Box::new(a), Box::new(b)),
    }
}

fn sub(a: Expr, b: Expr) -> Expr {
    match (&a, &b) {
        (Expr::Num(x), Expr::Num(y)) => num(x - y),
        _ if is_num(&b, 0.0) => a,
        _ if is_num(&a, 0.0) => neg(b),
        (_, Expr::Neg(inner)) => add(a, (**inner).clone()),
        _ => Expr::Sub(Box::new(a), Box::new(b)),
    }
}

fn mul(a: Expr, b: Expr) -> Expr {
    match (&a, &b) {
        (Expr::Num(x), Expr::Num(y)) => num(x * y),
        _ if is_num(&a, 0.0) || is_num(&b, 0.0) => num(0.0),
        _ if is_num(&a, 1.0) => b,
        _ if is_num(&b, 1.0) => a,
        _ if is_num(&a, -1.0) => neg(b),
        _ if is_num(&b, -1.0) => neg(a),
        (Expr::Neg(x), Expr::Neg(y)) => mul((**x).clone(), (**y).clone()),
        (Expr::Neg(x), _) => neg(mul((**x).clone(), b)),
        (_, Expr::Neg(y)) => neg(mul(a, (**y).clone())),
        _ => Expr::Mul(Box::new(a), Box::new(b)),
    }
}

fn div(a: Expr, b: Expr) -> Expr {
    match (&a, &b) {
        (Expr::Num(x), Expr::Num(y)) if *y != 0.0 => num(x / y),
        _ if is_num(&a, 0.0) => num(0.0),
        _ if is_num(&b, 1.0) => a,
        (Expr::Neg(x), _) => neg(div((**x).clone(), b)),
        _ => Expr::Div(Box::new(a), Box::new(b)),
    }
}

fn pow(a: Expr, b: Expr) -> Expr {
    match (&a, &b) {
        (Expr::Num(x), Expr::Num(y)) => num(x.powf(*y)),
        _ if is_num(&b, 0.0) => num(1.0),
        _ if is_num(&b, 1.0) => a,
        _ => Expr::Pow(Box::new(a), Box::new(b)),
    }
}

fn call(f: Func, a: Expr) -> Expr {
    match a {
        Expr::Num(x) => num(f.apply(x)),
        other => Expr::Call(f, Box::new(other)),
    }
}

impl Expr {
    /// Evaluates the expression at the given input values.
    pub fn eval(&self, x: &[f64]) -> f64 {
        match self {
            Expr::Num(v) => *v,
            Expr::Var(i) => x[*i],
            Expr::Neg(a) => -a.eval(x),
            Expr::Add(a, b) => a.eval(x) + b.eval(x),
            Expr::Sub(a, b) => a.eval(x) - b.eval(x),
            Expr::Mul(a, b) => a.eval(x) * b.eval(x),
            Expr::Div(a, b) => a.eval(x) / b.eval(x),
            Expr::Pow(a, b) => power(a.eval(x), b.eval(x)),
            Expr::Call(f, a) => f.apply(a.eval(x)),
            Expr::Atan2(a, b) => a.eval(x).atan2(b.eval(x)),
        }
    }

    /// True if the expression depends on variable `var`.
    pub fn depends_on(&self, var: usize) -> bool {
        match self {
            Expr::Num(_) => false,
            Expr::Var(i) => *i == var,
            Expr::Neg(a) | Expr::Call(_, a) => a.depends_on(var),
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Pow(a, b)
            | Expr::Atan2(a, b) => a.depends_on(var) || b.depends_on(var),
        }
    }

    /// Exact partial derivative with respect to variable `var`.
    pub fn diff(&self, var: usize) -> Expr {
        if !self.depends_on(var) {
            return num(0.0);
        }
        match self {
            Expr::Num(_) => num(0.0),
            Expr::Var(i) => num(if *i == var { 1.0 } else { 0.0 }),
            Expr::Neg(a) => neg(a.diff(var)),
            Expr::Add(a, b) => add(a.diff(var), b.diff(var)),
            Expr::Sub(a, b) => sub(a.diff(var), b.diff(var)),
            Expr::Mul(a, b) => add(mul(a.diff(var), (**b).clone()), mul((**a).clone(), b.diff(var))),
            Expr::Div(a, b) => {
                // (a/b)' = a'/b − a b' / b²
                let da = a.diff(var);
                let db = b.diff(var);
                sub(div(da, (**b).clone()), div(mul((**a).clone(), db), pow((**b).clone(), num(2.0))))
            }
            Expr::Pow(a, b) => {
                let a_dep = a.depends_on(var);
                let b_dep = b.depends_on(var);
                if !b_dep {
                    // (a^b)' = b a^(b−1) a'
                    mul(mul((**b).clone(), pow((**a).clone(), sub((**b).clone(), num(1.0)))), a.diff(var))
                } else if !a_dep {
                    // (a^b)' = a^b ln(a) b'
                    mul(mul(self.clone(), call(Func::Ln, (**a).clone())), b.diff(var))
                } else {
                    // (a^b)' = a^b (b' ln a + b a'/a)
                    mul(
                        self.clone(),
                        add(
                            mul(b.diff(var), call(Func::Ln, (**a).clone())),
                            div(mul((**b).clone(), a.diff(var)), (**a).clone()),
                        ),
                    )
                }
            }
            Expr::Call(f, a) => {
                let da = a.diff(var);
                let a = (**a).clone();
                let outer = match f {
                    Func::Sqrt => div(num(0.5), call(Func::Sqrt, a)),
                    Func::Exp => call(Func::Exp, a),
                    Func::Ln => div(num(1.0), a),
                    Func::Log10 => div(num(1.0), mul(a, num(std::f64::consts::LN_10))),
                    Func::Sin => call(Func::Cos, a),
                    Func::Cos => neg(call(Func::Sin, a)),
                    Func::Tan => div(num(1.0), pow(call(Func::Cos, a), num(2.0))),
                    Func::Asin => div(num(1.0), call(Func::Sqrt, sub(num(1.0), pow(a, num(2.0))))),
                    Func::Acos => neg(div(num(1.0), call(Func::Sqrt, sub(num(1.0), pow(a, num(2.0)))))),
                    Func::Atan => div(num(1.0), add(num(1.0), pow(a, num(2.0)))),
                    Func::Sinh => call(Func::Cosh, a),
                    Func::Cosh => call(Func::Sinh, a),
                    Func::Tanh => sub(num(1.0), pow(call(Func::Tanh, a), num(2.0))),
                    Func::Abs => call(Func::Sign, a),
                    Func::Sign => num(0.0),
                };
                mul(outer, da)
            }
            Expr::Atan2(a, b) => {
                // d atan2(a, b) = (b a' − a b') / (a² + b²)
                let num_ = sub(mul((**b).clone(), a.diff(var)), mul((**a).clone(), b.diff(var)));
                div(num_, add(pow((**a).clone(), num(2.0)), pow((**b).clone(), num(2.0))))
            }
        }
    }

    /// Compiles the expression to a flat stack program for fast repeated
    /// evaluation (Monte Carlo).
    pub fn compile(&self) -> Program {
        let mut ops = Vec::new();
        self.emit(&mut ops);
        let mut depth = 0usize;
        let mut max_depth = 0usize;
        for op in &ops {
            match op {
                Op::Push(_) | Op::Load(_) => depth += 1,
                Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Pow | Op::Atan2 => depth -= 1,
                Op::Neg | Op::Call(_) | Op::PowI(_) => {}
            }
            max_depth = max_depth.max(depth);
        }
        Program { ops, max_depth }
    }

    fn emit(&self, ops: &mut Vec<Op>) {
        match self {
            Expr::Num(v) => ops.push(Op::Push(*v)),
            Expr::Var(i) => ops.push(Op::Load(*i)),
            Expr::Neg(a) => {
                a.emit(ops);
                ops.push(Op::Neg);
            }
            Expr::Add(a, b) => bin(ops, a, b, Op::Add),
            Expr::Sub(a, b) => bin(ops, a, b, Op::Sub),
            Expr::Mul(a, b) => bin(ops, a, b, Op::Mul),
            Expr::Div(a, b) => bin(ops, a, b, Op::Div),
            Expr::Pow(a, b) => {
                if let Expr::Num(e) = **b {
                    if e.fract() == 0.0 && e.abs() <= 64.0 {
                        a.emit(ops);
                        ops.push(Op::PowI(e as i32));
                        return;
                    }
                }
                bin(ops, a, b, Op::Pow)
            }
            Expr::Call(f, a) => {
                a.emit(ops);
                ops.push(Op::Call(*f));
            }
            Expr::Atan2(a, b) => bin(ops, a, b, Op::Atan2),
        }
    }

    /// Formats the expression using the given variable names.
    pub fn to_string_with(&self, names: &[String]) -> String {
        let mut s = String::new();
        self.write(&mut s, names, 0);
        s
    }

    fn write(&self, s: &mut String, names: &[String], parent: u8) {
        // Precedence: 1 additive, 2 multiplicative, 3 unary, 4 power, 5 atom.
        let prec = self.precedence();
        let paren = prec < parent;
        if paren {
            s.push('(');
        }
        match self {
            Expr::Num(v) => s.push_str(&format_number(*v)),
            Expr::Var(i) => s.push_str(&names[*i]),
            Expr::Neg(a) => {
                s.push('-');
                a.write(s, names, 3);
            }
            Expr::Add(a, b) => {
                a.write(s, names, 1);
                s.push_str(" + ");
                b.write(s, names, 2);
            }
            Expr::Sub(a, b) => {
                a.write(s, names, 1);
                s.push_str(" - ");
                b.write(s, names, 2);
            }
            Expr::Mul(a, b) => {
                a.write(s, names, 2);
                s.push('*');
                b.write(s, names, 3);
            }
            Expr::Div(a, b) => {
                a.write(s, names, 2);
                s.push('/');
                b.write(s, names, 3);
            }
            Expr::Pow(a, b) => {
                a.write(s, names, 5);
                s.push('^');
                b.write(s, names, 3);
            }
            Expr::Call(f, a) => {
                s.push_str(f.name());
                s.push('(');
                a.write(s, names, 0);
                s.push(')');
            }
            Expr::Atan2(a, b) => {
                s.push_str("atan2(");
                a.write(s, names, 0);
                s.push_str(", ");
                b.write(s, names, 0);
                s.push(')');
            }
        }
        if paren {
            s.push(')');
        }
    }

    fn precedence(&self) -> u8 {
        match self {
            Expr::Add(..) | Expr::Sub(..) => 1,
            Expr::Mul(..) | Expr::Div(..) => 2,
            Expr::Neg(_) => 3,
            Expr::Num(v) if *v < 0.0 => 3,
            Expr::Pow(..) => 4,
            Expr::Num(_) | Expr::Var(_) | Expr::Call(..) | Expr::Atan2(..) => 5,
        }
    }
}

fn format_number(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{}", v as i64)
    } else {
        format!("{v}")
    }
}

fn bin(ops: &mut Vec<Op>, a: &Expr, b: &Expr, op: Op) {
    a.emit(ops);
    b.emit(ops);
    ops.push(op);
}

#[inline]
fn power(a: f64, b: f64) -> f64 {
    if b.fract() == 0.0 && b.abs() <= 64.0 {
        a.powi(b as i32)
    } else {
        a.powf(b)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    Push(f64),
    Load(usize),
    Neg,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    PowI(i32),
    Call(Func),
    Atan2,
}

/// Compiled expression (postfix program).
#[derive(Clone, Debug)]
pub struct Program {
    ops: Vec<Op>,
    max_depth: usize,
}

impl Program {
    /// Size of the evaluation stack needed by `eval_with`.
    pub fn stack_size(&self) -> usize {
        self.max_depth
    }

    /// Evaluates the program; `stack` is caller-provided scratch space.
    #[inline]
    pub fn eval_with(&self, x: &[f64], stack: &mut Vec<f64>) -> f64 {
        stack.clear();
        for op in &self.ops {
            match *op {
                Op::Push(v) => stack.push(v),
                Op::Load(i) => stack.push(x[i]),
                Op::Neg => {
                    let a = stack.last_mut().unwrap();
                    *a = -*a;
                }
                Op::PowI(n) => {
                    let a = stack.last_mut().unwrap();
                    *a = a.powi(n);
                }
                Op::Call(f) => {
                    let a = stack.last_mut().unwrap();
                    *a = f.apply(*a);
                }
                Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Pow | Op::Atan2 => {
                    let b = stack.pop().unwrap();
                    let a = stack.last_mut().unwrap();
                    *a = match *op {
                        Op::Add => *a + b,
                        Op::Sub => *a - b,
                        Op::Mul => *a * b,
                        Op::Div => *a / b,
                        Op::Atan2 => a.atan2(b),
                        _ => power(*a, b),
                    };
                }
            }
        }
        stack[0]
    }

    pub fn eval(&self, x: &[f64]) -> f64 {
        let mut stack = Vec::with_capacity(self.max_depth);
        self.eval_with(x, &mut stack)
    }
}

// ---------------------------------------------------------------------------
// Lexer and parser.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
enum Tok {
    Num(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
}

fn lex(src: &str) -> Result<Vec<(Tok, usize)>, UncError> {
    let chars: Vec<char> = src.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    let err = |message: &str, position: usize| UncError::Parse { message: message.to_string(), position };
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        let start = i;
        if c.is_ascii_digit() || (c == '.' && chars.get(i + 1).is_some_and(|d| d.is_ascii_digit())) {
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
                let mut j = i + 1;
                if j < chars.len() && (chars[j] == '+' || chars[j] == '-') {
                    j += 1;
                }
                if j < chars.len() && chars[j].is_ascii_digit() {
                    i = j;
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        i += 1;
                    }
                }
            }
            let text: String = chars[start..i].iter().collect();
            let v: f64 = text.parse().map_err(|_| err(&format!("invalid number '{text}'"), start))?;
            out.push((Tok::Num(v), start));
            continue;
        }
        if c.is_alphabetic() || c == '_' {
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            out.push((Tok::Ident(chars[start..i].iter().collect()), start));
            continue;
        }
        let tok = match c {
            '+' => Tok::Plus,
            '-' | '−' => Tok::Minus,
            '*' | '×' | '·' | '⋅' => {
                if c == '*' && chars.get(i + 1) == Some(&'*') {
                    i += 1;
                    Tok::Caret
                } else {
                    Tok::Star
                }
            }
            '/' | '÷' => Tok::Slash,
            '^' => Tok::Caret,
            '(' => Tok::LParen,
            ')' => Tok::RParen,
            ',' => Tok::Comma,
            _ => return Err(err(&format!("unexpected character '{c}'"), start)),
        };
        out.push((tok, start));
        i += 1;
    }
    Ok(out)
}

/// Resolves names during parsing.
pub trait SymbolTable {
    /// Returns the expression a name stands for (a variable or an inlined
    /// intermediate quantity), or `None` if the name is unknown.
    fn resolve(&self, name: &str) -> Option<Expr>;
}

impl<F: Fn(&str) -> Option<Expr>> SymbolTable for F {
    fn resolve(&self, name: &str) -> Option<Expr> {
        self(name)
    }
}

struct Parser<'a, S: SymbolTable> {
    toks: Vec<(Tok, usize)>,
    pos: usize,
    end: usize,
    symbols: &'a S,
}

/// Parses `src`, resolving names through `symbols`.
pub fn parse<S: SymbolTable>(src: &str, symbols: &S) -> Result<Expr, UncError> {
    let toks = lex(src)?;
    let mut p = Parser { toks, pos: 0, end: src.chars().count(), symbols };
    if p.toks.is_empty() {
        return Err(UncError::Parse { message: "empty expression".into(), position: 0 });
    }
    let e = p.expr()?;
    if p.pos < p.toks.len() {
        return Err(UncError::Parse { message: "unexpected token".into(), position: p.toks[p.pos].1 });
    }
    Ok(e)
}

impl<S: SymbolTable> Parser<'_, S> {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos).map(|t| &t.0)
    }

    fn position(&self) -> usize {
        self.toks.get(self.pos).map(|t| t.1).unwrap_or(self.end)
    }

    fn expect(&mut self, t: Tok, what: &str) -> Result<(), UncError> {
        if self.peek() == Some(&t) {
            self.pos += 1;
            Ok(())
        } else {
            Err(UncError::Parse { message: format!("expected {what}"), position: self.position() })
        }
    }

    fn expr(&mut self) -> Result<Expr, UncError> {
        let mut lhs = self.term()?;
        loop {
            match self.peek() {
                Some(Tok::Plus) => {
                    self.pos += 1;
                    lhs = Expr::Add(Box::new(lhs), Box::new(self.term()?));
                }
                Some(Tok::Minus) => {
                    self.pos += 1;
                    lhs = Expr::Sub(Box::new(lhs), Box::new(self.term()?));
                }
                _ => return Ok(lhs),
            }
        }
    }

    fn term(&mut self) -> Result<Expr, UncError> {
        let mut lhs = self.unary()?;
        loop {
            match self.peek() {
                Some(Tok::Star) => {
                    self.pos += 1;
                    lhs = Expr::Mul(Box::new(lhs), Box::new(self.unary()?));
                }
                Some(Tok::Slash) => {
                    self.pos += 1;
                    lhs = Expr::Div(Box::new(lhs), Box::new(self.unary()?));
                }
                _ => return Ok(lhs),
            }
        }
    }

    fn unary(&mut self) -> Result<Expr, UncError> {
        match self.peek() {
            Some(Tok::Minus) => {
                self.pos += 1;
                Ok(Expr::Neg(Box::new(self.unary()?)))
            }
            Some(Tok::Plus) => {
                self.pos += 1;
                self.unary()
            }
            _ => self.power(),
        }
    }

    fn power(&mut self) -> Result<Expr, UncError> {
        let base = self.primary()?;
        if self.peek() == Some(&Tok::Caret) {
            self.pos += 1;
            let exp = self.unary()?;
            return Ok(Expr::Pow(Box::new(base), Box::new(exp)));
        }
        Ok(base)
    }

    fn primary(&mut self) -> Result<Expr, UncError> {
        let position = self.position();
        match self.peek().cloned() {
            Some(Tok::Num(v)) => {
                self.pos += 1;
                Ok(Expr::Num(v))
            }
            Some(Tok::LParen) => {
                self.pos += 1;
                let e = self.expr()?;
                self.expect(Tok::RParen, "')'")?;
                Ok(e)
            }
            Some(Tok::Ident(name)) => {
                self.pos += 1;
                if self.peek() == Some(&Tok::LParen) {
                    self.pos += 1;
                    let mut args = vec![self.expr()?];
                    while self.peek() == Some(&Tok::Comma) {
                        self.pos += 1;
                        args.push(self.expr()?);
                    }
                    self.expect(Tok::RParen, "')'")?;
                    return self.function(&name, args, position);
                }
                if name == "pi" || name == "π" {
                    if let Some(e) = self.symbols.resolve(&name) {
                        return Ok(e);
                    }
                    return Ok(Expr::Num(std::f64::consts::PI));
                }
                self.symbols.resolve(&name).ok_or(UncError::UnknownSymbol(name))
            }
            _ => Err(UncError::Parse { message: "expected a number, name or '('".into(), position }),
        }
    }

    fn function(&self, name: &str, mut args: Vec<Expr>, position: usize) -> Result<Expr, UncError> {
        let arity_err =
            |n: usize| UncError::Parse { message: format!("function '{name}' takes {n} argument(s)"), position };
        if name == "pow" {
            if args.len() != 2 {
                return Err(arity_err(2));
            }
            let b = args.pop().unwrap();
            let a = args.pop().unwrap();
            return Ok(Expr::Pow(Box::new(a), Box::new(b)));
        }
        if name == "atan2" {
            if args.len() != 2 {
                return Err(arity_err(2));
            }
            let b = args.pop().unwrap();
            let a = args.pop().unwrap();
            return Ok(Expr::Atan2(Box::new(a), Box::new(b)));
        }
        if name == "log" {
            return Err(UncError::Parse { message: "'log' is ambiguous; use ln (natural) or log10".into(), position });
        }
        let f = Func::from_name(name)
            .ok_or_else(|| UncError::Parse { message: format!("unknown function '{name}'"), position })?;
        if args.len() != 1 {
            return Err(arity_err(1));
        }
        Ok(Expr::Call(f, Box::new(args.pop().unwrap())))
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max = max_var(self);
        let names: Vec<String> = (0..=max).map(|i| format!("x{i}")).collect();
        f.write_str(&self.to_string_with(&names))
    }
}

fn max_var(e: &Expr) -> usize {
    match e {
        Expr::Num(_) => 0,
        Expr::Var(i) => *i,
        Expr::Neg(a) | Expr::Call(_, a) => max_var(a),
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Pow(a, b) | Expr::Atan2(a, b) => {
            max_var(a).max(max_var(b))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars(names: &'static [&'static str]) -> impl Fn(&str) -> Option<Expr> {
        move |n: &str| names.iter().position(|v| *v == n).map(Expr::Var)
    }

    fn p(src: &str) -> Expr {
        parse(src, &vars(&["x", "y", "z", "ρ_a", "δm"])).unwrap()
    }

    #[test]
    fn precedence_and_associativity() {
        let x = [2.0, 3.0, 4.0, 1.2, 0.5];
        assert_eq!(p("1 + 2 * 3").eval(&x), 7.0);
        assert_eq!(p("(1 + 2) * 3").eval(&x), 9.0);
        assert_eq!(p("2 ^ 3 ^ 2").eval(&x), 512.0);
        assert_eq!(p("-x^2").eval(&x), -4.0);
        assert_eq!(p("x ** -1").eval(&x), 0.5);
        assert_eq!(p("8 / 4 / 2").eval(&x), 1.0);
        assert_eq!(p("x - y - z").eval(&x), -5.0);
        assert_eq!(p("ρ_a × δm").eval(&x), 0.6);
        assert_eq!(p("1.5e-3 * 2E3").eval(&x), 3.0);
        assert!((p("pi").eval(&x) - std::f64::consts::PI).abs() < 1e-15);
        assert_eq!(p("pow(x, 3)").eval(&x), 8.0);
        assert_eq!(p("x − y").eval(&x), -1.0);
    }

    #[test]
    fn parse_errors() {
        let s = vars(&["x"]);
        assert!(matches!(parse("x +", &s), Err(UncError::Parse { .. })));
        assert!(matches!(parse("q * 2", &s), Err(UncError::UnknownSymbol(n)) if n == "q"));
        assert!(matches!(parse("log(x)", &s), Err(UncError::Parse { .. })));
        assert!(matches!(parse("sqrt(x, x)", &s), Err(UncError::Parse { .. })));
        assert!(matches!(parse("(x", &s), Err(UncError::Parse { .. })));
        assert!(matches!(parse("x $ 2", &s), Err(UncError::Parse { .. })));
        assert!(matches!(parse("", &s), Err(UncError::Parse { .. })));
    }

    /// Symbolic derivatives agree with central finite differences.
    #[test]
    fn derivatives_match_finite_differences() {
        let exprs = [
            "x*y/z",
            "1000*x*y/z",
            "x^3 - 2*x*y + y^2",
            "sqrt(x^2 + y^2)",
            "exp(-x*y) * ln(z)",
            "log10(x*z) + sin(y)*cos(x) + tan(x/10)",
            "asin(x/10) + acos(y/10) + atan(x*y)",
            "sinh(x/3) + cosh(y/3) + tanh(z/5)",
            "x^y + z^0.5",
            "abs(x - y) / (1 + z)",
            "(x + y*(1 + 2e-6*(z - 20))) / (1 + 1.1e-5*z)",
        ];
        let x0 = [2.3, 1.7, 3.1, 1.2, 0.5];
        for src in exprs {
            let e = p(src);
            for var in 0..3 {
                let d = e.diff(var).eval(&x0);
                let h = 1e-6 * x0[var].abs().max(1.0);
                let mut xp = x0;
                let mut xm = x0;
                xp[var] += h;
                xm[var] -= h;
                let fd = (e.eval(&xp) - e.eval(&xm)) / (2.0 * h);
                assert!((d - fd).abs() <= 1e-6 * fd.abs().max(1.0), "{src} d/dx{var}: {d} vs {fd}");
            }
        }
    }

    #[test]
    fn second_derivatives() {
        let e = p("x^2 * y^3");
        // ∂²/∂x∂y = 6 x y²
        let dxy = e.diff(0).diff(1);
        assert!((dxy.eval(&[2.0, 3.0, 0.0, 0.0, 0.0]) - 108.0).abs() < 1e-12);
        // ∂³/∂x∂y² = 12 x y
        let dxyy = e.diff(0).diff(1).diff(1);
        assert!((dxyy.eval(&[2.0, 3.0, 0.0, 0.0, 0.0]) - 72.0).abs() < 1e-12);
    }

    #[test]
    fn compiled_program_matches_tree() {
        let x0 = [2.3, 1.7, 3.1, 1.2, 0.5];
        for src in ["x*y/z - 3", "-(x^2) + sqrt(y)*exp(z/10)", "x^y - y^-2", "((x))", "2"] {
            let e = p(src);
            let prog = e.compile();
            assert_eq!(prog.eval(&x0), e.eval(&x0), "{src}");
            for var in 0..3 {
                let d = e.diff(var);
                assert_eq!(d.compile().eval(&x0), d.eval(&x0));
            }
        }
    }

    #[test]
    fn display_round_trips() {
        let names: Vec<String> = ["x", "y", "z", "ρ_a", "δm"].iter().map(|s| s.to_string()).collect();
        let x0 = [2.3, 1.7, 3.1, 1.2, 0.5];
        for src in ["x - (y - z)", "x / (y * z)", "-x^2", "(-x)^2", "x^(y^z)", "(x^y)^z", "-(x + y)", "2 - -x"] {
            let e = p(src);
            let shown = e.to_string_with(&names);
            let again = p(&shown);
            assert_eq!(again.eval(&x0), e.eval(&x0), "{src} -> {shown}");
        }
    }

    #[test]
    fn atan2_four_quadrants_and_derivative() {
        let e = p("atan2(y, x)");
        for (x, y) in [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0), (-2.0, 0.0)] {
            let v = [x, y, 0.0, 0.0, 0.0];
            assert_eq!(e.eval(&v), f64::atan2(y, x));
            assert_eq!(e.compile().eval(&v), f64::atan2(y, x));
            // ∂/∂x = −y/(x² + y²), ∂/∂y = x/(x² + y²)
            assert!((e.diff(0).eval(&v) + y / (x * x + y * y)).abs() < 1e-15);
            assert!((e.diff(1).eval(&v) - x / (x * x + y * y)).abs() < 1e-15);
        }
        let names: Vec<String> = ["x", "y", "z", "a", "b"].iter().map(|s| s.to_string()).collect();
        assert_eq!(e.to_string_with(&names), "atan2(y, x)");
        assert!(parse("atan2(x)", &|_: &str| Some(Expr::Var(0))).is_err());
    }
}

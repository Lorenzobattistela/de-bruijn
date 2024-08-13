// The `to_de_bruijn` function converts a lambda calculus term with named variables (represented by the `Term` enum) into its corresponding De Bruijn index representation (represented by the `DeBruijnTerm` enum). Here's how it works:

// 1. The function takes two parameters:
//    - `term`: The input term to be converted
//    - `context`: A list of variable names, representing the current scope

// 2. The function uses pattern matching to handle different cases of the input term:

//    a. For a Lambda abstraction (`Term::Lam`):
//       - It creates a new context by inserting the lambda's variable name at the beginning of the current context.
//       - It recursively calls `to_de_bruijn` on the lambda's body with the new context.
//       - It wraps the result in a `DeBruijnTerm::Lam`.

//    b. For an Application (`Term::App`):
//       - It recursively calls `to_de_bruijn` on both the function and argument parts.
//       - It combines the results into a `DeBruijnTerm::App`.

//    c. For a Variable (`Term::Var`):
//       - It searches for the variable name in the context.
//       - If found, it returns a `DeBruijnTerm::Var` with the index of the variable in the context.
//       - If not found, it returns a `DeBruijnTerm::Var` with an index equal to the context length (representing an unbound variable).

// 3. The context is used to keep track of bound variables:
//    - Each time a new lambda abstraction is encountered, its variable is added to the front of the context.
//    - When a variable is processed, its De Bruijn index is determined by its position in the context.

// 4. The De Bruijn index for a bound variable is calculated as the distance from the variable to its binding lambda, counting from 0.
//    For unbound variables, the index is the number of enclosing lambdas (context length) plus one.

// This approach handles both bound and unbound variables, replacing them with numeric indices. Bound variables are represented by their distance to the binding lambda, while unbound variables are represented by indices greater than the number of enclosing lambdas.


use TSPL::Parser;
use std::fmt;

enum Term {
    Lam { name: String, body: Box<Term> },
    App { func: Box<Term>, argm: Box<Term> },
    Var { name: String },
}

#[derive(Clone)]
enum DeBruijnTerm {
    Lam(Box<DeBruijnTerm>),
    App(Box<DeBruijnTerm>, Box<DeBruijnTerm>),
    Var(usize),
}

TSPL::new_parser!(TermParser);

impl<'i> TermParser<'i> {
    fn parse(&mut self) -> Result<Term, String> {
        self.skip_trivia();
        match self.peek_one() {
            Some('λ') => {
                self.consume("λ")?;
                let name = self.parse_name()?;
                let body = Box::new(self.parse()?);
                Ok(Term::Lam { name, body })
            }
            Some('(') => {
                self.consume("(")?;
                let func = Box::new(self.parse()?);
                let argm = Box::new(self.parse()?);
                self.consume(")")?;
                Ok(Term::App { func, argm })
            }
            _ => {
                let name = self.parse_name()?;
                Ok(Term::Var { name })
            }
        }
    }
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Lam { name, body } => write!(f, "λ{} {:?}", name, body),
            Term::App { func, argm } => write!(f, "({:?} {:?})", func, argm),
            Term::Var { name } => write!(f, "{}", name),
        }
    }
}

impl fmt::Debug for DeBruijnTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeBruijnTerm::Lam(body) => write!(f, "λ{:?}", body),
            DeBruijnTerm::App(func, argm) => write!(f, "({:?} {:?})", func, argm),
            DeBruijnTerm::Var(index) => write!(f, "{}", index),
        }
    }
}

fn to_de_bruijn(term: &Term, context: &[String]) -> DeBruijnTerm {
    match term {
        Term::Lam { name, body } => {
            let mut new_context = context.to_vec();
            new_context.insert(0, name.clone());
            DeBruijnTerm::Lam(Box::new(to_de_bruijn(body, &new_context)))
        }
        Term::App { func, argm } => DeBruijnTerm::App(
            Box::new(to_de_bruijn(func, context)),
            Box::new(to_de_bruijn(argm, context)),
        ),
        Term::Var { name } => {
            let index = context.iter().position(|x| x == name)
                .unwrap_or_else(|| context.len());
            DeBruijnTerm::Var(index)
        }
    }
}

fn main() {
    //let mut parser = TermParser::new("(λx x y)");
    match parser.parse() {
        Ok(term) => {
            println!("Parsed: {:?}", term);
            let de_bruijn = to_de_bruijn(&term, &[]);
            println!("De Bruijn: {:?}", de_bruijn);
        }
        Err(err) => eprintln!("{}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_and_convert(input: &str) -> String {
        let mut parser = TermParser::new(input);
        let term = parser.parse().unwrap();
        let de_bruijn = to_de_bruijn(&term, &[]);
        format!("{:?}", de_bruijn)
    }

    #[test]
    fn test_de_bruijn_conversion() {
        assert_eq!(parse_and_convert("λx x"), "λ0");
        //assert_eq!(parse_and_convert("((λx x) y)"), "λ0");
        assert_eq!(parse_and_convert("λx λy x"), "λλ1");
        assert_eq!(parse_and_convert("λx λy y"), "λλ0");
        assert_eq!(parse_and_convert("λx λy λz (x (y z))"), "λλλ(2 (1 0))");
        assert_eq!(parse_and_convert("λf λx (f (f x))"), "λλ(1 (1 0))");
        assert_eq!(parse_and_convert("λx λy λz ((x z) (y z))"), "λλλ((2 0) (1 0))");
        assert_eq!(parse_and_convert("λf λg λx (f (g x))"), "λλλ(2 (1 0))");
        assert_eq!(parse_and_convert("λx λy λz λw ((x y) (z w))"), "λλλλ((3 2) (1 0))");
        assert_eq!(parse_and_convert("λx z"), "λ1");
        assert_eq!(parse_and_convert("λx λy (z x)"), "λλ(2 1)");
    }
}

// Implementation of beta reduction for De Bruijn terms

impl DeBruijnTerm {
    // Beta reduction function
    fn beta_reduce(&self) -> DeBruijnTerm {
        match self {
            // For lambda abstractions, we reduce the body
            DeBruijnTerm::Lam(body) => DeBruijnTerm::Lam(Box::new(body.beta_reduce())),
            
            // For applications, we check if it's a redex (reducible expression)
            DeBruijnTerm::App(func, arg) => {
                let reduced_func = func.beta_reduce();
                let reduced_arg = arg.beta_reduce();
                
                // If the function is a lambda, we can perform beta reduction
                if let DeBruijnTerm::Lam(body) = reduced_func {
                    // Substitute the argument into the body
                    body.substitute(0, &reduced_arg).beta_reduce()
                } else {
                    // If it's not a lambda, we just return the reduced application
                    DeBruijnTerm::App(Box::new(reduced_func), Box::new(reduced_arg))
                }
            }
            
            // Variables remain unchanged
            DeBruijnTerm::Var(index) => DeBruijnTerm::Var(*index),
        }
    }

    // Helper function to substitute a term for a variable
    fn substitute(&self, index: usize, replacement: &DeBruijnTerm) -> DeBruijnTerm {
        match self {
            DeBruijnTerm::Var(i) => {
                if *i == index {
                    // If the variable matches the index, replace it
                    replacement.clone()
                } else if *i > index {
                    // If the variable is "higher", decrease its index
                    DeBruijnTerm::Var(*i - 1)
                } else {
                    // Otherwise, keep it as is
                    self.clone()
                }
            }
            DeBruijnTerm::Lam(body) => {
                // For lambdas, we need to increase the index and substitute in the body
                DeBruijnTerm::Lam(Box::new(body.substitute(index + 1, &replacement.shift(1))))
            }
            DeBruijnTerm::App(func, arg) => {
                // For applications, substitute in both parts
                DeBruijnTerm::App(
                    Box::new(func.substitute(index, replacement)),
                    Box::new(arg.substitute(index, replacement)),
                )
            }
        }
    }

    // Helper function to shift De Bruijn indices
    fn shift(&self, amount: isize) -> DeBruijnTerm {
        self.shift_above(amount, 0)
    }

    // Helper function to shift De Bruijn indices above a certain cutoff
    fn shift_above(&self, amount: isize, cutoff: usize) -> DeBruijnTerm {
        match self {
            DeBruijnTerm::Var(i) => {
                if *i >= cutoff {
                    DeBruijnTerm::Var(((*i as isize) + amount) as usize)
                } else {
                    self.clone()
                }
            }
            DeBruijnTerm::Lam(body) => {
                DeBruijnTerm::Lam(Box::new(body.shift_above(amount, cutoff + 1)))
            }
            DeBruijnTerm::App(func, arg) => {
                DeBruijnTerm::App(
                    Box::new(func.shift_above(amount, cutoff)),
                    Box::new(arg.shift_above(amount, cutoff)),
                )
            }
        }
    }
}

// New test cases for beta reduction
#[cfg(test)]
mod beta_reduction_tests {
    use super::*;

    fn parse_convert_reduce(input: &str) -> String {
        let mut parser = TermParser::new(input);
        let term = parser.parse().unwrap();
        let de_bruijn = to_de_bruijn(&term, &[]);
        let reduced = de_bruijn.beta_reduce();
        format!("{:?}", reduced)
    }

    #[test]
    fn test_beta_reduction() {
        assert_eq!(parse_convert_reduce("(λx x y)"), "1");
        //assert_eq!(parse_convert_reduce("(λx λy x)"), "λλ1");
        //assert_eq!(parse_convert_reduce("((λx λy x) z)"), "λ1");
        //assert_eq!(parse_convert_reduce("((λx λy y) z)"), "λ0");
        //assert_eq!(parse_convert_reduce("(λf λx (f (f x)))"), "λλ(1 (1 0))");
        //assert_eq!(parse_convert_reduce("((λf λx (f (f x))) λy y)"), "λ(λ0 (λ0 0))");
    }
}

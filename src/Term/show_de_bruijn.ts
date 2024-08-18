import { Term } from "./_";

// Pretty prints a lambda calculus term using de Bruijn notation
// - term: the Term to be printed
// = a string representation of the term in de Bruijn notation
export function show_de_bruijn(term: Term): string {
  const show_term = (t: Term, parent_precedence: number): string => {
    switch (t.$) {
      case "Var": {
        return t.index.toString();
      }
      case "Lam": {
        var body = show_term(t.body, 0);
        return parent_precedence > 0 ? `(λ${body})` : `λ${body}`;
      }
      case "App": {
        var func = show_term(t.func, 1);
        var arg  = show_term(t.arg, 2);
        return parent_precedence > 1 ? `(${func} ${arg})` : `${func} ${arg}`;
      }
    }
  };

  return show_term(term, 0);
}

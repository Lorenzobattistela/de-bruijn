import { Term } from "../Term/_";

// Performs one step of beta reduction on a lambda calculus term
// - term: the Term to be reduced
// = the reduced Term if a reduction was possible, or null if the term is already in normal form
export function reduce(term: Term): Term | null {
  switch (term.$) {
    case "App": {
      // First, try to reduce the function part
      var reduced_func = reduce(term.func);
      if (reduced_func) {
        return { $: "App", func: reduced_func, arg: term.arg };
      }
      
      // If function is a value (lambda), try to reduce the argument
      if (term.func.$ === "Lam") {
        var reduced_arg = reduce(term.arg);
        if (reduced_arg) {
          return { $: "App", func: term.func, arg: reduced_arg };
        }
        
        // If both function is a lambda and argument is a value, perform beta reduction
        if (is_value(term.arg)) {
          return substitute(term.func.body, 0, shift(term.arg, 1, 0));
        }
      }
      
      return null;
    }
    case "Lam": {
      // Do not reduce under lambdas
      return null;
    }
    case "Var": {
      // Variables are already in normal form
      return null;
    }
  }
}

// Shifts the de Bruijn indices in a term
// - term: the term to shift
// - by: the amount to shift by
// - from: the cutoff index
// = the shifted term
function shift(term: Term, by: number, from: number): Term {
  switch (term.$) {
    case "Var": {
      return {
        $: "Var",
        index: term.index < from ? term.index : term.index + by
      };
    }
    case "Lam": {
      return { $: "Lam", body: shift(term.body, by, from + 1) };
    }
    case "App": {
      return {
        $: "App",
        func: shift(term.func, by, from),
        arg: shift(term.arg, by, from)
      };
    }
  }
}

// Substitutes a term for a variable in another term
// - term: the term to perform substitution in
// - index: the de Bruijn index to substitute for
// - replacement: the term to substitute
// = the term after substitution
function substitute(term: Term, index: number, replacement: Term): Term {
  switch (term.$) {
    case "Var": {
      if (term.index === index) {
        return replacement;
      } else if (term.index > index) {
        return { $: "Var", index: term.index - 1 };
      } else {
        return term;
      }
    }
    case "Lam": {
      return {
        $: "Lam",
        body: substitute(term.body, index + 1, shift(replacement, 1, 0))
      };
    }
    case "App": {
      return {
        $: "App",
        func: substitute(term.func, index, replacement),
        arg: substitute(term.arg, index, replacement)
      };
    }
  }
}

// Checks if a term is a value (i.e., it cannot be reduced further)
// - term: the term to check
// = true if the term is a value, false otherwise
function is_value(term: Term): boolean {
  return term.$ === "Lam" || term.$ === "Var";
}

import { Parser } from "./_";
import { Term } from "../Term/_";

// A parser for lambda terms using de Bruijn indices
// - input: the string to be parsed
// = the parsed Term if successful, or null if parsing fails
export const parse: Parser = (input: string): Term | null => {
  let index = 0;
  let bound_vars: string[] = [];

  // Helper function to parse a single term
  const parse_term = (): Term | null => {
    skip_whitespace();
    
    if (index >= input.length) {
      return null;
    }

    let term = parse_atom();
    if (!term) return null;

    // Parse applications
    while (index < input.length) {
      skip_whitespace();
      if (index >= input.length || input[index] === ')') break;
      const arg = parse_atom();
      if (!arg) break;
      term = { $: "App", func: term, arg };
    }

    return term;
  };

  // Helper function to parse an atom (variable, lambda, or parenthesized term)
  const parse_atom = (): Term | null => {
    skip_whitespace();
    
    if (index >= input.length) {
      return null;
    }

    switch (input[index]) {
      case 'λ': {
        // Lambda abstraction
        index++; // Skip 'λ'
        skip_whitespace();
        if (!is_variable(input[index])) return null;
        const var_name = input[index];
        index++; // Skip variable
        bound_vars.push(var_name);
        const body = parse_term();
        bound_vars.pop();
        return body ? { $: "Lam", body } : null;
      }
      case '(': {
        // Parenthesized expression
        index++; // Skip '('
        const term = parse_term();
        if (!term) return null;
        skip_whitespace();
        if (input[index] !== ')') return null;
        index++; // Skip ')'
        return term;
      }
      default: {
        // Variable
        if (is_variable(input[index])) {
          const var_name = input[index];
          const var_index = bound_vars.lastIndexOf(var_name);
          if (var_index === -1) return null; // Unbound variable
          index++;
          return { $: "Var", index: bound_vars.length - var_index - 1 };
        }
        return null;
      }
    }
  };

  // Helper function to skip whitespace
  const skip_whitespace = (): void => {
    while (index < input.length && /\s/.test(input[index])) {
      index++;
    }
  };

  // Helper function to check if a character is a valid variable name
  const is_variable = (char: string): boolean => {
    return /[a-z]/.test(char);
  };

  // Parse the entire input
  const result = parse_term();
  skip_whitespace();

  // Ensure we've consumed all input
  return index === input.length ? result : null;
};

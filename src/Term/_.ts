// Represents a lambda calculus term using de Bruijn indices
// - Var: a variable, represented by its de Bruijn index
// - Lam: a lambda abstraction
// - App: an application of one term to another

export type Term
  = { $: "Var", index: number }
  | { $: "Lam", body: Term }
  | { $: "App", func: Term, arg: Term };

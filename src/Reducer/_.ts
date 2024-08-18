import { Term } from "../Term/_";

// Represents a reducer function that takes a Term and performs one step of reduction
// - term: the Term to be reduced
// = the reduced Term if a reduction was possible, or null if the term is already in normal form
export type Reducer = (term: Term) => Term | null;

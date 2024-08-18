import { Term } from "../Term/_";

// Represents a parser function that takes a string input
// and returns a Term if successful, or null if parsing fails
// - input: the string to be parsed
// = the parsed Term if successful, or null if parsing fails
export type Parser = (input: string) => Term | null;

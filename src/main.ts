import { parse } from "./Parser/parse";
import { show_de_bruijn } from "./Term/show_de_bruijn";
import { reduce } from "./Reducer/reduce";
import { Term } from "./Term/_";

// Tests the parser and reducer with various lambda calculus terms
// and displays the results
function main() {
  const test_cases = [
    "(λa a) λb b", // normal form: λ0
    "(λa λb b a)", // normal form: λλ0 1
    "((λa λb b a) λc c)", // λb b λc c -> λ0 λ0 
  ];

  for (const input of test_cases) {
    console.log("Input:", input);
    const parsed = parse(input);
    if (parsed) {
      console.log("De Bruijn notation:", show_de_bruijn(parsed));
      console.log("Reduction steps:");
      let term: Term | null = parsed;
      let step = 1;
      while (term) {
        console.log(`  Step ${step}:`, show_de_bruijn(term));
        const reduced = reduce(term);
        if (reduced === null) {
          console.log("  Normal form reached.");
          break;
        }
        term = reduced;
        step++;
      }
    } else {
      console.log("Parsing failed");
    }
    console.log("---");
  }
}

// Run the main function
main();

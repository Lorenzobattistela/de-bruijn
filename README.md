# De Bruijn Representation and Reduction for Lambda Terms

This project is a simple implementation aimed at learning and exploring De Bruijn representation and reduction for lambda terms. De Bruijn notation is a way of representing lambda terms that eliminates the need for explicit variable names, making it easier to perform operations like substitution and alpha-conversion.

## What are De Bruijn Indices?

In lambda calculus, De Bruijn indices are a notation for lambda terms in which variables are replaced by natural numbers. These numbers represent the binding depth of the variable. Specifically:

- The index n refers to the variable bound by the nth lambda abstraction, counting from the innermost lambda.
- Free variables are represented by indices greater than the number of enclosing lambda abstractions.

For example, the lambda term λx.λy.x y would be represented as λλ.1 0 in De Bruijn notation.

## How Does the Reduction Algorithm Work?

The reduction algorithm for De Bruijn terms, also known as beta-reduction, works as follows:

1. Identify a redex (reducible expression) of the form (λ.M) N, where M and N are De Bruijn terms.
2. Substitute N for the bound variable in M, which is represented by the index 0.
3. During substitution, adjust the indices:
   - Decrement all free variables in N by 1 (as they move under one less lambda).
   - Increment all free variables in M that are greater than or equal to the substitution depth.

This process continues until no more redexes can be reduced.

Note that our current project restricts free variables for simplicity.

## Project Goals

1. Implement De Bruijn representation for lambda terms
2. Develop reduction algorithms for De Bruijn terms

## Features

- Representation of lambda terms using De Bruijn indices
- Beta-reduction of De Bruijn terms

## Getting Started

To get started with this project, clone the repository, compile and run:

```
git clone https://github.com/yourusername/de-bruijn.git
cd de-bruijn
tsc
npm start
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

## Resources

For those interested in learning more about De Bruijn representation and lambda calculus:

- [De Bruijn notation](https://en.wikipedia.org/wiki/De_Bruijn_index)
- [Lambda calculus](https://en.wikipedia.org/wiki/Lambda_calculus)

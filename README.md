# Slo
A simple functional oriented language under development with a syntax inspired by Haskell and Rust. Do NOT expect a release soon.

# THIS LANGUAGE IS UNDER DEVELPMENT, IT IS NOT COMPLETE.

## Example Idea

```slo
fib : Int -> Int
fib(0) = 0;
fib(1) = 1;
fib(n) = fib(n - 1) + fib(n - 2);

main : () -> ()
main() = {
    let result = fib(7);
    print("7th Fibonacci number: ", result );
}
```

## Roadmap
x - Done
I - In progress
[ ] - Not started

- [x] Lexer
- [ ] Parser
- [ ] Type checker
- [ ] Interpreter
- [ ] Compiler - LLVM

## Examples

You can find some examples/concept ideas in the [examples](examples) directory.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.
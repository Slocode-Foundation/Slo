# Slo
A simple functional oriented language under development with a syntax inspired by Haskell!

# THIS LANGUAGE IS UNDER DEVELPMENT, IT IS NOT COMPLETE.

## Example

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

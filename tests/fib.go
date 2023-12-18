type Test;

impl Test {
    fn fib(n: int) -> int {
        if (n < 2) {
            return n;
        } else {
            return self.fib(n - 1) + self.fib(n - 2);
        }

    }
}

let test = Test();
let start = clock();
println test.fib(35);
let end = clock();
println (end - start);


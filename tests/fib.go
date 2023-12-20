type Test {
    x: int,
    y: int
}

impl Test {
    fn new(x:int, y:int) {
        self.x = x;
        self.y = y;
    }
    fn fib(n: int) -> int {
        if (n < 2) {
            return n;
        } else {
            return self.fib(n - 1) + self.fib(n - 2);
        }

    }
}

let test = Test(1, 2);
let start = clock();
println test.fib(35);
let end = clock();
println (end - start);

println test.x;
println test.y;


export interface Example {
  key?: string;
  name: string;
  code: string;
}

export const examples: Example[] = [
  {
    key: "1",
    name: "Hello World",
    code: `println "Hello, World!";`,
  },
  {
    key: "2",
    name: "Fibonacci",
    code: `type test {
    x: int,
    y: int
}

impl test {
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

let test = test(1, 2);
let start = clock();
println test.fib(33);
let end = clock();
println (end - start);

println test.x;
println test.y;
`,
  },
  {
    key: "3",
    name: "Inheritance",
    code: `type Doughnut;

type Cruller;

impl Doughnut {
    fn new() {
    }

    fn do(s: String) {
        println "Do something with doughnut";
    }

    fn finish(ingredient: String) {
        println "Finish with " + ingredient;
    }

    fn cook(s: String) {
        println "Dunk in the fryer.";
        self.do("fry");
    }

}

impl Cruller < Doughnut {
    fn new() {
    }

    fn do(s: String) {
        println "Do something with cruller";
    }

    fn finish(ingredient: String) {
        super.finish("icing");
    }

    fn cook(s: String) {
        super.cook("S");
    }

}

let crul = Cruller();
crul.finish("S");
crul.cook("S");
`,
  },
];

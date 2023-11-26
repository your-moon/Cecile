let a = "a";
let b = "b";
let c = a + b;
println c;
println "";

let a = 1;
let b = 2;
let c = a + b;
println c;
println "";

fn fizzbuzz(n: int) {
    if (n % 15 == 0) {
        println "FizzBuzz";
    } else if (n % 5 == 0) {
        println "Fizz";
    } else if (n % 3 == 0) {
        println "Buzz";
    } else {
        println n;
    }
}

let sum = 0;

for (let i = 0; i < 20; i = i + 1) {
    fizzbuzz(i);
    sum = sum + i;
}
print "SUM OF 100: ";
println sum;

fn add_two_number(a:int, b:int) -> int {
    return a + b;
}

println add_two_number(5, 15);
println "Hello, World";


let n = [];
let n2 = 2;
n.extend([n2]);
println n;

type Point {
  x:int,
  y:int
}

impl Point {
  fn new(x:int, y:int) {
    self.x = x;
    self.y = y;
  }
}

fn distance(a: Point, b: Point) -> int {
  let x = (b.x - a.x);
  let y = (b.y - a.y);
  let d = x*x + y*y;
  return d;
}

let a = Point(3, 2);
let b = Point(5, 6);

println distance(a, b);

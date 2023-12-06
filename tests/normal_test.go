
// Variable declaration
let a = "a";
let b = "b";
// Concatenation
let c = a + b; 
// Print
println c; // out: ab
println ""; // out:

// Variable declaration
let a = 1;
let b = 2;

// Binary operation
let c = a + b;

// Print
println c; // out: 3
println ""; // out:


// Function declaration
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

fn add_two_number(a:int, b:int) -> int {
    return a + b;
}

println add_two_number(5, 15);

let sum = 0;


// For loop
for (let i = 0; i < 20; i = i + 1) {
    // fizzbuzz(i);
    sum = sum + i;
}

// Array declaration
let n = [];
let n2 = 2;

// Extending array
n.extend([n2]);
n.push(5);
println n;


// Declaration of structure
type Point {
  x:int,
  y:int
}

// Object methods
impl Point {
  fn new(x:int, y:int) {
    self.x = x;
    self.y = y;
  }
}

// Getting object as parameter, accessing object fields
fn distance(a: Point, b: Point) -> int {
  let x = (b.x - a.x);
  let y = (b.y - a.y);
  let d = x*x + y*y;
  return d;
}

let a = Point(3, 2);
let b = Point(5, 6);

println distance(a, b);

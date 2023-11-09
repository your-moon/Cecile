// let a = "a";
// let b = "b";
// let c = a + b;
// println c;
// println "";
//
// let a = 1;
// let b = 2;
// let c = a + b;
// println c;
// println "";
//
// fn fizzbuzz(n: int) {
//     if (n % 15 == 0) {
//         println "FizzBuzz";
//     } else if (n % 5 == 0) {
//         println "Fizz";
//     } else if (n % 3 == 0) {
//         println "Buzz";
//     } else {
//         println n;
//     }
// }
//
// let sum = 0;
//
// for (let i = 0; i < 20; i = i + 1) {
//     fizzbuzz(i);
//     sum = sum + i;
// }
// print "SUM OF 100: ";
// println sum;
//
// fn add_two_number(a:int, b:int) -> int {
//     return a + b;
// }
//
// println add_two_number(5, 15);
// println "Hello, World";

type Rectangle {
    width: int,
    height: int
}

impl Rectangle {
    fn new(width: int, height: int) {
        self.width = width;
        self.height = height;
    }

    fn area() -> int {
        return self.width * self.height;
    }
}

let rect = Rectangle(2, 2);
let sum = 0;
for (let i = 0; i < 10; i = i + 1) {
    sum = sum + rect.area();
}

println sum;


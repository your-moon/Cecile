type Rectangle {
    width: int,
    height: int
}

impl Rectangle {
    fn new(width: int, height: int) {
        self.width = width;
        self.height = height;
    }

    fn area(s: int) -> int {
        return self.width * self.height + s;
    }
}

// let rect = Rectangle(2, 2);
// println rect.area(1);


type Shape {
  width: int
}
// impl Shape;
let shape = Shape();
println shape.width;

// let rect = Rectangle(2, 2);
// let sum = 0;
// for (let i = 0; i < 10; i = i + 1) {
//     sum = sum + rect.area(i);
// }
//
// println sum;
//


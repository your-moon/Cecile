type Point {
    x: int,
    y: int

}

impl Point {
    fn new(x: int, y: int) {
        self.x = x;
        self.y = y;
    }
}

let a: Point = nil; 
println a.x;

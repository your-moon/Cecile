type Pair {
    first: String,
    second: String
}

impl Pair {
    fn new(a: String, b: String) {
        self.first = a;
        self.second = b;
    }

    fn sum() -> String {
        return self.first + self.second;
    }
}

type Point {
    x: int,
    y: int,
    name: String
}

impl Point {
    fn new(x: int, y: int, name: String) {
        self.x = x;
        self.y = y;
        self.name = name;
    }
}

let pair = Pair("Wow", " Cool");
let point = Point(1, 2, "Point");

println pair.first + point.name;

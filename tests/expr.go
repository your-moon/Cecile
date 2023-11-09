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
    name: String,
    pair: Pair
}

impl Point {
    fn new(x: int, y: int, name: String, pair: Pair) {
        self.x = x;
        self.y = y;
        self.name = name;
        self.pair = pair;
    }

    fn area() -> int {
        return self.x * self.y;
    }
}

let pair = Pair("Wow", " Cool");
let point = Point(10, 20, "Point", pair);

println point.area();
println point.pair.sum();

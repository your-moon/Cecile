type Pair {
    first: String,
    second: String
}

impl Pair {
    fn new() {
    }

    fn sum() -> String {
        return self.first + self.second;
    }
}

let a = Pair();
a.first = "Wow";
a.second = " Cool";
println a.sum();

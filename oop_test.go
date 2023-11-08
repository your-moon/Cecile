type Rectangle {
    x:int,
    y:int
}


impl Rectangle {
    fn new(x:int, y:int) {
        self.x = x;
        self.y = y;
    }
fn say_something(s: String) -> String {
        return "Hello";
    }
    
}

let rectangle = Rectangle(11, 25);

println rectangle.x;
println rectangle.y;
println rectangle.say_something("s");


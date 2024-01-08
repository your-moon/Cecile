type Point {
    x: int,
    y: int
}

type Ops {
    name: String
}

impl Ops {
    fn new(name: String) {
        self.name = name;
    }
}


let point = Point();
point.hi();
point.do();

impl Point {
    fn do() {
        let op = Ops("op name");
        println "do";
        println op.name;
    }
}


impl Point {
    fn hi() {
        println "hi";
    }
}


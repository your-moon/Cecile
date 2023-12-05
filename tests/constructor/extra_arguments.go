type Foo {
    a: int,
    b: int
}

impl Foo {
  fn new(a: int, b: int) {
    self.a = a;
    self.b = b;
  }
}

let foo = Foo(1, 2, 3, 4); // out: TypeError: init() takes 2 arguments but 4 were given

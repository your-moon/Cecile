type Foo {
    field: String
}

impl Foo {
  fn new(arg) {
    print "Foo.init(" + arg + ")";
    self.field = "init";
  }
}

fn new() {
  print "not initializer";
}

new(); // out: not initializer

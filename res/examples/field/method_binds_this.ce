type Foo {
    name: String,
    func: fn
}
impl Foo {
  fn sayName(a: String) {
    println self.name;
    println a;
  }
}

let foo1 = Foo();
foo1.name = "foo1";

let foo2 = Foo();
foo2.name = "foo2";

// Store the method reference on another object.
foo2.func = foo1.sayName;
// Still retains original receiver.
foo2.func(1);
// out: foo1
// out: 1

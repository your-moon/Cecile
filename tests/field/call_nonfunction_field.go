type Foo {
  bar: String
}

let foo = Foo();
foo.bar = "not fn";

foo.bar(); // out: TypeError: "string" object is not callable

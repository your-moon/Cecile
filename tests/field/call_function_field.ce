type Foo {
  bar: fn
}

fn bar(a, b) {
  println "bar";
  println a;
  println b;
}

let foo = Foo();
foo.bar = bar;

foo.bar(1, 2);
// out: bar
// out: 1
// out: 2

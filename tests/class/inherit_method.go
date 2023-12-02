type Foo;
impl Foo {
  fn inFoo() {
    println "in foo";
  }
}
type Bar;
impl Bar < Foo {
  fn inBar() {
    println "in bar";
  }
}

type Baz;
impl Baz < Bar {
  fn inBaz() {
    println "in baz";
  }
}

let baz = Baz();
baz.inFoo(); // out: in foo
baz.inBar(); // out: in bar
baz.inBaz(); // out: in baz


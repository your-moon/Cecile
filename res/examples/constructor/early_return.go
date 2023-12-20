type Foo;
impl Foo {
  fn new() {
    println "init";
    return;
    println "nope";
  }
}

let foo = Foo(); // out: init
println foo; // out: <object Foo>

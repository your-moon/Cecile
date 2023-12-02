{
  let foo = "closure";
  fn f() {
    {
      println foo; // out: closure
      let foo = "shadow";
      println foo; // out: shadow
    }
    println foo; // out: closure
  }
  f();
}

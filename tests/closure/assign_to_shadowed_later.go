let a = "global";

{
  fn assign() {
    a = "assigned";
  }

  let a = "inner";
  assign();
  println a; // out: inner
}

println a; // out: assigned

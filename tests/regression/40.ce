fn caller(g: fn) {
  g();
  // g should be a function, not nil.
  print g == nil; // out: false
}

fn callCaller() {
  let capturedVar = "before";
  let a = "a";

  fn f() {
    // Commenting the next line out prevents the bug!
    capturedVar = "after";

    // Returning anything also fixes it, even nil:
    //return nil;
  }

  caller(f);
}

callCaller();

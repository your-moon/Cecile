fn f(a:int, b:int) {
  print a;
  print b;
}

// out: TypeError: f() takes 2 arguments but 4 were given
f(1, 2, 3, 4);

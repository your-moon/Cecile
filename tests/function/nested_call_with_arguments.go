fn returnArg(arg: fn) -> fn -> String {
  return arg;
}

fn returnFunCallWithArg(func: fn, arg: String) -> String {
    // returnArg(func) => type: fn -> int
  println returnArg(func)(arg);
}

//
fn printArg(arg: String) -> String {
    return arg;
}
//
returnFunCallWithArg(printArg, "hello world"); // out: hello world

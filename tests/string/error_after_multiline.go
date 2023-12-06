// Tests that we correctly track the line info across multiline strings.
let a = "1
2
3
";

err; // out: NameError: name "err" is not defined

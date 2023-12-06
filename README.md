
---

# Cecile Программчлалын хэл

## Танилцуулга

Welcome to Cecile, a modern programming language designed for simplicity and flexibility. Cecile combines ease of use with powerful functionality to make programming a breeze.

### Syntax жишээнүүд

#### Хувьсагч зарлах болон string нийлүүлэх
```rust
// Хувьсагч зарлах
let a = "a";
let b = "b";

// String-үүдийг нийлүүлэх
let c = a + b;

// Хэвлэх
println c; // Гаралт: ab
println ""; // Гаралт:
```

#### Mathematical Operations and Functions
```rust
// Variable declaration
let a = 1;
let b = 2;

// Binary operation
let c = a + b;

// Print
println c; // Output: 3
println ""; // Output:

// Function declaration
fn fizzbuzz(n: int) {
    // Function body
}

fn add_two_number(a:int, b:int) -> int {
    // Function body
}

println add_two_number(5, 15);
```

#### Control Flow and Loops
```rust
// For loop
for (let i = 0; i < 20; i = i + 1) {
    // Loop body
}

// Conditional statements
if (condition) {
    // Statement
} else if (another_condition) {
    // Statement
} else {
    // Statement
}
```

#### Data Structures and Objects
```rust
// Array declaration
let n = [];
n.extend([2]);
n.push(5);
println n;

// Declaration of structure
type Point {
    x: int,
    y: int
}

// Object methods
impl Point {
    fn new(x: int, y: int) {
        // Method body
    }
}

// Object instantiation and usage
let a = Point(3, 2);
let b = Point(5, 6);

println distance(a, b);
```

#### Type Annotations and Redefinition
```rust
let dogs: String = "dog";
let dogs: int = 1;
let dogs: Vec<int> = [];
```

### Features Highlight

- **Simplicity**: Concise syntax and clear semantics for easy understanding.
- **Functions**: Support for defining and using functions with parameters and return types.
- **Control Flow**: Conditional statements (if-else) and looping constructs (for loops).
- **Data Structures**: Define and manipulate arrays and custom structures.
- **Type System**: Support for type annotations and type redefinition.
- **Object-Oriented**: Implementation of methods for custom structures.

---

Feel free to explore Cecile and its features further! For installation instructions and detailed documentation, refer to the official documentation at [NewLang Documentation](https://your-newlang-docs.com).

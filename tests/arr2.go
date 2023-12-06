type Stack {
    items: Vec<int>
}

impl Stack {
    fn new() {
        self.items = [];
    }
    fn push(item: int) {
        self.items.push(item);
    }

}

let stack = Stack();
stack.push(1);
println stack.items;

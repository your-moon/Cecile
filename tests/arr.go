// Compile time known type because of array's elements are defined, type is Vec<int>
// Decide type with first element
let arr1 = [123, 456, 101, 112, 543];

fn quick_sort(arr: Vec<int>) {
    if (arr.len() < 2) {
    }
    else {
        let pivot = arr[0];
        let left: Vec<int> = [];
        let right: Vec<int> = [];
        for (let i = 1; i < arr.len(); i = i + 1) {
            if (arr[i] < pivot) {
                left.push(arr[i]);
            }
            else {
                right.push(arr[i]);
            }
        }
        println arr;
        quick_sort(left);
        quick_sort(right);
        print "left: ";
        println left;
        print "pivot: ";
        println pivot;
        print "right: ";
        println right;
        // arr = left + [pivot] + right;
    }
}

fn bubble_sort(arr: Vec<int>) {
    let len_ = arr.len();
    for (let i = 0; i < len_; i = i + 1) {
        for (let j = 0; j < len_ - i - 1; j = j + 1) {
            if (arr[j] > arr[j+1]) {
                let temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
            }
        }
    }
    println arr;
}

bubble_sort(arr1);
quick_sort(arr1);

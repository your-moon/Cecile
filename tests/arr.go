// Declartion of array
let arr1 = [123, 456, 101, 112, 543];

// Accessing element of array
println arr1[0];

// Adding element to array
for (let i = 0; i < 10; i = i + 1) {
  arr1.push(random_number());
  
}

// Quicksort algorithm
fn partition(arr: Vec<int>, low: int, high: int) -> int {
    let pivot = arr[high];
    let i = low - 1;
    for (let j = low; j < high; j = j + 1) {
        if (arr[j] < pivot) {
            i = i + 1;
            let temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
        }
    }
    let temp = arr[i + 1];
    arr[i + 1] = arr[high];
    arr[high] = temp;
    return i + 1;
}


// Quicksort algorithm
fn quick_sort(arr: Vec<int>, low: int, high: int) {
    if (low < high) {
        let pi = partition(arr, low, high);
        quick_sort(arr, low, pi - 1);
        quick_sort(arr, pi + 1, high);
    }
}


// Bubble sort algorithm
fn bubble_sort(arr: Vec<int>) -> Vec<int> {
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
    return arr;
}

// Testing sorting algorithms

// Bubble sort
let start_time = clock();
let arr = bubble_sort(arr1.copy());
let end_time = clock();
print "bubble sort time: ";
println end_time - start_time;
println arr;


// Quick sort
let start_time = clock();
quick_sort(arr1, 0, arr1.len() - 1);
let end_time = clock();
print "quick sort time: ";
println end_time - start_time;
println arr1;



// Declartion of array of string
let str_arr = ["hello", "world", "this", "is", "a", "test"];

// Accessing element of array
println str_arr[0];


// Playing with array of string
fn concat_arr(arr: Vec<String>) -> String {
    let res = "";
    for (let i = 0; i < arr.len(); i = i + 1) {
        res = res + arr[i] + " ";
    }
    return res;
}
println concat_arr(str_arr);



// Declartion of array of Objects
type Dog {
  name: String
}

impl Dog {
    fn new(name: String) {
      self.name = name;
    }
}

// Declartion of array of Objects
let bob = Dog("bob");
let julie = Dog("julie");
let emy = Dog("emy"); 
let dogs: Vec<Dog> = [];

// Adding element to array
dogs.push(bob);
dogs.push(julie);
dogs.push(emy);

// Printing array's elements
for (let i = 0; i< dogs.len(); i = i + 1) {
  println dogs[i].name;
}


// Compile time known type because of array's elements are defined, type is Vec<int>
// Decide type with first element
let arr1 = [123, 456, 101, 112, 543];
println arr1;

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


fn quick_sort(arr: Vec<int>, low: int, high: int) {
    if (low < high) {
        let pi = partition(arr, low, high);
        quick_sort(arr, low, pi - 1);
        quick_sort(arr, pi + 1, high);
    }
}


//
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

let arr = bubble_sort(arr1);
println arr;
quick_sort(arr1, 0, arr1.len() - 1);
println arr1;

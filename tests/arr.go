





// Compile time known type because of array's elements are defined, type is Vec<int>
// Decide type with first element
let arr = [123, 456, 101, 112, 543, "s"];

// Create empty array with type
let arr2: Vec<String> = [];

//This should be error because of type is unknown 
let arr3 = []; 



// fn bubble_sort(arr: Vec<int>) {
//     let n = 5;
//     for (let i = 0; i < n - 1; i = i + 1) {
//         for (let j = 0; j < n - i - 1; j = j + 1) {
//             if (arr[j] > arr[j + 1]) {
//                 let tmp = arr[j];
//                 arr[j] = arr[j + 1];
//                 arr[j + 1] = tmp;
//             }
//         }
//     }
//     println arr;
// }
//
// bubble_sort(arr);
//
//
// fn find_max(arr: Vec<int>) -> int {
//     let max = arr[0];
//     for (let i = 1; i < 5; i = i + 1) {
//         if (arr[i] > max) {
//             max = arr[i];
//         }
//     }
//     return max;
// }
//
// println find_max(arr);


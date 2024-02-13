

fn main() {
    // ------------ Binary Search ---------------------
    // Searches in a sorted array
    // 
    // array = [ 2, 7, 9 ,11, 20, 25, 27, 50 , 51, 60], 
    // target = 25
    // ------------------------------------------------
    let array = [ 2, 7, 9 ,11, 20, 25, 27, 50 , 51, 60];
    let target = 60;

    let start = 0;
    let end  = array.len() -1;

    let result = binary_search(start, end, array, target);

    println!("The value searching is in the position  {}", result);
}

/*
Steps:
1. Define the start and the end index;
2. Define the middle index (start + end / 2);
3. If A[middle] < target , search to the right of the middle element.
Need to change our start by the middle value and repeat the process
*/

fn binary_search(start:usize, end:usize , array: [i32;10], target: i32) -> i32 {
    

    if start > end {
        return -1;
    }

    let middle = (start + end) / 2;
    
    if target == array[middle] {
        return middle as i32;
    }

    if array[middle] < target {
        return binary_search(middle +1 , end, array, target)
    }
    else {
        return binary_search(start , middle -1, array, target)
    }

    
}

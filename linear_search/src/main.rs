fn main() {
    let array = [ 2, 7, 9 ,11, 20, 25, 27, 50 , 51, 60];
    let target = 1;
    //let target = 9;
    //let target = 60;

    let result = linear_search(&array, target);

    if result == -1 {
        println!("The value is not found");
    }
    else {
        println!("The value is found in the position: {}", result);
    }

    
}

fn linear_search(array:&[i32], target:i32) -> i32 {
    for (index, _value) in array.iter().enumerate() {
        if array[index] == target {
            return index as i32;
        }
    }
    return -1;
}

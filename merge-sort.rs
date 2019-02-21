fn main() {
    let mut my_vector = vec![11,45,14,8,10,191,9,36,4,3,64];
    println!("{:?}", merge_sort(&my_vector));
}

fn merge_sort(my_vector:&Vec<u32>) -> Vec<u32> {
    println!("in {:?}", my_vector.clone());
    if my_vector.len() == 1 {
        println!("{:?}", my_vector.clone());
        return my_vector.clone();
    }
    if my_vector.len() == 2 {
        if my_vector[0] > my_vector[1] {
            println!("{:?}", vec![my_vector[1], my_vector[2]]);
            return vec![my_vector[1], my_vector[2]]
        } else {
            println!("{:?}", my_vector.clone());
            return my_vector.clone();
        }
    }
    let left_sorted = merge_sort(&my_vector[0..my_vector.len() / 2].to_vec());
    let right_sorted = merge_sort(&my_vector[(my_vector.len() / 2)..my_vector.len()].to_vec());
    let mut result:Vec<u32> = Vec::new();
    let mut left_counter = 0;
    let mut right_counter = 0;
    
    loop {
        if (left_counter == left_sorted.len()) {
            let mut append_vector:Vec<u32> = right_sorted[right_counter..right_sorted.len()].to_vec();
            result.append(&mut append_vector);
            println!("{:?}", result);
            return result;
        }
        if (right_counter == right_sorted.len()) {
            let mut append_vector = left_sorted[left_counter..left_sorted.len()].to_vec();
            result.append(&mut append_vector);
            println!("{:?}", result);
            return result;
        }
        if (left_sorted[left_counter] < right_sorted[right_counter]) {
            result.push(left_sorted[left_counter]);
            left_counter += 1;
        } else {
            result.push(right_sorted[right_counter]);
            right_counter += 1;
        }
    }
}

fn main() {
    let my_set = vec![16, 32, 43];

    let mut result: Vec<Vec<i32>> = vec![vec![]];

    for element in &my_set {
        let mut new_subsets: Vec<Vec<i32>> = Vec::new();

        for subset in &result {
            let mut new_subset: Vec<i32> = subset.clone();
            new_subset.push(*element);
            new_subsets.push(new_subset);
        }
        result.extend_from_slice(&new_subsets);
    }
    println!("The power set of {:?} is {:?}", &my_set, result);
}

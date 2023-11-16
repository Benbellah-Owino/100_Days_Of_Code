fn main() {
    println!("{}", progress_days(Vec::from([3, 4, 1, 2])))
}

fn progress_days(arr: Vec<i32>) -> i32 {
    let mut pres;
    let mut next;
    let mut progress: i32 = 0;

    //let arr2 = arr.clone();

    for (i, v) in arr.iter().enumerate() {
        println!("Hit {progress}");
        let next_index = i + 1;
        if next_index == arr.len() {
            println!("break");
            break;
        }
        pres = v;
        next = &arr[i + 1];

        println!("Iteration: {i}, Present Number {pres}, Next Number {next}, Progress{progress} ");
        if *next > *pres {
            progress += 1;
        }
    }

    progress
}

pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut sum = 0;
    let mut res = Vec::new();
    res.push(0);
    for i in arr {
        sum += i;
        res.push(sum);
    }
    res.reverse();
    res
}


fn main() {
    println!(
        "Partial sums of [5, 18, 3, 23] is : {:?}",
        parts_sums(&[5, 18, 3, 23])
    );
}

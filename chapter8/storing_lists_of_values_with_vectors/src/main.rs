fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3];
    println!("{:?}", v);

    // Updating a Vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);
}
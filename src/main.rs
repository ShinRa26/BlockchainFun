fn main() {
    let mut x = vec![1,2,3,4,5,6,7,8,9,10];
    let mut y = [0,0,0,0,0,0,0,0,0,0];
    y.clone_from_slice(&x);
    for elem in y.iter() {
        println!("{}", elem);
    }
    x.clear();
    for elem in y.iter() {
        println!("{}", elem);
    }
}
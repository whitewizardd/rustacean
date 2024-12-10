

fn main() {

    let mut x = 5;
    let mut y = x;
    let mut z = &mut x;
    //NV: you can only borrow a mutable reference once while you can borrow to more than one
    let mut zz = &mut x;
    // x = 7;
    println!("here is the value of x ::: {} ", x);
}
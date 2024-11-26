

fn main() {
    // in rust there are different data types
    // the u8 and i8 are the smallest amongest all data types along with the bool
    // the u stands for unsigned integer
    // the i stands for the signed integer

    // any variable that it data type starts with u means that the value can never have a negative value 
    // while the i means the variable can contain both negative and positive value 

    // the formula for u is this 2^(n) - 1
    // hence u8 means , 2 ^ 8 - 1 = 255
    // while i8 is between -128 to 127
    // i8 = -2^(n - 1) to 2^(n-1) -1 


    // below is to check the min and max value for signed and unsigned integers (i.e, i and u)
    // also we'd check that of the floating point as well, f64 and f32

    // signed integers i8, -128 to 127
    println!("here is the minimum and maximum value for the signed integer for i8 ::: {} , {}", i8::MIN , i8::MAX);

    //unsigned integers u8, 0 - 255
    println!("here is the minimum and maximum value for the unsigned integer for u8 ::: {} , {}", u8::MIN , u8::MAX);

    // max and min for f32
    println!("here is the minimum and maximum value for the floating integer for f32::: {} , {}", f32::MIN , f32::MAX);

    // max and min for f64 
    println!("here is the minimum and maximum value for the floating integer for f64 ::: {} , {}", f64::MIN , f64::MAX);
}
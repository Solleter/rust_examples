pub fn execute () {
    let logical: bool = true;
    let a_float: f64 = 1.0;     // 常规说明
    let an_integer = 5i32; // 后缀说明

    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

//    mutable = true; // error

    let mutable = true; // 覆盖前面的类型

    println!("{} {} {} {} {} {} {}", logical, a_float, an_integer, default_float,
    default_integer, inferred_type, mutable);
}
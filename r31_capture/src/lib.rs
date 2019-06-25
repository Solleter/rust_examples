pub fn execute() {
    capture1();
    capture2();
}

fn capture1() {
    use std::mem;
    let color = "green";

    let print = || println!("Closure Color: {}", color);
    print();
    println!("Color: {}", color);
    print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("Closure Count: {}", count);
    };

    inc();
    // 这一句加在这里就会报错，是因为借用...?
//    println!("origin count: {}", count);
    inc();
    println!("origin count: {}", count);

    let movable = Box::new(3);
    let consume = || {
        println!("closure moable: {:?}", movable);
        mem::drop(movable);
    };

    consume();
}

fn capture2() {
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    // haystack 已经移动到了闭包里，不能在外面继续使用
//    println!("There're {} elements in vec", haystack.len());
    println!("{}", contains(&1));
    println!("{}", contains(&2));

}
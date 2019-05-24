use r01_display;
use r02_testlist;
use r03_format;
use r04_datatype;
use r05_literal;
use r06_tuple;
use r07_array_slice;

fn main() {
    let example = 7;

    match example {
        1 => r01_display::execute(),
        2 => r02_testlist::execute(),
        3 => r03_format::execute(),
        4 => r04_datatype::execute(),
        5 => r05_literal::execute(),
        6 => r06_tuple::execute(),
        7 => r07_array_slice::execute(),
        _ => (),
    }

}

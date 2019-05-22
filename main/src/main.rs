use r01_display;
use r02_testlist;
use r03_format;
use r04_datatype;
use r05_literal;

fn main() {
    let example = 5;

    match example {
        1 => r01_display::execute(),
        2 => r02_testlist::execute(),
        3 => r03_format::execute(),
        4 => r04_datatype::execute(),
        5 => r05_literal::execute(),
        _ => (),
    }

}

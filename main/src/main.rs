use r01_display;
use r02_testlist;
use r03_format;
use r04_datatype;
use r05_literal;
use r06_tuple;
use r07_array_slice;
use r08_struct;
use r09_enum;
use r10_enum_use;
use r11_enum_cstyle;
use r12_testcase_linklist;
use r13_const;

fn main() {
    let example = 13;

    match example {
        1 => r01_display::execute(),
        2 => r02_testlist::execute(),
        3 => r03_format::execute(),
        4 => r04_datatype::execute(),
        5 => r05_literal::execute(),
        6 => r06_tuple::execute(),
        7 => r07_array_slice::execute(),
        8 => r08_struct::execute(),
        9 => r09_enum::execute(),
        10 => r10_enum_use::execute(),
        11 => r11_enum_cstyle::execute(),
        12 => r12_testcase_linklist::execute(),
        13=> r13_const::execute(),
        _ => (),
    }

}

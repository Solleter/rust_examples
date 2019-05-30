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
use r14_variable_bind;
use r15_mut_variable;
use r16_scope_shadowing;
use r17_type_convert;
use r18_literal_variable;
use r19_type_deduce;
use r20_alias;

fn main() {
    let example = 20;

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
        13 => r13_const::execute(),
        14 => r14_variable_bind::execute(),
        15 => r15_mut_variable::execute(),
        16 => r16_scope_shadowing::execute(),
        17 => r17_type_convert::execute(),
        18 => r18_literal_variable::execute(),
        19 => r19_type_deduce::execute(),
        20 => r20_alias::execute(),
        _ => (),
    }

}

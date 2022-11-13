// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.


// use std::collections::btree_map::Values;

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        //my_option.unwrap();
        println!("none");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // unit value  not elegant
    let my_empty_vec = vec![1, 2, 3, 4, 5];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let value_b: i32 = std::mem::replace(&mut value_a, 66);
    // let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    // std::mem::replace()
    println!("value a: {}; value b: {}", value_a, value_b);
}

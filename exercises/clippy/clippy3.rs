// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
#[allow(clippy::let_unit_value)]
#[allow(clippy::unnecessary_literal_unwrap)]
#[allow(clippy::unnecessary_unwrap)]
fn main() {
    let my_option: Option<()> = None;
    if let Some(()) = my_option {
        // 如果 my_option 不是 None，这里的代码块将被执行
        println!("my_option has a value");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    println!("value a: {}; value b: {}", value_a, value_b);
}

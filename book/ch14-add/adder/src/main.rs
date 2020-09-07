// @Note: Clippy warns that these imports are redundant:
//  |
//  |   use add_one;
//  |   use add_two;
//
// See https://rust-lang.github.io/rust-clippy/master/index.html#single_component_path_imports

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
    println!(
        "And {} plus two is equal to {}!",
        num,
        add_two::add_two(num)
    );
}

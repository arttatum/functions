fn main() {
    let x = five();
    println!("x: {}", x);

    let y = plus_one(five());
    println!("y: {}", y);
}

// 1) Use -> to define return types
// 2) Expressions don't need to include a closing ;
//      - the lack of a ; indicates this is what will be returned.
//      - adding a ; alone would convert this 'expression' into a 'statement'.
//      - the return keyword can be used to indicate we are returning an expression.
fn five() -> i32 {
    return 5
}

fn plus_one(input: i32) -> i32 {
    input + 1
}
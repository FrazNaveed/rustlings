// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn add_to_list<T>(item: T) -> Vec<T> {
    let mut shopping_list: Vec<T> = Vec::new();
    shopping_list.push(item);
    return shopping_list;
}

fn main() {
    let shopping_list = add_to_list("milk");
    println!("{:?}", shopping_list);
}

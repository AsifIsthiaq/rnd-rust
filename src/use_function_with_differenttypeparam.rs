// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];
//
//     let mut sort_operations = vec![];
//     // let value = String::from("by key called");
//     let value = 10;
//
//     list.sort_by_key(|r| {
//         sort_operations.push(value);
//         r.width
//     });
//     println!("{:#?}", list);
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn main() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];
//
//     let mut num_sort_operations = 0;
//     list.sort_by_key(|r| {
//         num_sort_operations += 1;
//         r.width
//     });
//     println!("{:#?}, sorted in {num_sort_operations} operations", list);
// }

trait Number{
    fn get(&self) -> Self;
}

impl Number for u32 {
    fn get(&self) -> Self {
        *self
    }
}

impl Number for f32 {
    fn get(&self) -> Self {
        *self
    }
}
fn test<T:Number>(param:T)->T{
    param
    // println!("{:?}",param);
}
fn main() {
    println!("{}", test(1));
    println!("{}", test(1.1));
    println!("{}", test(-1.1));
    println!("{}", test(1));
    // test(1);
    //  test(1.1);
    //  test(-1.1);
    //  test(-1);

}
fn main() {
    let new_vec = (1..).take(10).collect::<Vec<i32>>();
    println!("{:?}", new_vec);

    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // into_iter gives us owned values not references
    let new_vec = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();
    println!("{:?}", new_vec);
}

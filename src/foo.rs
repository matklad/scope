pub fn bar() {
    println!("I'm bar");
}

pub fn fn_foo() {
    println!("calling function using the crate scope");
    ::fn_main();

//     Will not compile
//     ::bar();

    ::foo::bar();
}

use test_61a7cd7f28e0::*;

fn main() {
    let foo: Foo = Foo::new("yes".into());
    let bar = Bar { amount: 1200. };
    println!("{}", bar.pa());
}

#[derive(Debug)]
pub struct Foo {
    foo: String,
    yes: bool,
}

impl Foo {
    pub fn new(foo: String) -> Self {
        Self { foo, yes: true }
    }

    pub fn maybe(&self) {
        if self.yes {
            println!("{}", self.foo)
        }
    }
}

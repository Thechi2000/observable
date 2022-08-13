
#[cfg(test)]
mod tests {
    use crate::Observable;

    #[derive(Observable)]
    struct Foo {
        field: usize,
    }

    #[test]
    fn main() {
        let a = Foo {
            field: 0
        };
        let b = Foo {
            field: 1
        };

        a.update(b);
    }
}
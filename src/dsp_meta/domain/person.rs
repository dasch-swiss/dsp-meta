#[derive(Debug, PartialEq)]
pub struct Person<'a> {
    id: &'a str,
}

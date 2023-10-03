pub mod Book {
    use crate::Person;
    pub struct Book<'a> {
        pub title: String,
        pub author: String,
        pub is_available: bool,
        pub borrower: Option<&'a Person>,
    }

    impl<'a> Book<'a> {
        pub fn create(title:  String, author: String, is_available: bool) -> Book<'a> {
            Book {
                title,
                author,
                is_available,
                borrower: None,
            }
        }
    }
}

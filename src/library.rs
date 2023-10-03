pub mod Library {
    use std::{ io::Error, io::ErrorKind};

    use crate::{book::Book::Book, person::Person::Person};
    pub struct Library<'a> {
        books: Vec<Book<'a>>,
    }

    impl<'a> Library<'a>{
        pub fn create() -> Self {
            Library { books: vec![] }
        }

        pub fn add_book(self: &mut Self, book:  Book<'a>) {
            self.books.push(book);
        }

        pub fn checkout_book(
            self: &mut Self,
            title: String,
            person: &'a Person,
        ) -> Result<&Book, Error> {
            let book = self.books.iter_mut().find(|b| b.title == title);
            if let Some(book) = book {
                println!("{} successfully checkedout by {}", title, person.name);
                book.is_available = false;
                book.borrower = Some(person);
                Ok(book)
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Book {} was already borrowed", title),
                ));
            }
        }

        pub fn return_book() {}

        pub fn list_all_books() {}

        pub fn list_checkout_books() {}
    }
}

pub mod book;
pub mod library;
pub mod person;

use book::Book::Book;
use library::Library::Library;
use person::Person::Person;

fn main() {
    let person1 = Person::create("P1".to_string(), 18);
    person1.print();
    let book1 = Book::create("Book1".to_string(), "Author1".to_string(), true);
    let book2 = Book::create("Book2".to_string(), "Author2".to_string(), true);
    let mut library = Library::create();
    library.add_book(book1);
    library.add_book(book2);
    let result = library.checkout_book("Book1".to_string(), &person1);
    if result.is_ok() {
        println!("Successfully checkedout by {}", person1.name);
    }
}

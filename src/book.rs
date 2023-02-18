#[derive(Debug)]
pub struct LibBook {
    pub book_details: Book,
    book_id: u8,
    pub author_id: u8,
    no_of_borrow: u8,
    is_borrowed: bool,
    status: char,
}

#[derive(Debug)]
/// general book information
pub struct Book {
    pub name: String,
}

pub struct Library {
    books: Vec<LibBook>,
    last_book_id: u8,
}

pub enum BookState<'a> {
    Found(&'a Book),
    NotFound,
    Borrowed,
}

/// first create a library
///  then books can be created in the library
impl Library {
    /// create a new library
    pub fn new() -> Library {
        Library {
            books: vec![],
            last_book_id: 0,
        }
    }

    fn get_last_id(&self) -> u8 {
        self.last_book_id
    }

    /// register a new book
    pub fn register_new_book(&mut self, name: String, author_id: u8) -> &LibBook {
        let new_book = LibBook {
            book_details: Book { name },
            book_id: self.get_last_id(),
            author_id,
            no_of_borrow: 0,
            is_borrowed: false,
            status: 'A',
        };
        self.last_book_id += 1;
        self.books.push(new_book);
        self.books.last().unwrap()
    }

    /// get all books as ref
    pub fn list_of_books(&self) -> &Vec<LibBook> {
        &self.books
    }

    /// borrow a book from library if found and already not borrowed
    pub fn borrow_book(&mut self, book_id: u8) -> BookState {
        // find the book a take its value
        let book_op = self.books.iter_mut().find(|bk| bk.book_id == book_id);
        match book_op {
            None => BookState::NotFound,
            Some(bk) => {
                // if the book has been removed from this library
                if bk.status == 'D' {
                    return BookState::NotFound;
                }
                // if the book is already borrowed
                if bk.is_borrowed {
                    return BookState::Borrowed;
                }
                // else increment borrowed counter by 1
                bk.no_of_borrow += 1;
                bk.is_borrowed = true;
                BookState::Found(&bk.book_details)
            }
        }
    }
}

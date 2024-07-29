use std::error::Error;
use std::os::unix::raw::time_t;
use std::{fs, io};
use std::io::{Write};
use std::process;
use std::path::Path;


struct BookWareHouse {
    all_book: Vec<book>,
}

impl BookWareHouse {
    fn new() -> Result<BookWareHouse, Box<dyn Error>> {

       let mut books = Vec::new();
       let books_path = Path::new("book");

       if books_path.is_dir() {
            for entry in fs::read_dir(books_path) ? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    let file_name =path.file_stem().unwrap().to_str().unwrap();
                    let parts: Vec<&str> = file_name.split('_').collect();
                    if parts.len() == 2 {
                        let title = parts[0].to_string();
                        let author = parts[1].to_string();
                        let content = fs::read_to_string(path)?;

                        books.push(book {
                            title,
                            author,
                            content,
                        })
                    }
                }
            }
       } else {
            return Err("The books directory does not exist".into());
       }

        Ok(BookWareHouse { all_book: books})

    }

    pub fn list_book(&self) {
       for book in & self.all_book {
        println!("《{}》 作者：{}", book.title, book.author);
       }
    }

    pub fn add_book(&mut self, title: String, author: String, content: String) {
        self.all_book.push(book {
            title,
            author,
            content,
        })
    }

    pub fn find_book(&self, title:&str) -> Option<&book> {
        self.all_book.iter().find(|&book| book.title == title)
    }

    pub fn remove_book(&mut self, title: &str) {
        self.all_book.retain(|book| book.title != title)
    }
}

struct book {
    title: String,
    author: String,
    content: String,
}



pub fn run() -> Result<(), Box<dyn Error>> {

    let mut book_warehouse = BookWareHouse::new().unwrap_or_else(|err| {
        println!("Problem: {}", err);
        process::exit(1);
    });


    println!("当前书籍总数: {}", book_warehouse.all_book.len());

    loop {

        println!("
        Hi there! Welcome to the Library\n\n
        Please select the service:\n
        + 1. Add a Book.\n
        + 2. List all Books. \n
        + 3. Find a Book. \n
        + 4. Remove a Book. \n
        + 5. Save and Load. \n
        + 6. Exit the program. \n
        ");    

        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let choice: i32 = input.trim().parse().expect("Please enter a valid number");

        
        match choice {
            1 => {
                println!("Enter the book title:");
                    let mut title = String::new();
                    io::stdin().read_line(&mut title).expect("Failed to read line");

                    println!("Enter the book author:");
                    let mut author = String::new();
                    io::stdin().read_line(&mut author).expect("Failed to read line");

                    println!("Enter the book content:");
                    let mut content = String::new();
                    io::stdin().read_line(&mut content).expect("Failed to read line");
                    book_warehouse.add_book(title, author, content)
            }   
            2 => book_warehouse.list_book(),
            3 => {
                println!("Please enter a book title to search");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                match book_warehouse.find_book(title.trim()) {
                    Some(book) => println!("Found the book"),
                    None => println!("Book not found"),
                };
            },
            4 => {
                println!("Enter the book title to remove:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                book_warehouse.remove_book(title.trim());
            },
            5 => println!("You selected: Save and Load."),
            6 => {
                println!("Bye Bye!");
                break;
            }
            _ => {
                eprintln!("Invalid choice. Please select a number between 1 and 5.");
                process::exit(1); // Exit the program with an error code
            },
   
        }

    }   
    Ok(())
}
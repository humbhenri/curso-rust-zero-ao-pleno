use ordinal::ToOrdinal as _;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Book {
    title: String,
    author: String,
    year: u32,
}

fn collection() -> Vec<Book> {
    vec![
        Book {
            title: "Book 1".to_string(),
            author: "Author 1".to_string(),
            year: 2001,
        },
        Book {
            title: "Book 2".to_string(),
            author: "Author 2".to_string(),
            year: 1991,
        },
        Book {
            title: "Book 3".to_string(),
            author: "Author 3".to_string(),
            year: 2003,
        },
        Book {
            title: "Book 4".to_string(),
            author: "Author 4".to_string(),
            year: 1990,
        },
        Book {
            title: "Book 5".to_string(),
            author: "Author 5".to_string(),
            year: 1800,
        },
    ]
}

fn main() {
    let books = collection();

    // agrupa os livros por século
    let mut seculos: HashMap<u32, Vec<Book>> = HashMap::new();
    for book in books.iter() {
        let seculo = book.year / 100 + 1;
        seculos
            .entry(seculo)
            .or_insert_with(Vec::new)
            .push(book.clone());
    }
    for (seculo, livros) in seculos {
        println!("Século {}", seculo.to_ordinal_string());
        println!("Livros:");
        for livro in livros.iter() {
            println!("{} {}", livro.title, livro.author);
        }
        println!("-------\n");
    }
}

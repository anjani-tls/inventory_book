#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data buku (upgrade)
#[contracttype]
#[derive(Clone, Debug)]
pub struct Book {
    id: u64,
    title: String,
    author: String,
    category: String,
    stock: u32,
}

// Storage key
const BOOK_DATA: Symbol = symbol_short!("BOOK_DATA");

#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {

    // Ambil semua buku
    pub fn get_books(env: Env) -> Vec<Book> {
        env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env))
    }

    // Tambah buku
    pub fn add_book(env: Env, title: String, author: String, category: String, stock: u32) -> String {
        let mut books: Vec<Book> = env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));

        let book = Book {
            id: env.prng().gen::<u64>(),
            title,
            author,
            category,
            stock,
        };

        books.push_back(book);
        env.storage().instance().set(&BOOK_DATA, &books);

        String::from_str(&env, "Buku berhasil ditambahkan")
    }

    // Hapus buku
    pub fn delete_book(env: Env, id: u64) -> String {
        let mut books: Vec<Book> = env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));

        for i in 0..books.len() {
            if books.get(i).unwrap().id == id {
                books.remove(i);
                env.storage().instance().set(&BOOK_DATA, &books);
                return String::from_str(&env, "Buku berhasil dihapus");
            }
        }

        String::from_str(&env, "Buku tidak ditemukan")
    }

    // Update stok (tidak boleh minus)
    pub fn update_stock(env: Env, id: u64, new_stock: u32) -> String {
        let mut books: Vec<Book> = env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));

        for i in 0..books.len() {
            let mut book = books.get(i).unwrap();

            if book.id == id {
                book.stock = new_stock;
                books.set(i, book);

                env.storage().instance().set(&BOOK_DATA, &books);
                return String::from_str(&env, "Stok berhasil diperbarui");
            }
        }

        String::from_str(&env, "Buku tidak ditemukan")
    }

    // 🔥 Borrow buku (stok -1)
    pub fn borrow_book(env: Env, id: u64) -> String {
        let mut books: Vec<Book> = env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));

        for i in 0..books.len() {
            let mut book = books.get(i).unwrap();

            if book.id == id {
                if book.stock == 0 {
                    return String::from_str(&env, "Stok habis");
                }

                book.stock -= 1;
                books.set(i, book);

                env.storage().instance().set(&BOOK_DATA, &books);
                return String::from_str(&env, "Buku berhasil dipinjam");
            }
        }

        String::from_str(&env, "Buku tidak ditemukan")
    }

    // 🔥 Return buku (stok +1)
    pub fn return_book(env: Env, id: u64) -> String {
        let mut books: Vec<Book> = env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));

        for i in 0..books.len() {
            let mut book = books.get(i).unwrap();

            if book.id == id {
                book.stock += 1;
                books.set(i, book);

                env.storage().instance().set(&BOOK_DATA, &books);
                return String::from_str(&env, "Buku berhasil dikembalikan");
            }
        }

        String::from_str(&env, "Buku tidak ditemukan")
    }

    // 🔍 Search berdasarkan judul
    pub fn search_book(env: Env, keyword: String) -> Vec<Book> {
        let books: Vec<Book> = env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));
        let mut result = Vec::new(&env);

        for i in 0..books.len() {
            let book = books.get(i).unwrap();

            if book.title == keyword {
                result.push_back(book);
            }
        }

        result
    }
}

mod test;
use std::collections::HashMap;

pub enum Gender {
    Male,
    Female,
    NoAnswer,
}

// type-builder
pub struct User {
    pub id: u32,
    pub age: u8,
    pub gender: Gender,
}

struct Movie {
    id: u32,
    title: String,
}

struct SeetId(u16);

struct Sheet {
    id: SeetId,
    user: User,
    movie: Movie,
}

struct Schedule {
    movie: Movie,
    date: String,
    reservation: HashMap<SeetId, Sheet>,
}

pub struct TheaterData {
    name: String,
    schedule: Schedule,
}

fn main() {
    println!("Hello, world!");
}

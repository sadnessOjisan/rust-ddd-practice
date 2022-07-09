use std::collections::HashMap;

enum Gender {
    Male,
    Female,
    NoAnswer
}

struct User {
    id: u32,
    age: u8,
    gender: Gender
}

struct Movie {
    id: u32,
    title: String
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
    reservation: HashMap<SeetId, Sheet>
}

pub struct TheaterData {
    name: String,
    schedule: Schedule
}

fn main() {
    println!("Hello, world!");
}

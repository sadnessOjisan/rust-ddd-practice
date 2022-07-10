use domain::{TheaterData};
#![feature(once_cell)]
use std::lazy::OnceCell;

static Cella = OnceCell::new();

struct Repository {
    data: TheaterData
}
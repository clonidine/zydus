pub mod user;

pub trait Repository<T> {
    fn find_one(id: u64) -> T;
    fn find_all() -> Vec<T>;
}
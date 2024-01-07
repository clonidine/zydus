pub mod user;

pub trait Repository<T> {
    fn find_one(id: u64) -> color_eyre::Result<T>;
    fn find_all() -> Vec<T>;
}
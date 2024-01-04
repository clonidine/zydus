use crate::domain::user::User;

use super::Repository;

pub struct UserRepository;

impl Repository<User> for UserRepository {
    fn find_one(id: u64) -> User {
        todo!()
    }

    fn find_all() -> Vec<User> {
        todo!()
    }
}
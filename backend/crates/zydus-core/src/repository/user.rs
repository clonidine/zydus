use crate::domain::user::User;

use super::Repository;

pub struct UserRepository;

impl Repository<User> for UserRepository {
    #[allow(unused_variables)]
    fn find_one(id: u64) -> color_eyre::Result<User> {
        todo!()
    }

    fn find_all() -> Vec<User> {
        todo!()
    }
}


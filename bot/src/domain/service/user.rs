use domain::*;

pub fn find_or_create(
    user_telegram_id: i64,
    username: &str,
    user_repo: &UserRepository
) -> User {
    user_repo.find_one_by_telegram_id(user_telegram_id)
        .unwrap_or_else(|| user_repo.create(NewUser::new(username.to_owned(), user_telegram_id)))
}

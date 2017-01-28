use domain::*;

pub fn find_or_create(
    adjective_value: &str,
    repository: &AdjectiveRepository
) -> Adjective {
    repository.find_one_by_value(adjective_value)
        .unwrap_or_else(|| repository.create(NewAdjective::with_value(adjective_value.to_owned())))
}

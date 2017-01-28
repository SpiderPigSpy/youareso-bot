use domain::*;

pub fn find_or_create(
    subject_value: &str,
    repository: &SubjectRepository
) -> Subject {
    repository.find_one_by_value(subject_value)
        .unwrap_or_else(|| repository.create(NewSubject::with_value(subject_value.to_owned())))
}

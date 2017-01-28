use id::*;

pub type SubjectId = Id<i64, Subject>;

#[derive(Debug, Clone)]
pub struct Subject {
    id: SubjectId,
    value: String
}

pub struct NewSubject {
    pub value: String
}

impl NewSubject {
     pub fn with_value(value: String) -> NewSubject{
         NewSubject {
             value: value
         }
     }
}

impl Subject {
     pub fn new(id: SubjectId, value: String) -> Subject {
         Subject {
             id: id,
             value: value
         }
     }

     pub fn id(&self) -> SubjectId {
         self.id
     }
}

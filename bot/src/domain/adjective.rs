use id::*;

pub type AdjectiveId = Id<i64, Adjective>;

#[derive(Debug, Clone)]
pub struct Adjective {
    id: AdjectiveId,
    value: String
}

pub struct NewAdjective {
    pub value: String
}

impl NewAdjective {
     pub fn with_value(value: String) -> NewAdjective{
         NewAdjective {
             value: value
         }
     }
}

impl Adjective {
     pub fn new(id: AdjectiveId, value: String) -> Adjective {
         Adjective {
             id: id,
             value: value
         }
     }

     pub fn id(&self) -> AdjectiveId {
         self.id
     }
}

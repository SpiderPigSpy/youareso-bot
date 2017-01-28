use diesel::expression::sql_literal::SqlLiteral;
use diesel::types::*;

///Грязный хак, работает только в Postgres
pub fn random_order() -> SqlLiteral<Text> {
    SqlLiteral::new("random() ".to_owned())
}

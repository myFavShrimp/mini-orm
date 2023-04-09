use super::{Identifiable, TableEntity};

use mini_orm_macro::iden;

iden! {
    Id => "id",
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IdEntity<I>
where
    I: Copy + Send + Sync,
{
    pub id: I,
}

impl<I> TableEntity for IdEntity<I>
where
    I: Copy + Send + Sync,
{
    type Iden = Iden;

    fn all_columns() -> Vec<Self::Iden> {
        vec![Iden::Id]
    }

    fn table() -> Self::Iden {
        panic!("IdEntity does not have a table")
    }
}

impl<I: Copy> Identifiable<I> for IdEntity<I>
where
    I: Copy + Send + Sync,
{
    fn id(&self) -> I {
        self.id
    }

    fn id_column() -> Self::Iden {
        Iden::Id
    }
}

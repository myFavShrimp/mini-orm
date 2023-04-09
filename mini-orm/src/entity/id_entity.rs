use super::{Identifiable, TableEntity};

use mini_orm_macro::iden;

iden! {
    Id => "id",
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
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

impl<I> Identifiable for IdEntity<I>
where
    I: Copy + Send + Sync,
{
    type IdType = I;

    fn id(&self) -> I {
        self.id
    }

    fn id_column() -> Self::Iden {
        Iden::Id
    }
}

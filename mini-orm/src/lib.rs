mod id_entity;

pub trait Identifiable: TableEntity
where
    Self::IdType: Copy + Send + Sync,
{
    type IdType;

    fn id(&self) -> Self::IdType;

    fn id_column() -> Self::Iden;
}

pub trait TableEntity {
    type Iden;

    fn all_columns() -> Vec<Self::Iden>;

    fn table() -> Self::Iden;
}

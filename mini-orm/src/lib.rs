
pub trait Identifiable<I>: TableEntity {
    fn id(&self) -> I;

    fn id_column() -> Self::Iden;
}

pub trait TableEntity {
    type Iden;

    fn all_columns() -> Vec<Self::Iden>;

    fn table() -> Self::Iden;
}

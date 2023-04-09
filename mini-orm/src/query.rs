use sea_query::{Expr, Query, SelectStatement};

use crate::entity::{Identifiable, TableEntity};

pub async fn query<E>() -> SelectStatement
where
    E: TableEntity,
    <E as TableEntity>::Iden: 'static,
{
    Query::select()
        .columns(<E as TableEntity>::all_columns())
        .from(<E as TableEntity>::table())
        .to_owned()
}

pub async fn query_by_ids<E>(ids: Vec<<E as Identifiable>::IdType>) -> SelectStatement
where
    E: TableEntity + Identifiable,
    <E as TableEntity>::Iden: 'static,
    sea_query::Value: From<<E as Identifiable>::IdType>,
{
    Query::select()
        .columns(<E as TableEntity>::all_columns())
        .from(<E as TableEntity>::table())
        .and_where(Expr::col(<E as Identifiable>::id_column()).is_in(ids))
        .to_owned()
}

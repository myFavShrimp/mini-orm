use sea_query::{Expr, Query, SelectStatement};

use crate::{
    entity::{Identifiable, TableEntity},
    relation::OneToXRelation,
};

pub async fn query<E>() -> SelectStatement
where
    E: TableEntity,
{
    Query::select()
        .columns(<E as TableEntity>::all_columns())
        .from(<E as TableEntity>::table())
        .to_owned()
}

pub async fn query_by_ids<E>(ids: Vec<<E as Identifiable>::IdType>) -> SelectStatement
where
    E: Identifiable,
    sea_query::Value: From<<E as Identifiable>::IdType>,
{
    Query::select()
        .columns(<E as TableEntity>::all_columns())
        .from(<E as TableEntity>::table())
        .and_where(Expr::col(<E as Identifiable>::id_column()).is_in(ids))
        .to_owned()
}

pub async fn query_relational_by_id<A, B>(id: <A as Identifiable>::IdType) -> SelectStatement
where
    A: Identifiable + OneToXRelation<B>,
    B: Identifiable,
    sea_query::Value: From<<A as Identifiable>::IdType>,
{
    Query::select()
        .columns(<B as TableEntity>::all_columns())
        .from(<B as TableEntity>::table())
        .and_where(Expr::col(<A as OneToXRelation<B>>::target_relation_id_column()).eq(id))
        .to_owned()
}

use sea_query::{Alias, Expr, JoinType, Query, SelectStatement};

use crate::{
    entity::{Identifiable, TableEntity},
    relation::{ManyToManyRelation, OneToXRelation},
};

pub fn query<E>() -> SelectStatement
where
    E: TableEntity,
{
    Query::select()
        .columns(<E as TableEntity>::all_columns())
        .from(<E as TableEntity>::table())
        .to_owned()
}

pub fn query_by_ids<E>(ids: Vec<<E as Identifiable>::IdType>) -> SelectStatement
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

pub fn query_related_by_id<A, B>(id: <A as Identifiable>::IdType) -> SelectStatement
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

pub fn query_related_associative_by_id<A, B, R>(id: <A as Identifiable>::IdType) -> SelectStatement
where
    A: Identifiable + ManyToManyRelation<B, R>,
    B: Identifiable + ManyToManyRelation<A, R>,
    R: TableEntity,
    sea_query::Value: From<<A as Identifiable>::IdType>,
{
    Query::select()
        .columns(<B as TableEntity>::all_columns())
        .from(<B as TableEntity>::table())
        .join(
            JoinType::Join,
            <R as TableEntity>::table(),
            Expr::col((
                <R as TableEntity>::table(),
                <B as ManyToManyRelation<A, R>>::own_relation_id_column(),
            ))
            .equals((
                <B as TableEntity>::table(),
                <B as Identifiable>::id_column(),
            )),
        )
        .and_where(Expr::col(<A as ManyToManyRelation<B, R>>::own_relation_id_column()).eq(id))
        .to_owned()
}

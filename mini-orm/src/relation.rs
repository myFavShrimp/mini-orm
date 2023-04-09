use sea_query::Iden;

use crate::entity::{Identifiable, TableEntity};

/// Helper trait to define one to one and one to many relations on database entities.
/// B is the related entity.
pub trait OneToXRelation<B>
where
    B: TableEntity + Identifiable,
    B::Iden: Iden,
{
    fn target_relation_id_column() -> B::Iden;
}

/// Helper trait to define many to many relations on database tables.
/// B is the entity related to Self while R is the associative entity.
pub trait ManyToManyRelation<B, R>
where
    B: TableEntity + Identifiable,
    R: TableEntity,
{
    /// Id column on the relational table for the Self entity.
    fn own_relation_id_column() -> R::Iden;

    /// Id field of the related entity on the associative entity.
    fn other_entity_id(entity: R) -> <B as Identifiable>::IdType;
}

#[cfg(test)]
mod test {
    use crate::{entity::TableEntity, relation::OneToXRelation};

    use super::ManyToManyRelation;

    mod a {
        use crate::{
            entity::{Identifiable, TableEntity},
            macros::iden,
        };

        iden! {
            Table => "table",
            Id => "id",
            BId => "b_id",
        }

        pub struct Entity {
            id: u32,
        }

        impl TableEntity for Entity {
            type Iden = Iden;

            fn all_columns() -> Vec<Self::Iden> {
                vec![Iden::Id, Iden::BId]
            }

            fn table() -> Self::Iden {
                Iden::Table
            }
        }

        impl Identifiable for Entity {
            type IdType = u32;

            fn id(&self) -> u32 {
                self.id
            }

            fn id_column() -> Self::Iden {
                Iden::Id
            }
        }
    }

    mod b {
        use crate::{
            entity::{Identifiable, TableEntity},
            macros::iden,
        };

        iden! {
            Table => "table",
            Id => "id",
            AId => "a_id",
        }

        pub struct Entity {
            id: u32,
        }

        impl TableEntity for Entity {
            type Iden = Iden;

            fn all_columns() -> Vec<Self::Iden> {
                vec![Iden::Id, Iden::AId]
            }

            fn table() -> Self::Iden {
                Iden::Table
            }
        }

        impl Identifiable for Entity {
            type IdType = u32;

            fn id(&self) -> u32 {
                self.id
            }

            fn id_column() -> Self::Iden {
                Iden::Id
            }
        }
    }

    mod r {
        use crate::{entity::TableEntity, macros::iden};

        iden! {
            Table => "table",
            AId => "a_id",
            BId => "b_id",
        }

        pub struct Entity {
            pub a_id: u32,
            pub b_id: u32,
        }

        impl TableEntity for Entity {
            type Iden = Iden;

            fn all_columns() -> Vec<Self::Iden> {
                vec![Iden::AId, Iden::BId]
            }

            fn table() -> Self::Iden {
                Iden::Table
            }
        }
    }

    impl OneToXRelation<b::Entity> for a::Entity {
        fn target_relation_id_column() -> <b::Entity as TableEntity>::Iden {
            b::Iden::AId
        }
    }
    impl OneToXRelation<a::Entity> for b::Entity {
        fn target_relation_id_column() -> <a::Entity as TableEntity>::Iden {
            a::Iden::BId
        }
    }

    impl ManyToManyRelation<b::Entity, r::Entity> for a::Entity {
        fn own_relation_id_column() -> <r::Entity as TableEntity>::Iden {
            r::Iden::AId
        }

        fn other_entity_id(entity: r::Entity) -> u32 {
            entity.b_id
        }
    }
    impl ManyToManyRelation<a::Entity, r::Entity> for b::Entity {
        fn own_relation_id_column() -> <r::Entity as TableEntity>::Iden {
            r::Iden::BId
        }

        fn other_entity_id(entity: r::Entity) -> u32 {
            entity.a_id
        }
    }

    #[test]
    fn one_to_x() {
        let b_aid = <a::Entity as OneToXRelation<b::Entity>>::target_relation_id_column();
        let a_bid = <b::Entity as OneToXRelation<a::Entity>>::target_relation_id_column();

        assert!(b_aid == b::Iden::AId);
        assert!(a_bid == a::Iden::BId);
    }

    #[test]
    fn many_to_many() {
        let r_aid =
            <a::Entity as ManyToManyRelation<b::Entity, r::Entity>>::own_relation_id_column();
        let r_bid =
            <b::Entity as ManyToManyRelation<a::Entity, r::Entity>>::own_relation_id_column();

        assert!(r_bid == r::Iden::BId);
        assert!(r_aid == r::Iden::AId);
    }
}

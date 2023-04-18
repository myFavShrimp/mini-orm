# mini-orm 🚀

mini-orm is a blazingly fast Rust crate that utilizes traits to model relations between structs and makes them queriable. 💻

> While this crate serves a very specific use case in some of my projects, I welcome your feedback and suggestions. If you find it useful and would like to contribute, please feel free to submit pull requests. Additionally, if you think some changes are needed to make it work better for your particular use case, I'm open to hearing your thoughts and making updates accordingly.

## Features

- 🏗️ Trait-based modelling: Define your relations using Rust traits, allowing for easy customization and extensibility.
- 🌊 Built on top of sea-query: mini-orm utilizes the powerful SQL query builder, sea-query, to generate SQL queries automatically.

## Getting Started

Using mini-orm is easy! Here's a quick example to get you started:

First, define the structs representing your database tables.
```rust
#[derive(Iden)]
punb enum TableA {
    Table,
    Id,
    BId,
}

pub struct EntityA {
    id: u32,
    bid: u32,
}

impl TableEntity for EntityA {
    type Iden = TableA;

    fn all_columns() -> Vec<Self::Iden> {
        vec![TableA::Id, TableA::BId]
    }

    fn table() -> Self::Iden {
        TableA::Table
    }
}

impl Identifiable for EntityA {
    type IdType = u32;

    fn id(&self) -> u32 {
        self.id
    }

    fn id_column() -> Self::Iden {
        TableA::Id
    }
}
```

```rust
#[derive(Iden)]
punb enum TableB {
    Table,
    Id,
    AId,
}

pub struct EntityB {
    id: u32,
    aid: u32,
}

impl TableEntity for EntityB {
    type Iden = TableB;

    fn all_columns() -> Vec<Self::Iden> {
        vec![TableB::Id, TableB::AId]
    }

    fn table() -> Self::Iden {
        TableB::Table
    }
}

impl Identifiable for EntityB {
    type IdType = u32;

    fn id(&self) -> u32 {
        self.id
    }

    fn id_column() -> Self::Iden {
        TableB::Id
    }
}
```

Next, implement a relation between them.

```rust
impl OneToXRelation<EntityB> for EntityA {
    fn target_relation_id_column() -> <b::Entity as TableEntity>::Iden {
        TableB::AId
    }
}
```

```rust
impl OneToXRelation<EntityA> for EntityB {
    fn target_relation_id_column() -> <a::Entity as TableEntity>::Iden {
        TableA::BId
    }
}
```

Now, you can easily create queries using the provided methods.

```rust
let entity_query = query::<EntityA>();  // query all entities from TableA
let relational_query = query_related_by_id::<EntityA, EntityB>(42);  // query entities of TableB related to the entity of TableA with id 42
```



## Contributions

Contributions to mini-orm are always welcome! If you have ideas for new features or improvements, feel free to open an issue or submit a pull request. 🤝

## License

mini-orm is licensed under the MIT license. See the LICENSE file for more information. 📄

---

Disclaimer: This README was generated using ChatGPT. The crate author acknowledges their own limitations and laziness, and advises that this README should not be solely relied upon for accuracy or completeness.

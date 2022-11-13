//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "parents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub gender: Option<u8>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub family_id: Option<u64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::families::Entity",
        from = "Column::FamilyId",
        to = "super::families::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Families,
}

impl Related<super::families::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Families.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

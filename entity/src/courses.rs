//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "courses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sessions::Entity")]
    Sessions,
    #[sea_orm(has_many = "super::teachers::Entity")]
    Teachers,
}

impl Related<super::sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sessions.def()
    }
}

impl Related<super::teachers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teachers.def()
    }
}

impl Related<super::students::Entity> for Entity {
    fn to() -> RelationDef {
        super::student_course::Relation::Students.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::student_course::Relation::Courses.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

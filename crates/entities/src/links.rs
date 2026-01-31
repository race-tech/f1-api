use sea_orm::{Linked, RelationTrait};

pub struct DriverToCircuits;

impl Linked for DriverToCircuits {
    type FromEntity = crate::drivers::Entity;
    type ToEntity = crate::circuits::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            crate::results::Relation::Drivers.def().rev(),
            crate::results::Relation::Races.def(),
            crate::races::Relation::Circuits.def(),
        ]
    }
}

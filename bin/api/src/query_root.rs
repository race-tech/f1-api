use async_graphql::{dynamic::*, Result};
use sea_orm::{
    entity::*, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, RelationTrait,
};
use seaography::{
    apply_order, apply_pagination, async_graphql, get_filter_conditions, get_having_conditions,
    lazy_static::lazy_static, Builder, BuilderContext, CustomOutputType, OrderInputBuilder,
    PaginationInputBuilder,
};

use entities::register_entity_modules;
use macros::{query, FilterInputObject, OutputObject};

lazy_static! {
    static ref CONTEXT: BuilderContext = BuilderContext::default();
}

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    schema_builder(&CONTEXT, database, depth, complexity).finish()
}

pub fn schema_builder(
    context: &'static BuilderContext,
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> SchemaBuilder {
    let mut builder = Builder::new(context, database.clone());
    builder = register_entity_modules(builder);

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
}

fn modify_entity(builder: &mut Builder) {
    let drivers_index = builder
        .outputs
        .iter()
        .position(|o| o.type_name() == "Drivers");

    let context = builder.context;

    if let Some(drivers_index) = drivers_index {
        let drivers = builder.outputs.swap_remove(drivers_index).field(
            Field::new(
                "circuits",
                TypeRef::named_nn("CircuitsConnection"),
                move |ctx| {
                    FieldFuture::new(async move {
                        let Ok(parent) = ctx
                            .parent_value
                            .try_downcast_ref::<entities::drivers::Model>()
                        else {
                            return Err(async_graphql::Error::new(format!(
                                "Failed to downcast object to Drivers",
                            )));
                        };

                        let filters = ctx.args.get(&context.entity_query_field.filters);
                        let filters =
                            get_filter_conditions::<entities::circuits::Entity>(context, filters)?;
                        let having = ctx.args.get(&context.entity_query_field.having);
                        let filters = get_having_conditions::<entities::circuits::Entity>(
                            context, &ctx, filters, having,
                        )?;
                        let order_by = ctx.args.get(&context.entity_query_field.order_by);
                        let order_by = OrderInputBuilder { context }
                            .parse_object::<entities::circuits::Entity>(order_by)?;
                        let pagination = ctx.args.get(&context.entity_query_field.pagination);
                        let pagination =
                            PaginationInputBuilder { context }.parse_object(pagination)?;

                        let stmt = entities::circuits::Entity::find();
                        let stmt = stmt
                            .join(
                                sea_orm::JoinType::LeftJoin,
                                entities::races::Relation::Circuits.def().rev(),
                            )
                            .join(
                                sea_orm::JoinType::LeftJoin,
                                entities::races::Relation::Results.def(),
                            )
                            .join(
                                sea_orm::JoinType::LeftJoin,
                                entities::results::Relation::Drivers.def(),
                            )
                            .filter(filters)
                            .filter(entities::drivers::Column::DriverId.eq(parent.driver_id))
                            .distinct();

                        let stmt = apply_order(stmt, order_by);

                        let db = ctx.data::<DatabaseConnection>()?;

                        let connection = apply_pagination::<entities::circuits::Entity, _>(
                            context, db, stmt, pagination,
                        )
                        .await?;

                        Ok(Some(FieldValue::owned_any(connection)))
                    })
                },
            )
            .argument(InputValue::new(
                "filters",
                TypeRef::named("CircuitsFilterInput"),
            ))
            .argument(InputValue::new(
                "having",
                TypeRef::named("CircuitsHavingInput"),
            ))
            .argument(InputValue::new(
                "orderBy",
                TypeRef::named("CircuitsOrderInput"),
            ))
            .argument(InputValue::new(
                "pagination",
                TypeRef::named("PaginationInput"),
            )),
        );
        builder.outputs.push(drivers);
    }
}

pub trait Object {
    fn type_name() -> &'static str;
}

pub trait InputObject {
    fn gql_input_type_ref(
        ctx: &'static seaography::BuilderContext,
    ) -> async_graphql::dynamic::TypeRef;
    fn to_object(
        context: &'static seaography::BuilderContext,
    ) -> async_graphql::dynamic::InputObject;

    fn build_filter<'a>(
        _context: &'static BuilderContext,
        _ctx: async_graphql::dynamic::ResolverContext<'a>,
    ) -> sea_orm::Condition {
        sea_orm::Condition::all()
    }

    fn register(builder: &mut seaography::Builder) {
        let context = builder.context;
        let input_object = Self::to_object(context);

        builder.inputs.push(input_object);
    }
}

#[derive(FilterInputObject)]
#[allow(dead_code)]
struct DriverCircuitsFilter {
    #[filter_input_object(entity = entities::drivers::Entity, column = entities::drivers::Column::Code)]
    code: String,
    #[filter_input_object(entity = entities::results::Entity, column = entities::results::Column::Position)]
    position: i32,
}

#[query(output_object = DriverCircuitsOutput, output_type = Vec::<DriverCircuitsOutput>, filters = DriverCircuitsFilter)]
async fn drivers_circuits(
    db: &DatabaseConnection,
    condition: sea_orm::Condition,
) -> Result<Vec<DriverCircuitsOutput>> {
    let stmt = entities::drivers::Entity::find()
        .join(
            sea_orm::JoinType::LeftJoin,
            entities::results::Relation::Drivers.def().rev(),
        )
        .join(
            sea_orm::JoinType::LeftJoin,
            entities::results::Relation::Races.def(),
        )
        .join(
            sea_orm::JoinType::LeftJoin,
            entities::races::Relation::Circuits.def(),
        )
        .select_also(entities::circuits::Entity)
        .filter(condition)
        .group_by(entities::drivers::Column::DriverId)
        .distinct();

    let res = stmt.all(db).await?;

    let mut map = std::collections::HashMap::<
        i32,
        (entities::drivers::Model, Vec<entities::circuits::Model>),
    >::new();
    res.into_iter().for_each(|(d, c)| {
        let entry = map.entry(d.driver_id).or_insert_with(|| (d, Vec::new()));
        if let Some(circuit) = c {
            entry.1.push(circuit);
        }
    });

    Ok(map
        .into_values()
        .map(|(driver, circuits)| DriverCircuitsOutput { driver, circuits })
        .collect())
}

#[derive(Clone, OutputObject)]
struct DriverCircuitsOutput {
    driver: entities::drivers::Model,
    circuits: Vec<entities::circuits::Model>,
}

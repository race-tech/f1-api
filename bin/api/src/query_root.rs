use async_graphql::{dynamic::*, Result};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, RelationTrait};
use seaography::{
    async_graphql, lazy_static::lazy_static, Builder, BuilderContext, CustomFields,
    CustomOutputType, GqlModelType,
};

use entities::register_entity_modules;
use macros::{query, FilterInputObject};

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
    builder.mutations.clear();

    // builder.register_custom_output::<DriverCircuitsOutput>();
    //builder.register_custom_query::<Operation>();
    drivers_circuits(&mut builder);

    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
}

pub trait InputObject {
    fn gql_input_type_ref(
        ctx: &'static seaography::BuilderContext,
    ) -> async_graphql::dynamic::TypeRef;
    fn to_object(
        context: &'static seaography::BuilderContext,
    ) -> async_graphql::dynamic::InputObject;

    fn build_filter<'a>(
        context: &'static BuilderContext,
        ctx: async_graphql::dynamic::ResolverContext<'a>,
    ) -> sea_orm::Condition {
        unimplemented!()
    }
    fn register(builder: &mut seaography::Builder) {
        let context = builder.context;
        let input_object = Self::to_object(context);

        builder.inputs.push(input_object);
    }
}

pub trait OutputObject {
    fn to_object(context: &'static BuilderContext) -> async_graphql::dynamic::Object;
    fn gql_output_type_ref(
        ctx: &'static seaography::BuilderContext,
    ) -> async_graphql::dynamic::TypeRef;
    fn register(builder: &mut seaography::Builder) {
        let context = builder.context;
        let output_object = Self::to_object(context);

        builder.outputs.push(output_object);
    }
}

impl<T: OutputObject> OutputObject for Vec<T> {
    fn to_object(_context: &'static BuilderContext) -> async_graphql::dynamic::Object {
        unreachable!()
    }
    fn gql_output_type_ref(
        ctx: &'static seaography::BuilderContext,
    ) -> async_graphql::dynamic::TypeRef {
        async_graphql::dynamic::TypeRef::NonNull(Box::new(async_graphql::dynamic::TypeRef::List(
            Box::new(T::gql_output_type_ref(ctx)),
        )))
    }
}

#[derive(FilterInputObject)]
struct DriverCircuitsFilter {
    #[filter_input_object(entity = entities::drivers::Entity, column = entities::drivers::Column::Code)]
    code: String,
    #[filter_input_object(entity = entities::results::Entity, column = entities::results::Column::Position)]
    position: i32,
}

#[query(output = DriverCircuitsOutput, filters = DriverCircuitsFilter)]
async fn drivers_circuits(db: &DatabaseConnection) -> Result<Vec<DriverCircuitsOutput>> {
    todo!()
}

// fn drivers_circuits(mut builder: Builder) -> Builder {
//     let context = builder.context;
//     let input = async_graphql::dynamic::InputObject::new("DriverCircuitsFilterInput")
//         .field(async_graphql::dynamic::InputValue::new(
//             "code",
//             async_graphql::dynamic::TypeRef::named(
//                 context.filter_types.string_filter_info.type_name.clone(),
//             ),
//         ))
//         .field(async_graphql::dynamic::InputValue::new(
//             "position",
//             async_graphql::dynamic::TypeRef::named(
//                 context.filter_types.integer_filter_info.type_name.clone(),
//             ),
//         ))
//         .field(InputValue::new(
//             "and",
//             TypeRef::named_nn_list("DriverCircuitsFilterInput"),
//         ))
//         .field(InputValue::new(
//             "or",
//             TypeRef::named_nn_list("DriverCircuitsFilterInput"),
//         ))
//         .field(InputValue::new(
//             "not",
//             TypeRef::named("DriverCircuitsFilterInput"),
//         ));
//
//     let query = async_graphql::dynamic::Field::new(
//         "drivers_circuits",
//         async_graphql::dynamic::TypeRef::named_nn_list("DriverCircuitsOutput"),
//         move |ctx| {
//             async_graphql::dynamic::FieldFuture::new(async move {
//                 let filters = ctx.args.get(&context.entity_query_field.filters);
//                 let filter_types_map_helper = seaography::FilterTypesMapHelper { context };
//                 let mut condition = sea_orm::Condition::all();
//
//                 if let Some(filters) = filters.map(|o| o.object().unwrap()) {
//                     if let Some(code) = filters.get("code") {
//                         let code = code.object().expect("cannot convert code to object");
//                         condition = filter_types_map_helper
//                             .prepare_column_condition::<entities::drivers::Entity>(
//                                 condition,
//                                 &code,
//                                 &entities::drivers::Column::Code,
//                             )
//                             .unwrap();
//                     }
//
//                     if let Some(position) = filters.get("position") {
//                         let position = position
//                             .object()
//                             .expect("cannot convert position to object");
//                         condition = filter_types_map_helper
//                             .prepare_column_condition::<entities::results::Entity>(
//                                 condition,
//                                 &position,
//                                 &entities::results::Column::Position,
//                             )
//                             .unwrap();
//                     }
//                 }
//
//                 let stmt = entities::drivers::Entity::find()
//                     .join(
//                         sea_orm::JoinType::LeftJoin,
//                         entities::results::Relation::Drivers.def().rev(),
//                     )
//                     .join(
//                         sea_orm::JoinType::LeftJoin,
//                         entities::results::Relation::Races.def(),
//                     )
//                     .join(
//                         sea_orm::JoinType::LeftJoin,
//                         entities::races::Relation::Circuits.def(),
//                     )
//                     .select_also(entities::circuits::Entity)
//                     .filter(condition)
//                     .distinct();
//
//                 let db = ctx.data::<DatabaseConnection>()?;
//
//                 let res = stmt.all(db).await?;
//
//                 let mut map = std::collections::HashMap::<
//                     i32,
//                     (entities::drivers::Model, Vec<entities::circuits::Model>),
//                 >::new();
//
//                 res.into_iter().for_each(|(d, c)| {
//                     let entry = map.entry(d.driver_id).or_insert_with(|| (d, Vec::new()));
//
//                     if let Some(circuit) = c {
//                         entry.1.push(circuit);
//                     }
//                 });
//
//                 Ok(Some(FieldValue::list(
//                     map.into_values()
//                         .map(Into::into)
//                         .map(|o: DriverCircuitsOutput| FieldValue::owned_any(o)),
//                 )))
//             })
//         },
//     )
//     .argument(async_graphql::dynamic::InputValue::new(
//         "filters",
//         async_graphql::dynamic::TypeRef::named_nn("DriverCircuitsFilterInput"),
//     ));
//
//     builder.inputs.push(input);
//     builder.queries.push(query);
//     builder
// }

// #[derive(Clone, CustomOutputType)]
struct DriverCircuitsOutput {
    driver: entities::drivers::Model,
    circuits: Vec<entities::circuits::Model>,
}

impl OutputObject for DriverCircuitsOutput {
    fn to_object(context: &'static BuilderContext) -> async_graphql::dynamic::Object {
        use seaography::GqlModelHolderType;

        async_graphql::dynamic::Object::new("DriverCircuitsOutput")
            .field(async_graphql::dynamic::Field::new(
                "driver",
                entities::drivers::Model::gql_output_type_ref(context),
                move |ctx| {
                    async_graphql::dynamic::FieldFuture::new(async move {
                        let obj =
                            seaography::try_downcast_ref::<DriverCircuitsOutput>(ctx.parent_value)?;
                        Ok(<entities::drivers::Model>::gql_field_value(
                            obj.driver.clone(),
                            context,
                        ))
                    })
                },
            ))
            .field(async_graphql::dynamic::Field::new(
                "circuits",
                Vec::<entities::circuits::Model>::gql_output_type_ref(context),
                move |ctx| {
                    async_graphql::dynamic::FieldFuture::new(async move {
                        let obj =
                            seaography::try_downcast_ref::<DriverCircuitsOutput>(ctx.parent_value)?;
                        Ok(<Vec<entities::circuits::Model>>::gql_field_value(
                            obj.circuits.clone(),
                            context,
                        ))
                    })
                },
            ))
    }
    fn gql_output_type_ref(
        _ctx: &'static seaography::BuilderContext,
    ) -> async_graphql::dynamic::TypeRef {
        async_graphql::dynamic::TypeRef::named_nn("DriverCircuitsOutput")
    }
}

// impl From<(entities::drivers::Model, Vec<entities::circuits::Model>)> for DriverCircuitsOutput {
//     fn from(value: (entities::drivers::Model, Vec<entities::circuits::Model>)) -> Self {
//         Self {
//             driver: value.0,
//             circuits: value.1,
//         }
//     }
// }
//
// struct Operation;

// #[CustomFields]
// impl Operation {
//     pub async fn driver_circuits(
//         ctx: &async_graphql::Context<'_>,
//         driver_id: i32,
//     ) -> async_graphql::Result<Vec<DriverCircuitsOutput>> {
//         let db = ctx.data::<DatabaseConnection>()?;
//
//         entities::drivers::Entity::find_by_id(driver_id)
//             .find_with_linked(entities::links::DriverToCircuits)
//             .distinct()
//             .all(db)
//             .await
//             .map(|res| res.into_iter().map(Into::into).collect())
//             .map_err(Into::into)
//     }
// }

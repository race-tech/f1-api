use async_graphql::dynamic::{Field, FieldFuture, FieldValue, TypeRef};
use sea_orm::{
    ConnectionTrait, EntityTrait, Iterable, ModelTrait, PrimaryKeyArity, PrimaryKeyToColumn,
    PrimaryKeyTrait, SelectTwo,
};
use seaography::{
    encode_cursor, itertools::Itertools, BuilderContext, CursorInput, PageInfo, PaginationInfo,
    PaginationInput,
};

use crate::query_root::Object;

pub struct Connection<T>
where
    T: Sync,
{
    pub page_info: PageInfo,
    pub pagination_info: Option<PaginationInfo>,
    pub edges: Vec<Edge<T>>,
}

pub struct Edge<T>
where
    T: Sync,
{
    pub cursor: String,
    pub node: T,
}

impl<T> seaography::CustomOutputObject for Connection<T>
where
    T: Object + Send + Sync + 'static,
{
    fn basic_object(context: &'static BuilderContext) -> async_graphql::dynamic::Object {
        let object_name = T::type_name();
        let name = context.connection_object.type_name.as_ref()(object_name);
        let edge_name = context.edge_object.type_name.as_ref()(object_name);

        async_graphql::dynamic::Object::new(name)
            .field(Field::new(
                &context.connection_object.page_info,
                TypeRef::named_nn(&context.page_info_object.type_name),
                |ctx| {
                    FieldFuture::new(async move {
                        let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
                        Ok(Some(FieldValue::borrowed_any(&connection.page_info)))
                    })
                },
            ))
            .field(Field::new(
                &context.connection_object.pagination_info,
                TypeRef::named(&context.pagination_info_object.type_name),
                |ctx| {
                    FieldFuture::new(async move {
                        let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
                        if let Some(value) = connection
                            .pagination_info
                            .as_ref()
                            .map(|pagination_info| FieldValue::borrowed_any(pagination_info))
                        {
                            Ok(Some(value))
                        } else {
                            Ok(FieldValue::NONE)
                        }
                    })
                },
            ))
            .field(Field::new(
                &context.connection_object.nodes,
                TypeRef::named_nn_list_nn(object_name),
                |ctx| {
                    FieldFuture::new(async move {
                        let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
                        Ok(Some(FieldValue::list(connection.edges.iter().map(
                            |edge: &Edge<T>| FieldValue::borrowed_any(&edge.node),
                        ))))
                    })
                },
            ))
            .field(Field::new(
                &context.connection_object.edges,
                TypeRef::named_nn_list_nn(edge_name),
                |ctx| {
                    FieldFuture::new(async move {
                        let connection = ctx.parent_value.try_downcast_ref::<Connection<T>>()?;
                        Ok(Some(FieldValue::list(
                            connection
                                .edges
                                .iter()
                                .map(|edge: &Edge<T>| FieldValue::borrowed_any(edge)),
                        )))
                    })
                },
            ))
    }
}

async fn apply_pagination_no_inputs<L, R, C, F, O>(
    db: &C,
    stmt: SelectTwo<L, R>,
    mut map: F,
) -> Result<Connection<O>, sea_orm::DbErr>
where
    L: EntityTrait,
    <L as EntityTrait>::Model: Sync,
    R: EntityTrait,
    <R as EntityTrait>::Model: Sync,
    C: ConnectionTrait,
    O: Sync,
    F: FnMut((<L as EntityTrait>::Model, Option<<R as EntityTrait>::Model>)) -> O,
{
    let data = stmt.all(db).await?;

    let edges: Vec<Edge<O>> = data
        .into_iter()
        .map(|node| {
            let values = node.0.get_primary_key_value();

            let cursor: String = encode_cursor(values);

            Edge {
                cursor,
                node: map(node),
            }
        })
        .collect();

    let start_cursor = edges.first().map(|edge| edge.cursor.clone());
    let end_cursor = edges.last().map(|edge| edge.cursor.clone());

    let total = edges.len() as u64;

    Ok(Connection {
        edges,
        page_info: PageInfo {
            has_previous_page: false,
            has_next_page: false,
            start_cursor,
            end_cursor,
        },
        pagination_info: Some(PaginationInfo {
            pages: 1,
            current: 1,
            offset: 0,
            total,
        }),
    })
}

async fn apply_pagination_with_cursor<L, R, C, F, O>(
    context: &'static BuilderContext,
    db: &C,
    stmt: SelectTwo<L, R>,
    mut map: F,
    cursor_object: CursorInput,
) -> Result<Connection<O>, sea_orm::DbErr>
where
    L: EntityTrait,
    <L as EntityTrait>::Model: Sync,
    R: EntityTrait,
    <R as EntityTrait>::Model: Sync,
    C: ConnectionTrait,
    O: Sync,
    F: FnMut((<L as EntityTrait>::Model, Option<<R as EntityTrait>::Model>)) -> O,
{
    check_limit(context, cursor_object.limit)?;

    let next_stmt = stmt.clone();
    let previous_stmt = stmt.clone();

    fn apply_stmt_cursor_by<L, R>(
        stmt: SelectTwo<L, R>,
    ) -> Result<sea_orm::Cursor<sea_orm::SelectTwoModel<L::Model, R::Model>>, sea_orm::DbErr>
    where
        L: EntityTrait,
        <L as EntityTrait>::Model: Sync,

        R: EntityTrait,
        <R as EntityTrait>::Model: Sync,
    {
        let size = <<L::PrimaryKey as PrimaryKeyTrait>::ValueType as PrimaryKeyArity>::ARITY;
        if size == 1 {
            let column = L::PrimaryKey::iter()
                .map(|variant| variant.into_column())
                .collect::<Vec<L::Column>>()[0];
            Ok(stmt.cursor_by(column))
        } else if size == 2 {
            let columns = L::PrimaryKey::iter()
                .map(|variant| variant.into_column())
                .collect_tuple::<(L::Column, L::Column)>()
                .expect("infallible as arity is already checked");
            Ok(stmt.cursor_by(columns))
        } else if size == 3 {
            let columns = L::PrimaryKey::iter()
                .map(|variant| variant.into_column())
                .collect_tuple::<(L::Column, L::Column, L::Column)>()
                .expect("infallible as arity is already checked");
            Ok(stmt.cursor_by(columns))
        } else {
            Err(sea_orm::DbErr::Custom(format!(
                "Not supporting primary key with arity > 3: {size}"
            )))
        }
    }

    let mut stmt = apply_stmt_cursor_by(stmt)?;
    let data = stmt.first(cursor_object.limit).all(db).await?;

    let has_next_page: bool = {
        let mut next_stmt = apply_stmt_cursor_by(next_stmt)?;

        let last_node = data.last();

        if let Some(node) = last_node {
            let values = node.0.get_primary_key_value();

            let next_data = next_stmt.first(1).after(values).all(db).await?;

            !next_data.is_empty()
        } else {
            false
        }
    };

    let has_previous_page: bool = {
        let mut previous_stmt = apply_stmt_cursor_by(previous_stmt)?;

        let first_node = data.first();

        if let Some(node) = first_node {
            let values = node.0.get_primary_key_value();

            let previous_data = previous_stmt.first(1).before(values).all(db).await?;

            !previous_data.is_empty()
        } else {
            false
        }
    };

    let edges: Vec<Edge<O>> = data
        .into_iter()
        .map(|node| {
            let values = node.0.get_primary_key_value();

            let cursor: String = encode_cursor(values);

            Edge {
                cursor,
                node: map(node),
            }
        })
        .collect();

    let start_cursor = edges.first().map(|edge| edge.cursor.clone());
    let end_cursor = edges.last().map(|edge| edge.cursor.clone());

    Ok(Connection {
        edges,
        page_info: PageInfo {
            has_previous_page,
            has_next_page,
            start_cursor,
            end_cursor,
        },
        pagination_info: None,
    })
}

pub async fn apply_pagination<L, R, C, F, O>(
    context: &'static BuilderContext,
    db: &C,
    stmt: SelectTwo<L, R>,
    pagination: PaginationInput,
    map: F,
) -> Result<Connection<O>, sea_orm::DbErr>
where
    L: EntityTrait,
    <L as EntityTrait>::Model: Sync,
    R: EntityTrait,
    <R as EntityTrait>::Model: Sync,
    C: ConnectionTrait,
    O: Sync,
    F: FnMut((<L as EntityTrait>::Model, Option<<R as EntityTrait>::Model>)) -> O,
{
    let pagination = apply_pagination_defaults(context, pagination);

    if let Some(cursor_object) = pagination.cursor {
        apply_pagination_with_cursor(context, db, stmt, map, cursor_object).await
    } else {
        apply_pagination_no_inputs(db, stmt, map).await
    }
}

fn apply_pagination_defaults(
    context: &'static BuilderContext,
    pagination: PaginationInput,
) -> PaginationInput {
    // If there are no pagination options supplied, but a default of or maximum limit has been
    // configured, use page-based pagination with a page number of 0 and the lower of the two
    // applied.

    if pagination.cursor.is_some() || pagination.page.is_some() || pagination.offset.is_some() {
        return pagination;
    }

    let opts = &context.pagination_input;
    let use_limit = match (opts.default_limit, opts.max_limit) {
        (None, None) => None,
        (None, Some(max_limit)) => Some(max_limit),
        (Some(default_limit), None) => Some(default_limit),
        (Some(default_limit), Some(max_limit)) => Some(std::cmp::min(default_limit, max_limit)),
    };

    if let Some(use_limit) = use_limit {
        PaginationInput {
            cursor: None,
            offset: None,
            page: Some(seaography::PageInput {
                page: 0,
                limit: use_limit,
            }),
        }
    } else {
        pagination
    }
}

fn check_limit(
    context: &'static BuilderContext,
    requested_limit: u64,
) -> Result<(), sea_orm::DbErr> {
    if requested_limit == 0 {
        return Err(sea_orm::DbErr::Query(sea_orm::RuntimeErr::Internal(
            "Requested pagination limit must be greater than 0".to_string(),
        )));
    }

    if let Some(max_limit) = context.pagination_input.max_limit {
        if requested_limit > max_limit {
            return Err(sea_orm::DbErr::Query(sea_orm::RuntimeErr::Internal(
                format!(
                    "Requested pagination limit ({requested_limit}) exceeds maximum allowed ({max_limit})"
                ),
            )));
        }
    }

    Ok(())
}

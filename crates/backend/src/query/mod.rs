pub mod circuit;

pub trait Query<'a> {
    type Params;
    type BoxedQuery;

    fn by_ref(ref_: &'a str) -> Self::BoxedQuery;
    fn by_params(params: Self::Params) -> Self::BoxedQuery;
}

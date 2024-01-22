use nom::{bytes::complete::take_while1, combinator::map_res, IResult};

#[derive(Debug, PartialEq)]
pub struct Property {
    key: String,
    value: String,
}

#[derive(Debug, PartialEq)]
pub struct SectionItem {
    name: String,
    properties: Vec<Property>,
}

type Section = Vec<SectionItem>;

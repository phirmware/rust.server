use std::collections::hash_map::HashMap;
use std::convert::From;

#[derive(Debug)]
pub enum QueryValue<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

pub struct Query<'a> {
    pub query: HashMap<&'a str, QueryValue<'a>>,
}

impl<'a> From<Option<&'a str>> for Query<'a> {
    // query-string: a=b&b&c===&a=t
    // result: { a: [b, t], b: '', c: '==' }
    fn from(value: Option<&'a str>) -> Self {
        let result = value.map(|query_string| Query::convert_to_query(query_string));
        result.unwrap_or_else(|| Query {
            query: HashMap::new(),
        })
    }
}

impl<'a> Query<'a> {
    fn convert_to_query(query_string: &str) -> Query {
        let mut q_map = HashMap::new();

        for (_, q) in query_string.split("&").enumerate() {
            let mut key = q;
            let value: &str;
            if let Some(i) = q.find("=") {
                key = &q[..i];
                value = &q[(i + 1)..];
            } else {
                value = "";
            }

            q_map
                .entry(key)
                .and_modify(|v| Query::modify_query_value(v, value))
                .or_insert(QueryValue::Single(value));
        }

        Query { query: q_map }
    }

    fn modify_query_value(v: &mut QueryValue<'a>, val: &'a str) {
        match v {
            QueryValue::Multiple(value) => value.push(val),
            QueryValue::Single(value) => {
                *v = QueryValue::Multiple(vec![value, val]);
            }
        }
    }
}

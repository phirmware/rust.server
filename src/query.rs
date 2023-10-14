use std::collections::hash_map::HashMap;
use std::convert::From;

#[derive(Debug)]
pub enum QueryValue {
    Single(String),
    Multiple(Vec<String>),
}

pub struct Query {
    pub query: HashMap<String, QueryValue>,
}

impl From<Option<&str>> for Query {
    // query-string: a=b&b&c===&a=t
    // result: { a: [b, t], b: '', c: '==' }
    fn from(value: Option<&str>) -> Self {
        let result = value.map(|query_string| Query::convert_to_query(query_string));
        result.unwrap_or_else(|| Query {
            query: HashMap::new(),
        })
    }
}

impl Query {
    fn convert_to_query(query_string: &str) -> Query {
        let mut q_map = HashMap::new();

        for (_, q) in query_string.split("&").enumerate() {
            println!("{}", q);

            let mut key = q;
            let value: &str;
            if let Some(i) = q.find("=") {
                key = &q[..i];
                value = &q[(i + 1)..];
            } else {
                value = "";
            }

            q_map
                .entry(key.to_string())
                .and_modify(|v| Query::modify_query_value(v, value))
                .or_insert(QueryValue::Single(value.to_string()));
        }

        Query { query: q_map }
    }

    fn modify_query_value(v: &mut QueryValue, val: &str) {
        match v {
            QueryValue::Multiple(value) => value.push(val.to_string()),
            QueryValue::Single(value) => {
                *v = QueryValue::Multiple(vec![value.to_string(), val.to_string()]);
            }
        }
    }
}

use serde::{Serialize, Deserialize};
use serde_json::value::Serializer;


#[derive(Debug, Serialize, Deserialize)]
enum Field<T> {
    Default,
    Value(T),
}

impl<T> Default for Field<T> {
    fn default() -> Self {
        Self::Default
    }
}

impl<T> Field<T> {
    fn is_default(&self) -> bool {
        match self {
            Self::Default => true,
            Self::Value(_) => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    #[serde(skip_serializing_if="Field::is_default", default)]
    x: Field<i32>,
    #[serde(skip_serializing_if="Field::is_default", default)]
    y: Field<i32>,
    #[serde(skip_serializing_if="Field::is_default", default)]
    name: Field<String>,
}


fn main() {
    let point = Point{x: Field::Value(1), y: Field::Default, name: Field::Default};
    let res = point.serialize(Serializer).unwrap();
    println!("{:?}", res.to_string());

    let point: Point = serde_json::from_value(res).unwrap();
    println!("{:?}", point);
}

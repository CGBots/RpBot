use mongodb::bson::oid::ObjectId;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum StatValue {
    Int(u32),
    Float(f32),
    Text(String),
    Bool(bool)
}

impl StatValue {
    pub fn to_dynamic(&self) -> rhai::Dynamic {
        match self {
            StatValue::Int(v) => rhai::Dynamic::from(*v as i64),
            StatValue::Float(v) => rhai::Dynamic::from(*v as f64),
            StatValue::Text(v) => rhai::Dynamic::from(v.clone()),
            StatValue::Bool(v) => rhai::Dynamic::from(*v),
        }
    }
}


#[derive(Clone, Debug)]
pub struct Stat {
    pub name: String,
    pub base_value: StatValue,
    pub formula: Option<String>,
    pub min: Option<StatValue>,
    pub max: Option<StatValue>,
    pub modifiers: Vec<Modifier>
}

impl Stat {
    pub fn is_within_bounds(&self) -> bool {
        if let Some(min) = &self.min {
            if self.base_value < *min {
                return false;
            }
        }
        if let Some(max) = &self.max {
            if self.base_value > *max {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, Debug)]
struct Modifier{
    priority: i32,
    stat: ObjectId,
    variable_name: String,
    value: StatValue,
    formula: String,
    end_timestamp: Option<u64>,
}

use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use super::schema::todos;
use chrono::serde::ts_seconds::serialize as to_ts;
use chrono::serde::ts_seconds::deserialize as from_ts;

#[derive(Queryable, Clone, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Todo {
    pub id: i32,
    pub value: String,

    #[serde(serialize_with = "to_ts", deserialize_with = "from_ts")]
    pub create_at: DateTime<Utc>,
    #[serde(serialize_with = "to_ts", deserialize_with = "from_ts")]
    pub update_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(value: String) -> Todo {
       Todo { id: 0, value: value, create_at: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc), update_at: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc) }
    }

    pub fn execute_validation(&self) -> Option<Vec<String>> {
      let mut errors: Vec<String> = Vec::new();

      if self.value.chars().count() > 200 {
         errors.push(String::from("内容は200文字以下で入力してください。"));
      }

      if errors.is_empty() {
         None
      } else {
         Some(errors)
      }
    }
}

#[derive(Insertable)]
#[table_name="todos"]
pub struct NewTodo<'a> {
    pub value: &'a str,
}
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    #[serde(rename = "username")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    // #[serde(rename = "st_id")]
    pub student_id: String,
    #[serde(skip)]
    pub birthday: NaiveDate,
    pub age: u8,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub pets: Vec<String>,
    #[serde(flatten)]
    pub address: Address,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Address {
    pub country: String,
    pub city: String,
}

#[derive(Debug, Deserialize, Serialize)]
// #[serde(untagged)]
#[serde(tag = "type", content = "student")]
pub enum StudentType {
    #[serde(rename = "studentA")]
    StudentWithLabel(Student),
    #[serde(rename = "studentB")]
    StudentNoLabel(Student),
}

#[test]
fn test_se() -> anyhow::Result<()> {
    let address = Address {
        country: "中国".to_string(),
        city: "杭州".to_string(),
    };
    let st = Student {
        name: "王志飞".to_string(),
        middle_name: None,
        student_id: "001".to_string(),
        birthday: NaiveDate::parse_from_str("1990-08-15", "%Y-%m-%d")?,
        age: 34,
        pets: vec![],
        address,
    };
    println!("st:{:#?}", st);
    // 两种获取序列化后字符串的方式
    // 1.使用serde_json::json!(value).to_string() 2.使用serde_json::to_string().unwrap()
    // 使用json宏序列化，会将字段按照字段名进行排序，排序是字典顺序。
    // 序列后的json字符串中没有空格，非常紧凑。另外字段名也需要用引号括起来
    let st_str1 = serde_json::json!(st).to_string();
    println!("st序列化后的字符串st_str1:{}", st_str1);

    let st1 = serde_json::from_slice::<Student>(st_str1.as_bytes())?;
    println!("反序列化后的结构体st1:{:?}", st1);
    // assert_eq!(st_str1, r#"{"id":"001","name":"王志飞"}"#);

    // 使用模块级别的函数to_string()不会对字段进行重新排序，按照结构体定义的顺序进行。
    let st_str2 = serde_json::to_string(&st).unwrap();
    println!("st序列化后的字符串st_str2:{}", st_str2);

    let st2 = serde_json::from_str::<Student>(&st_str2)?;
    println!("反序列化后的结构体st2:{:?}", st2);
    // assert_eq!(st_str2, r#"{"name":"王志飞","id":"001"}"#);

    // 序列后字段和结构体字段名不一样
    // 使用#[serde( rename = "序列化后的名字")]注解改名的字段

    // 序列后的字段命名风格不一样
    // 使用#[serde(rename_all="camelCase")]将snake-case风格的命名改成驼峰命名风格
    // 如果字段上面已经做了重命名，那么会被忽略
    // 可选的命名风格： snake_case, camelCase, lowercase, UPPERCASE, PascalCase, kebab-case

    // 如果某个字段不想序列化和反序列化，可以加#[serde(skip)]
    // 不想序列化的原因：该字段类型可能没有实现Serialize和Deserialize trait，
    // 但是该类型需要实现Default trait，否则反序列化后该字段无法赋值

    let st_ty_list = vec![
        StudentType::StudentWithLabel(st.clone()),
        StudentType::StudentNoLabel(st.clone()),
    ];
    println!("{:?}", st_ty_list);
    let list_str = serde_json::to_string::<Vec<_>>(&st_ty_list)?;
    println!("列表序列化后list_str:{}", list_str);

    Ok(())
}

#[test]
fn test_value() -> anyhow::Result<()> {
    let address = Address {
        country: "中国".to_string(),
        city: "杭州".to_string(),
    };

    let st = Student {
        name: "王志飞".to_string(),
        middle_name: None,
        student_id: "001".to_string(),
        birthday: NaiveDate::parse_from_str("1990-08-15", "%Y-%m-%d")?,
        age: 34,
        pets: vec![],
        address,
    };

    let mut st_value = json!(st);
    println!("st_vlue:{:#?}", st_value);
    let st_value_take = st_value.take();
    println!("st_value_take:{:?}", st_value_take);

    println!("st_value:{:?}",st_value);
    let age_value = st_value.get("age");
    println!("age_value:{:?}", age_value);

    let mut v = json!({ "x": "y" });
    assert_eq!(v["x"].take(), json!("y"));
    assert_eq!(v, json!({ "x": null }));
    Ok(())
}
#[derive(Debug, Deserialize, Serialize)]
enum Lang {
    Rust,
    Java,
    Python,
}

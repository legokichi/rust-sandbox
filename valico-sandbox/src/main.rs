use valico::json_dsl;
use serde_json::to_string_pretty;
use valico::json_schema;
use failure::Fail;
fn main() -> Result<(), ::failure::Error> {
    let mut json_v4_schema: ::serde_json::Value = ::serde_json::from_str(r#"{
    "type": "object",
    "properties": {
        "user": {
            "type": "object",
            "properties": {
                "name": {
                    "type": "string"
                }
            },
            "required": ["name"]
        }
    },
    "required": ["user"]
}"#)?;
    println!("uo");
    let mut scope = json_schema::Scope::new();
    let schema = scope.compile_and_return(json_v4_schema.clone(), false).map_err(|e| failure::err_msg(format!("{:?}", e)))?;
    println!("uo uo");
    let mut valid_json: ::serde_json::Value = ::serde_json::from_str(r#"{
    "user": {
        "name": "korosuzo"
    }
}"#)?;
    let mut unvalid_json: ::serde_json::Value = ::serde_json::from_str(r#"{
    "users": [
        {
            "user": {
                "name": "korosuzo"
            }
        }
    ]
}"#)?;
    println!("uo uo fish");
    assert!(schema.validate(&valid_json).is_valid());
    assert!(!schema.validate(&unvalid_json).is_valid());
    println!("uo uo fish life");
    Ok(())
}

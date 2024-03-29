use crate::ast::TsType;

pub fn build(ts_type: TsType) -> String {
    match ts_type {
        TsType::Basic(b) => b,
        TsType::Array(t) => format!("{}[]", build(*t)),
        TsType::Map(key_type, value_type) => {
            format!("Map<{}, {}>", build(*key_type), build(*value_type))
        }
        TsType::Union(types) => {
            let mut types_str: Vec<String> = types.into_iter().map(|t| build(t)).collect();
            types_str.sort_by(|a, b| b.len().cmp(&a.len()));
            types_str.join(" | ")
        }
    }
}

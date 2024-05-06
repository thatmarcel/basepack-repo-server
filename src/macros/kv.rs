#[macro_export]
macro_rules! kv {
    ( $($tt:tt)+ ) => {{
        let mut object = mongodb::bson::Document::new();
        mongodb::bson::bson!(@object object () ($($tt)+) ($($tt)+));
        let mut result = String::new();
        for kvp in object {
            let value = match kvp.1.as_str() {
                Some(s) => s.to_string(),
                None => match kvp.1.as_i64() {
                    Some(i) => i.to_string(),
                    None => match kvp.1.as_bool() {
                        Some(b) => b.to_string(),
                        None => String::new()
                    }
                }
            };

            match &*value {
                "" => {},
                v => {
                    result.push_str(format!("{}: {}\n", kvp.0, v).as_str());
                }
            };
        }
        result.trim().to_string()
    }};
}
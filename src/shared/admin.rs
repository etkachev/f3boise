/// get list of admin users (slack ids)
pub fn admin_users() -> Vec<String> {
    vec![
        // backslash
        backslash_id(),
        // stinger
        stinger_id(),
        // guac
        String::from("U04SS5FQXQ9"),
        // deepdish
        String::from("U05CWQ1FSV8"),
    ]
}

pub fn backslash_id() -> String {
    String::from("U03SR452HL7")
}

pub fn stinger_id() -> String {
    String::from("U03T87KHRFE")
}

#![allow(dead_code)]
pub fn collapse(s: &str) -> String {
    let mut collapsed = String::new();
    for c in s.chars() {
        match c {
            ' ' | '\n' | '\r' => continue,
            _ => collapsed.push(c),
        }
    }
    collapsed
}

/*
#[test]
pub fn test_template() {
    let json = collapse(
        r#""#,
    );
    let structure: Current = serde_json::from_str(json.as_str()).expect("failed to stringify");
    assert_eq!(
        json,
        serde_json::to_string(&structure).expect("failed to stringify structure")
    );
}
*/

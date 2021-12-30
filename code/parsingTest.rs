#[test]
fn parsing_multiple() {
    let sql = "select * from users where name=${req.name} and age=${req.age} or name = upper(${req.name})";
    let parsed = SqlWithVariables::from_sql(sql).unwrap();

    assert_eq!(
        &parsed.sql,
        "select * from users where name=$1 and age=$2 or name = upper($3)"
    );
    assert_eq!(&parsed.variables, &vec!["req.name", "req.age", "req.name"]);
}

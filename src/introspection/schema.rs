use postgres::{Error, Transaction};

#[derive(Debug, PartialEq)]
pub struct Schema {
    pub name: String,
}

pub fn introspect_schemas(
    client: &mut Transaction,
    schemas: &[&str],
) -> Result<Vec<Schema>, Error> {
    let query = include_str!("schema.sql");
    let stmt = client.prepare(query)?;
    let rows = client.query(&stmt, &[&schemas])?;

    let mut vals = Vec::new();
    for row in rows {
        let val = Schema { name: row.get(0) };
        vals.push(val);
    }
    Ok(vals)
}

#[cfg(test)]
mod tests {
    use super::introspect_schemas;
    use crate::{introspection::schema::Schema, util::test::get_test_connection};

    #[test]
    fn test_introspect_schemas() {
        let mut conn = get_test_connection();
        let mut tx = conn.transaction().unwrap();
        tx.execute("CREATE SCHEMA test_schema", &[]).unwrap();
        let res = introspect_schemas(&mut tx, &vec!["test_schema"]).unwrap();

        assert_eq!(
            res,
            vec![Schema {
                name: "test_schema".to_string()
            }]
        );
    }
}

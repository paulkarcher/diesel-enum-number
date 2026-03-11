use diesel::prelude::*;
use diesel_enum_number::diesel_enum_number;

#[diesel_enum_number]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active = 1,
    Inactive = 2,
}

diesel::table! {
    items (id) {
        id -> Integer,
        status -> SmallInt,
    }
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = items)]
struct Item {
    id: i32,
    status: Status,
}

fn connection() -> PgConnection {
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set to run integration tests");
    PgConnection::establish(&url).expect("Failed to connect to database")
}

fn setup(conn: &mut PgConnection) {
    diesel::sql_query(
        "CREATE TEMP TABLE items (id INT NOT NULL, status SMALLINT NOT NULL)",
    )
    .execute(conn)
    .unwrap();
}

#[test]
fn roundtrip() {
    let conn = &mut connection();
    setup(conn);

    diesel::insert_into(items::table)
        .values(&Item { id: 1, status: Status::Active })
        .execute(conn)
        .unwrap();

    let loaded = items::table.first::<Item>(conn).unwrap();
    assert_eq!(loaded.id, 1);
    assert_eq!(loaded.status, Status::Active);
}

#[test]
fn roundtrip_all_variants() {
    let conn = &mut connection();
    setup(conn);

    let rows = vec![
        Item { id: 1, status: Status::Active },
        Item { id: 2, status: Status::Inactive },
    ];

    diesel::insert_into(items::table)
        .values(&rows)
        .execute(conn)
        .unwrap();

    let loaded = items::table.order(items::id.asc()).load::<Item>(conn).unwrap();
    assert_eq!(loaded[0].status, Status::Active);
    assert_eq!(loaded[1].status, Status::Inactive);
}

#[test]
fn filter_by_enum_value() {
    let conn = &mut connection();
    setup(conn);

    let rows = vec![
        Item { id: 1, status: Status::Active },
        Item { id: 2, status: Status::Inactive },
        Item { id: 3, status: Status::Active },
    ];

    diesel::insert_into(items::table)
        .values(&rows)
        .execute(conn)
        .unwrap();

    let active = items::table
        .filter(items::status.eq(Status::Active))
        .load::<Item>(conn)
        .unwrap();

    assert_eq!(active.len(), 2);
    assert!(active.iter().all(|i| i.status == Status::Active));
}

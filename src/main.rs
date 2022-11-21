use lmdb::Cursor;
use lmdb::Transaction;

fn main() {
    let path = std::path::Path::new("./foo");
    // std::fs::create_dir_all(path).unwrap();
    let command: &str = &std::env::args().nth(1).unwrap();

    let env = lmdb::Environment::new().open(path).unwrap();
    let db = env.open_db(None).unwrap();

    match command {
        "put" => {
            let key = scru128::new();

            let mut txn = env.begin_rw_txn().unwrap();
            txn.put(
                db,
                &key.to_u128().to_ne_bytes(),
                &"bar",
                lmdb::WriteFlags::empty(),
            )
            .unwrap();
            txn.commit().unwrap();
        }

        "cat" => {
            let txn = env.begin_ro_txn().unwrap();
            let c = txn.open_ro_cursor(db).unwrap();
            let (key, value) = c.get(None, None, lmdb_sys::MDB_LAST).unwrap();
            let key = scru128::Scru128Id::from_u128(u128::from_ne_bytes(
                key.unwrap().try_into().unwrap(),
            ));
            let value = std::str::from_utf8(value).unwrap();
            println!("{:?} {:?} {:?}", key.timestamp(), value, lmdb_sys::MDB_LAST);
        }

        _ => panic!(),
    }
}

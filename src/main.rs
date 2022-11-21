use lmdb::Transaction;

fn main() {
    let path = std::path::Path::new("./foo");
    std::fs::create_dir_all(path).unwrap();
    let env = lmdb::Environment::new().open(path).unwrap();
    let db = env.open_db(None).unwrap();
    /*
    {
        let mut txn = env.begin_rw_txn().unwrap();
        txn.put(db, &"foo", &"bar", lmdb::WriteFlags::empty())
            .unwrap();
        txn.commit().unwrap();
    }
    */

    let txn = env.begin_ro_txn().unwrap();
    let value = txn.get(db, &"foo").unwrap();
    let value = std::str::from_utf8(value).unwrap();
    println!("{}", value);
}

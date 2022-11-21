use lmdb::Cursor;
use lmdb::Transaction;

fn main() {
    let path = std::path::Path::new("./foo");
    std::fs::create_dir_all(path).unwrap();
    let command: &str = &std::env::args().nth(1).unwrap();

    let env = lmdb::Environment::new().open(path).unwrap();
    let db = env.open_db(None).unwrap();

    match command {
        "put" => {
            let key = scru128::new();
            let mut txn = env.begin_rw_txn().unwrap();
            txn.put(
                db,
                // if I understand the docs right, this should be 'to_ne_bytes', but that doesn't
                // work
                &key.to_u128().to_be_bytes(),
                &"bar",
                lmdb::WriteFlags::empty(),
            )
            .unwrap();
            txn.commit().unwrap();
            println!("{:?}", key);
        }

        "cat" => {
            let mut last_key: Option<&[u8]> = None;

            loop {
                let txn = env.begin_ro_txn().unwrap();
                {
                    let mut c = txn.open_ro_cursor(db).unwrap();
                    let i = match last_key {
                        Some(key) => {
                            let mut i = c.iter_from(key);
                            i.next();
                            i
                        }
                        None => c.iter_start(),
                    };

                    for item in i {
                        let (key, value) = item.unwrap();
                        last_key = Some(key);
                        let key = scru128::Scru128Id::from_u128(u128::from_be_bytes(
                            key.try_into().unwrap(),
                        ));
                        let value = std::str::from_utf8(value).unwrap();
                        println!("{:?} {:?} {:?}", key.timestamp(), key, value);
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        }

        _ => panic!(),
    }
}

use common_lib::print_hello;
use rocksdb::Writable;
use rocksdb::DB;



fn main() {
    print_hello();
    let db = DB::open_default("/data2/michael_devtidb2/github/tonesnake/tmp/rocksdb/db1").unwrap();
    let res = db.put(b"my key", b"my value");
    match res {
        Ok(_) => (),
        Err(err) => {
            println!("Put kv pair failed, {}", err);
            return;
        }
    }
    match db.get(b"my key") {
        Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    let res = db.delete(b"my key");
    match res {
        Ok(_) => (),
        Err(err) => {
            println!("Delete key failed, {}", err);
            return;
        }
    }
}

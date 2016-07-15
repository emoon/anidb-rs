extern crate anidb;

mod mock_server;

use anidb::Anidb;
use std::thread;
use std::time::Duration;
use mock_server::MockServer;

#[test]
fn it_works_1() {
    let port: u16 = 4444u16;

    thread::spawn(move || {
        let server = MockServer::new(port).unwrap();
        server.update();
    });

    thread::sleep(Duration::from_millis(200));

    let mut db = Anidb::new(("127.0.0.1", port)).unwrap();

    db.login("foo", "bar").unwrap();
    db.logout().unwrap();
}

#[test]
fn it_works_2() {
}

#[test]
fn it_works_3() {
}



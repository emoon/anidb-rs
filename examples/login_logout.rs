extern crate anidb;

use anidb::{Anidb, Result};

//
// ----- YOU NEED TO CHANGE THIS FOR THE EXAMPLE TO WORK <----
//

static USERNAME: &'static str = "user";
static PASSWORD: &'static str = "pass";

fn login_logout() -> Result<()> {
    let mut db = Anidb::new(("api.anidb.net", 9000))?;
    db.login(USERNAME, PASSWORD)?;
    db.wait_exec_command(500);  // wait 500 ms
    db.logout()?;
    println!("Evenything went ok!");
    Ok(())
}

fn main() {
    login_logout().unwrap_or_else(|e| {
        println!("Failed to login_logout {:?}", e);
    });
}

mod model_viewer;
mod main_loop;
mod viewer;
mod fps;
mod config;

use config::Config;

use crate::cli::Args;
use crate::db::Database;
use crate::connection::{Connection, ConnectionOptions};
use crate::errors::Result;

pub fn main(args: &Args) -> Result<()> {
    let config = Config::from_cli_args(args)?;
    let db = Database::from_cli_args(args)?;
    db.print_status();

    if db.models.len() == 0 {
        println!("No models, nothing to do!");
        return Ok(());
    }

    let conn_options = ConnectionOptions::from_cli_args(args);
    let conn = Connection::build(&db, conn_options);

    // Print the controls
    println!("{}", viewer::CONTROL_HELP);

    main_loop::main_loop(&config, db, conn);

    Ok(())
}

use clap::{ArgMatches, Parser};

use crate::ReplContext;

use super::{ReplCommand, ReplResult};

#[derive(Debug, Parser)]
pub struct SqlOpts {
    #[arg(short, long, help = "The SQL query")]
    pub query: String,
}

pub fn sql(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let query = args
        .get_one::<String>("query")
        .expect("expect query")
        .to_string();
    let cmd = SqlOpts::new(query).into();
    let _ = ctx.send(cmd);
    Ok(None)
}

impl From<SqlOpts> for ReplCommand {
    fn from(opts: SqlOpts) -> Self {
        ReplCommand::Sql(opts)
    }
}

impl SqlOpts {
    pub fn new(query: String) -> Self {
        Self { query }
    }
}

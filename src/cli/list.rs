use clap::ArgMatches;

use crate::ReplContext;

use super::{ReplCommand, ReplResult};

pub fn list(_args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let _ = ctx.send(ReplCommand::List);
    Ok(None)
}

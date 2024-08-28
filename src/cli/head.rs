use clap::{ArgMatches, Parser};

use crate::ReplContext;

use super::{ReplCommand, ReplResult};

#[derive(Debug, Parser)]
pub struct HeadOpts {
    #[arg(short, long, help = "The name of dataset")]
    pub name: String,
    #[arg(short, long, help = "The number of rows to show")]
    pub n: Option<usize>,
}
pub fn head(args: ArgMatches, ctx: &mut ReplContext) -> ReplResult {
    let name = args
        .get_one::<String>("name")
        .expect("expect name")
        .to_string();

    let n = args.get_one::<usize>("n").copied();
    let cmd = HeadOpts::new(name, n).into();
    let _ = ctx.send(cmd);
    Ok(None)
}
impl From<HeadOpts> for ReplCommand {
    fn from(value: HeadOpts) -> Self {
        ReplCommand::Head(value)
    }
}
impl HeadOpts {
    pub fn new(name: String, n: Option<usize>) -> Self {
        Self { name, n }
    }
}

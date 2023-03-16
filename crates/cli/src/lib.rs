use anyhow::Result;
use clap::Parser;
use cmd::{build::Build, create::Create, init::Init, publish::Publish, run::Run, test::Test};

pub mod cmd;
pub mod template;

#[derive(Parser)]
#[clap(author, version)]
pub struct Arguments {
    #[clap(long, short, global = true)]
    pub package: Option<String>,
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    Init(Init),
    Create(Create),
    Build(Build),
    Publish(Publish),
    Test(Test),
    Run(Run),
}

pub async fn run_cli() -> Result<()> {
    let args = Arguments::parse();

    match args.cmd {
        Command::Init(c) => c.execute(),
        Command::Create(c) => c.execute(),
        Command::Build(c) => c.execute(args.package).await,
        Command::Publish(c) => c.execute(args.package).await,
        Command::Test(c) => c.execute(args.package).await,
        Command::Run(c) => c.execute(args.package).await,
    }
}

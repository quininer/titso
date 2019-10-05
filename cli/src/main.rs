mod util;
mod db;

use std::{ fs, mem };
use std::path::PathBuf;
use rand::rngs::OsRng;
use futures_executor::block_on;
use structopt::StructOpt;
use directories::ProjectDirs;
use ttyaskpass::AskPass;
use titso_core::Titso;
use db::RkvStore;

type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;
type AnyResult<T> = std::result::Result<T, AnyError>;


#[derive(StructOpt)]
struct Options {
    #[structopt(short="c")]
    db: Option<PathBuf>,

    #[structopt(subcommand)]
    action: Action,

    tags: Vec<String>
}

#[derive(StructOpt)]
enum Action {
    Init,
    Get {
        password: bool,
        note: bool,
        rule: bool
    },
    Put {
        fixed: Option<String>,
        length: Option<usize>,
        count: Option<usize>,
        note: bool
    },
    Del,
    Import,
    Export
}

async fn start() -> AnyResult<()> {
    let options = Options::from_args();

    let db_path = options.db
        .or_else(|| ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .map(|dir| dir.config_dir().to_owned())
        )
        .ok_or(util::msg("Can't find directory"))?;


    if let Action::Init = options.action {
        fs::create_dir_all(&db_path)?;
    }

    let db = RkvStore::new(&db_path)?;
    let mut cli = AskPass::new(vec![0; 256].into_boxed_slice());
    let pass = cli.askpass("Password:")?;

    match options.action {
        Action::Init => {
            Titso::init(db, &mut OsRng, pass).await?;
        },
        Action::Get { password, note, rule } => {
            Titso::open(db, pass).await?;
        },
        Action::Put { fixed, length, count, note } => (),
        Action::Del => (),
        Action::Import => (),
        Action::Export => ()
    }

    // TODO

    Ok(())
}

fn main() -> AnyResult<()> {
    block_on(start())
}

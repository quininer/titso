mod util;
mod db;
mod hint;

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

    #[structopt(long="init")]
    init: bool,

    #[structopt(short="p")]
    pretag: Option<String>,
}

enum State {
    Start,
    Tag(String),
    Put(String)
}

impl State {
    pub fn prompt(&self) -> String {
        match self {
            State::Start => ">> ".to_owned(),
            State::Tag(tags) => format!("({}) >> ", tags),
            State::Put(tags) => format!("put({}) >> ", tags)
        }
    }
}

async fn start() -> AnyResult<()> {
    let options = Options::from_args();

    let db_path = options.db
        .or_else(|| ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .map(|dir| dir.config_dir().to_owned())
        )
        .ok_or(util::msg("Can't find directory"))?;

    if options.init {
        fs::create_dir_all(&db_path)?;
    }

    let mut rng = OsRng;
    let db = RkvStore::new(&db_path)?;
    let mut cli = AskPass::new(vec![0; 256].into_boxed_slice());
    let pass = cli.askpass("Password:")?;

    let mut titso = if options.init {
        Titso::init(db, &mut rng, pass).await?;
    } else {
        Titso::open(db, pass).await?;
    };

    let mut rl = rustyline::Editor::<()>::new();
    let mut state = State::Start;

    while let Ok(command) = rl.readline(state.prompt().as_str()) {
        rl.add_history_entry(&command);

        match mem::replace(&mut state, State::Start) {
            State::Start => if !command.trim().is_empty() {
                state = State::Tag(command);
            },
            State::Tag(tags) => match command.as_str() {
                "get" => (),
                "put" => state = State::Put(tags),
                "del" => (),
                "q" | "quit" => state = State::Start,
                _ => ()
            },
            State::Put(tags) => ()
        }
    }

    Ok(())
}

fn main() -> AnyResult<()> {
    block_on(start())
}

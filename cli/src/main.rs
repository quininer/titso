mod util;
mod db;

use std::fs;
use std::io::{ self, Read, Write };
use std::path::PathBuf;
use rand::rngs::OsRng;
use futures_executor::block_on;
use structopt::StructOpt;
use directories::ProjectDirs;
use ttyaskpass::AskPass;
use titso_core::{ Titso, packet };
use db::RkvStore;

type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;
type AnyResult<T> = std::result::Result<T, AnyError>;


#[derive(StructOpt)]
struct Options {
    #[structopt(short)]
    db: Option<PathBuf>,

    #[structopt(short)]
    password: Option<String>,

    #[structopt(subcommand)]
    action: Action,

    tags: Vec<String>
}

#[derive(StructOpt)]
enum Action {
    Init,
    Get {
        #[structopt(long)]
        no_password: bool,
        #[structopt(long)]
        rule: bool,
        #[structopt(long)]
        note: bool,
    },
    Put {
        #[structopt(long)]
        no_password: bool,
        #[structopt(long)]
        fixed: Option<String>,
        #[structopt(long)]
        chars: Option<String>,
        #[structopt(long)]
        length: Option<u16>,
        #[structopt(long)]
        count: Option<u64>,
        #[structopt(long)]
        note: bool
    },
    Del,
    Hint,
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

    let pass = if let Some(pass) = options.password
        .as_ref()
        .map(String::as_bytes)
    {
        pass
    } else {
        cli.askpass("Password:")?
    };

    let mut stdout = io::stdout();

    match options.action {
        Action::Init => {
            Titso::init(db, &mut OsRng, pass).await?;
        },
        Action::Get { no_password, rule, note } => {
            let mut titso = Titso::open(db, pass).await?;

            let tag = titso.tag(&options.tags);
            let item = titso.get(tag).await?
                .ok_or(util::msg("Tag not found or Password wrong"))?;

            if !no_password {
                match &item.password {
                    packet::Type::Derive(rule) =>
                        writeln!(&mut stdout, "{}", titso.derive(tag, rule))?,
                    packet::Type::Fixed(pass) => writeln!(&mut stdout, "{}", pass)?
                }
            }

            if let (packet::Type::Derive(rule), true) = (&item.password, rule) {
                writeln!(&mut stdout, "count: {}", rule.count)?;
                writeln!(&mut stdout, "chars: {:?}", rule.chars)?;
                writeln!(&mut stdout, "length: {}", rule.length)?;
            }

            if note {
                stdout.write_all(&item.note)?;
            }
        },
        Action::Put { no_password, fixed, length, chars, count, note } => {
            let mut titso = Titso::open(db, pass).await?;

            let tag = titso.tag(&options.tags);
            let count = count.unwrap_or(0);
            let chars = chars.unwrap_or_else(|| titso_core::chars!{
                numeric,
                alphabet_lowercase,
                alphabet_uppercase,
                punctuation_simple
            }.to_string());
            let length = length
                .unwrap_or_else(|| titso_core::suggest(chars.len()) as u16);

            let password = fixed
                .map(packet::Type::Fixed)
                .unwrap_or_else(move || packet::Type::Derive(packet::Rule {
                    count,
                    chars,
                    length
                }));
            let item = packet::Item {
                password,
                note: if note {
                    let mut buf = Vec::new();
                    io::stdin().read_to_end(&mut buf)?;
                    buf
                } else {
                    Vec::new()
                }
            };

            titso.put(tag, &item).await?;

            if !no_password {
                match &item.password {
                    packet::Type::Derive(rule) =>
                        writeln!(&mut stdout, "{}", titso.derive(tag, rule))?,
                    packet::Type::Fixed(pass) => writeln!(&mut stdout, "{}", pass)?
                }
            }
        },
        Action::Del => {
            let mut titso = Titso::open(db, pass).await?;

            let tag = titso.tag(&options.tags);
            if titso.del(tag).await? {
                // TODO
            }
        },
        Action::Hint => (),
        Action::Import => (),
        Action::Export => ()
    }

    // TODO

    Ok(())
}

fn main() -> AnyResult<()> {
    block_on(start())
}

mod util;
mod titso;

use std::fs;
use std::io::{ self, Read, Write };
use std::path::PathBuf;
use anyhow::anyhow;
use rkv::Rkv;
use structopt::StructOpt;
use directories::ProjectDirs;
use ttyaskpass::AskPass;
use titso_core::packet::*;
use crate::util::StoreError;
use crate::titso::Titso;


#[derive(StructOpt)]
#[structopt(setting(structopt::clap::AppSettings::DeriveDisplayOrder))]
struct Options {
    #[structopt(short, long)]
    db: Option<PathBuf>,

    #[structopt(short, long)]
    password: Option<String>,

    #[structopt(subcommand)]
    action: Action,

    #[structopt(last(true), global(true))]
    tags: Vec<String>
}

#[derive(StructOpt)]
enum Action {
    Init,
    Get {
        #[structopt(long)]
        no_password: bool,
        #[structopt(short, long)]
        rule: bool,
        #[structopt(short, long)]
        note: bool,
    },
    Put {
        #[structopt(long)]
        no_password: bool,
        #[structopt(long)]
        fixed: Option<String>,
        #[structopt(long)]
        chars: Option<String>,
        #[structopt(short, long)]
        length: Option<u16>,
        #[structopt(short, long)]
        count: Option<u64>,
        #[structopt(short, long)]
        note: bool
    },
    Del,
    Hint,
    Import,
    Export
}

fn main() -> anyhow::Result<()> {
    let options = Options::from_args();

    let db_path = options.db
        .or_else(|| ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .map(|dir| dir.config_dir().to_owned())
        )
        .ok_or(anyhow!("Can't find directory"))?;

    let tags = options.tags;

    if let Action::Init = options.action {
        fs::create_dir_all(&db_path)?;
    }

    let db = Rkv::new(&db_path)
        .map_err(StoreError)?;
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

    writeln!(&mut stdout, "tags: {:?}", &tags)?;

    match options.action {
        Action::Init => {
            let _ = Titso::init(&db, pass)?;
        },
        Action::Get { no_password, rule, note } => {
            let titso = Titso::open(&db, pass)?;
            let item = titso.get(&tags)?;

            if !no_password {
                match &item.password {
                    Type::Derive(rule) =>
                        writeln!(&mut stdout, "{}", titso.derive(&tags, rule))?,
                    Type::Fixed(pass) => writeln!(&mut stdout, "{}", pass)?
                }
            }

            if let (Type::Derive(rule), true) = (&item.password, rule) {
                writeln!(&mut stdout, "count: {}", rule.count)?;
                writeln!(&mut stdout, "chars: {:?}", rule.chars)?;
                writeln!(&mut stdout, "length: {}", rule.length)?;
            }

            if note {
                stdout.write_all(&item.note)?;
            }
        },
        Action::Put { no_password, fixed, length, chars, count, note } => {
            let titso = Titso::open(&db, pass)?;

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
                .map(Type::Fixed)
                .unwrap_or_else(move || Type::Derive(Rule {
                    count,
                    chars,
                    length
                }));
            let item = Item {
                password,
                note: if note {
                    let mut buf = Vec::new();
                    io::stdin().read_to_end(&mut buf)?;
                    buf
                } else {
                    Vec::new()
                }
            };

            titso.put(&tags, &item)?;

            if !no_password {
                match &item.password {
                    Type::Derive(rule) =>
                        writeln!(&mut stdout, "{}", titso.derive(&tags, rule))?,
                    Type::Fixed(pass) => writeln!(&mut stdout, "{}", pass)?
                }
            }
        },
        Action::Del => {
            let titso = Titso::open(&db, pass)?;
            titso.del(&tags)?;
        },
        Action::Hint => (),
        Action::Import => (),
        Action::Export => ()
    }

    // TODO

    Ok(())
}

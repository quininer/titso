mod utils;
mod input;
mod store;

use std::fs;
use std::io::{ self, Write };
use std::path::PathBuf;
use argh::FromArgs;
use anyhow::Context;
use directories::ProjectDirs;
use crossterm::{ queue, execute, terminal, style };
use seckey::ZeroAllocator;
use titso_core::{ packet, Core };
use store::Storage;
use utils::{ SafeTools, default_chars };


#[global_allocator]
static ALLOC: ZeroAllocator<std::alloc::System> = ZeroAllocator(std::alloc::System);

/// Titso Command line tools
#[derive(FromArgs)]
struct Options {
    #[argh(subcommand)]
    subcmd: SubCommands,

    /// specify config dir
    #[argh(option, short = 'c')]
    config: Option<PathBuf>,

    /// password tags
    #[argh(positional, greedy)]
    tags: Vec<String>
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommands {
    NewProfile(NewProfile),
    Get(GetItem),
    Set(SetItem),
    Remove(RemoveItem)
}

/// Create a new profile
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "new-profile")]
struct NewProfile {
    //
}

/// Get item
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "get")]
struct GetItem {
    /// show rule
    #[argh(switch)]
    rule: bool,

    /// show note
    #[argh(switch)]
    note: bool
}

/// Set item
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "set")]
struct SetItem {
    /// derive count
    #[argh(option)]
    count: Option<u64>,

    /// length rule
    #[argh(option)]
    length: Option<u16>,

    /// chars rule
    #[argh(option)]
    chars: Option<String>,

    /// note
    #[argh(option)]
    note: Option<String>,

    /// use fixed password
    #[argh(switch)]
    fixed: bool
}

/// Remove item
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "remove")]
struct RemoveItem {
    //
}


fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();

    let config_path = options.config
        .or_else(|| ProjectDirs::from("", "", "titso").map(|dir| dir.config_dir().into()))
        .ok_or_else(|| anyhow::format_err!("config path not found"))?;

    terminal::enable_raw_mode()?;

    scopeguard::defer!{
        let _ = terminal::disable_raw_mode();
    }

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let secret_path = config_path.join("titso-secret.bin");
    let storage = Storage::new(config_path);

    match options.subcmd {
        SubCommands::NewProfile(_) => {
            let mut master_secret_fd = fs::File::options()
                .write(true)
                .create_new(true)
                .open(&secret_path)?;

            let pass = input::askpass(&mut stdout)?;
            let (_core, master_secret) = Core::<SafeTools>::create(pass.as_bytes())?;

            master_secret_fd.write_all(&master_secret)?;
            master_secret_fd.sync_all()?;
        },
        SubCommands::Get(args) => {
            let mut core = {
                let master_secret = fs::read(&secret_path).context("read master secret failed")?;
                let pass = input::askpass(&mut stdout)?;
                Core::<SafeTools>::open(&master_secret, pass.as_bytes())?
            };

            let tags = if options.tags.is_empty() {
                input::asktags(&mut stdout)?
            } else {
                options.tags
            };

            let tag = core.store_tag(&tags);
            let mut itembuf = storage.get(&tag)?.context("not found this tag")?;
            let item = core.get(&tags, &mut itembuf)?;

            if args.rule {
                match &item.password {
                    packet::Type::Derive(rule) => queue!(
                        stdout,
                        style::Print("count: "),
                        style::Print(rule.count),
                        style::Print("\r\n"),
                        style::Print("length: "),
                        style::Print(rule.length),
                        style::Print("\r\n"),
                        style::Print("chars: "),
                        style::Print(format_args!("{:?}", rule.chars.as_slice())),
                        style::Print("\r\n")
                    )?,
                    packet::Type::Fixed(_) => queue!(stdout, style::Print("fixed\r\n"))?
                }
            }

            let pass = match item.password {
                packet::Type::Derive(rule) => core.derive(&tags, &rule),
                packet::Type::Fixed(pass) => pass,
            };

            queue!(
                stdout,
                style::Print(pass.as_str()),
            )?;

            if args.note {
                queue!(
                    stdout,
                    style::Print("\r\n"),
                    style::Print(item.note.as_str())
                )?;
            }

            execute!(stdout, style::Print("\r\n"))?;
        }
        SubCommands::Set(args) => {
            let mut core = {
                let master_secret = fs::read(&secret_path).context("read master secret failed")?;
                let pass = input::askpass(&mut stdout)?;
                Core::<SafeTools>::open(&master_secret, pass.as_bytes())?
            };

            let tags = if options.tags.is_empty() {
                input::asktags(&mut stdout)?
            } else {
                options.tags
            };
            let tag = core.store_tag(&tags);

            let password = if args.fixed {
                let pass = input::askpass(&mut stdout)?;
                packet::Type::Fixed(pass)
            } else {
                packet::Type::Derive(packet::Rule {
                    count: args.count.unwrap_or_default(),
                    length: args.length.unwrap_or(16),
                    chars: args.chars
                        .map(|s| s.chars().collect())
                        .unwrap_or_else(default_chars)
                })
            };

            if let packet::Type::Derive(rule) = &password {
                let pass = core.derive(&tags, rule);

                execute!(
                    stdout,
                    style::Print(pass.as_str()),
                    style::Print("\r\n")
                )?;
            }

            let item = packet::Item {
                password,
                note: args.note.unwrap_or_default(),
                padding: vec![]
            };

            let itembuf = core.put(&tags, &item)?;
            storage.set(&tag, &itembuf)?;
        }
        SubCommands::Remove(_) => {
            let mut core = {
                let master_secret = fs::read(&secret_path).context("read master secret failed")?;
                let pass = input::askpass(&mut stdout)?;
                Core::<SafeTools>::open(&master_secret, pass.as_bytes())?
            };

            let tags = if options.tags.is_empty() {
                input::asktags(&mut stdout)?
            } else {
                options.tags
            };

            let tag = core.store_tag(&tags);
            storage.remove(&tag)?;
        }
    }

    Ok(())
}

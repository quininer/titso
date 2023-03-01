mod utils;
mod input;
mod store;

use std::fs;
use std::io::{ self, Write };
use std::path::PathBuf;
use argh::FromArgs;
use anyhow::Context;
use directories::ProjectDirs;
use crossterm::{ execute, terminal, style };
use seckey::ZeroAllocator;
use titso_core::{ Core, SafeFeatures };
use utils::SafeTools;


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
    //
}

/// Set item
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "set")]
struct SetItem {
    /// derive count
    #[argh(option)]
    count: Option<u64>,

    /// chars rule
    #[argh(option)]
    chars: Option<String>,

    /// length rule
    #[argh(option)]
    length: Option<u16>,

    /// note
    #[argh(option)]
    note: Option<String>,

    /// use fixed password
    #[argh(option)]
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
        SubCommands::Get(_) => {
            let master_secret = fs::read(&secret_path).context("read master secret failed")?;
            let pass = input::askpass(&mut stdout)?;

            let mut core = Core::<SafeTools>::open(&master_secret, pass.as_bytes())?;

            //
        }
        SubCommands::Set(_) => {
            //
        }
        SubCommands::Remove(_) => {
            //
        }
    }

    Ok(())
}

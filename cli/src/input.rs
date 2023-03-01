use std::io;
use crossterm::{ queue, execute, terminal, style, event, cursor };


pub fn askpass<W: io::Write>(mut stdout: W) -> anyhow::Result<String> {
    let mut strbuf = String::new();

    loop {
        queue!(
            stdout,
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::CurrentLine),
            style::Print("Password: "),
        )?;
        ColorStar::hashpass(strbuf.as_str()).render(&mut stdout)?;

        if let event::Event::Key(event) = event::read()? {
            match event.code {
                event::KeyCode::Enter => break,
                event::KeyCode::Char(c) => {
                    if event.modifiers == event::KeyModifiers::CONTROL && c == 'c' {
                        return Err(io::Error::new(io::ErrorKind::Interrupted, "Ctrl-c").into());
                    }

                    if event.modifiers.intersects(event::KeyModifiers::CONTROL | event::KeyModifiers::ALT) {
                        continue;
                    }

                    if event.kind == event::KeyEventKind::Release {
                        continue;
                    }

                    strbuf.push(c);
                },
                event::KeyCode::Backspace => {
                    let _ = strbuf.pop();
                },
                event::KeyCode::Esc => strbuf.clear(),
                _ => ()
            }
        }
    }

    Ok(strbuf)
}

pub fn asktags<W: io::Write>(mut stdout: W) -> anyhow::Result<Vec<String>> {
    todo!()
}


struct ColorStar {
    colors: [u8; 6]
}

impl ColorStar {
    fn hashpass(pass: &str) -> ColorStar {
        use gimli_hash::GimliHash;

        let mut colors = [0; 6];

        let mut hasher = GimliHash::default();
        hasher.update(b"titso-askpass");
        hasher.update(pass.as_bytes());
        hasher.finalize(&mut colors);

        ColorStar { colors }
    }

    fn render<W: io::Write>(&self, mut writer: W) -> anyhow::Result<()> {
        fn asni(b: u8) -> style::Color {
            style::Color::AnsiValue(b.saturating_add(16) % 216)
        }

        for &b in self.colors.iter() {
            queue!(
                writer,
                style::SetForegroundColor(asni(b)),
                style::Print("**")
            )?;
        }

        execute!(writer, style::SetForegroundColor(style::Color::Reset))?;

        Ok(())
    }
}

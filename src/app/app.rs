use std::io::{self, Write};

pub const APP_NAME: &str = "Recall";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn run() -> io::Result<()> {
    let stdout = io::stdout();
    let mut output = stdout.lock();

    writeln!(output, "{APP_NAME} v{APP_VERSION}")?;
    writeln!(
        output,
        "Your learning companion. Pick up where you left off."
    )?;
    writeln!(output)?;
    writeln!(output, "Application foundation initialized.")?;
    writeln!(
        output,
        "The graphical interface has not been implemented yet."
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_name_is_recall() {
        assert_eq!(APP_NAME, "Recall");
    }

    #[test]
    fn application_version_is_not_empty() {
        assert!(!APP_VERSION.is_empty());
    }
}

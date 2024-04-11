use darling_api as darling;

pub static PACKAGE_MANAGER: VSCode = VSCode;

pub struct VSCode;

impl darling::PackageManager for VSCode {
    fn name(&self) -> String {
        "vscode".to_owned()
    }

    fn install(&self, _context: &darling::Context, package: &darling::InstallationEntry) -> anyhow::Result<()> {
        std::process::Command::new(code_or_codium()?)
            .arg("--install-extension")
            .arg(&package.name)
            .spawn()?
            .wait()?;

        Ok(())
    }

    fn uninstall(&self, _context: &darling::Context, package: &darling::InstallationEntry) -> anyhow::Result<()> {
        std::process::Command::new(code_or_codium()?)
            .arg("--uninstall-extension")
            .arg(&package.name)
            .spawn()?
            .wait()?;
        Ok(())
    }

    fn get_all_explicit(&self, _context: &darling::Context) -> anyhow::Result<Vec<(String, String)>> {
        let extensions = String::from_utf8(
            std::process::Command::new(code_or_codium()?)
                .arg("--show-versions")
                .arg("--list-extensions")
                .output()?
                .stdout,
        )?;
        let list = extensions.lines().filter(|line| !line.chars().all(|char| char.is_whitespace()));
        Ok(list
            .map(|line| {
                let parts = line.split('@').collect::<Vec<_>>();
                (parts[0].to_owned(), parts[1].to_owned())
            })
            .collect::<Vec<_>>())
    }
}

fn code_or_codium() -> anyhow::Result<&'static str> {
    if which::which("code").is_ok() {
        return Ok("code");
    }

    if which::which("codium").is_ok() {
        return Ok("codium");
    }

    Err(anyhow::anyhow!("No installation of VSCode or VSCodium found!"))
}

#[macro_use]
extern crate lazy_static;

use std::{
    env,
    fs::{self, File},
    io::LineWriter,
    io::Write,
    path::PathBuf,
};

use clap::Parser;
use format::Theme;

mod color;
mod error;
mod format;
mod highlight;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// The input colorscheme file
    pub filename: String,
    /// Directory of generated colorscheme, default to the current working directory
    pub output: Option<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let args: Args = Args::parse();

    let output = get_root_dir(args.output)?;
    let theme = format::parse_theme(&args.filename)?;

    setup_directories(&output, &theme.name)?;
    generate_vim_colors_file(&output, &theme.name)?;
    generate_init(&output, theme)?;

    Ok(())
}

fn get_root_dir(output: Option<String>) -> Result<String, anyhow::Error> {
    Ok(match output {
        Some(root) => PathBuf::from(root),
        None => match env::current_dir() {
            Ok(output) => output,
            Err(error) => return Err(error.into()),
        },
    }
    .display()
    .to_string())
}

fn setup_directories(output: &str, name: &str) -> Result<(), anyhow::Error> {
    match fs::create_dir_all(format!("{output}/{name}/lua/{name}")) {
        Ok(_) => {}
        Err(error) => return Err(error.into()),
    };

    match fs::create_dir_all(format!("{output}/{name}/colors")) {
        Ok(_) => {}
        Err(error) => return Err(error.into()),
    }

    Ok(())
}

fn generate_vim_colors_file(output: &str, name: &str) -> Result<(), anyhow::Error> {
    match fs::write(
        format!("{output}/{name}/colors/{name}.lua"),
        format!("require(\"{name}\").init({{}})\n"),
    ) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.into()),
    }
}

fn generate_init(output: &str, theme: Theme) -> Result<(), anyhow::Error> {
    let name = &theme.name;
    let background = &theme.background;

    let file = File::create(format!("{output}/{name}/lua/{name}/init.lua"))?;
    let mut file = LineWriter::new(file);

    file.write_all(
        b"local M = {}

local function set_hl_groups()
    local hl = vim.api.nvim_set_hl
",
    )?;

    for highlight in &theme.highlights {
        file.write_all(highlight.as_bytes())?;
    }

    file.write_all(
        format!(
            "
end

function M.init()
    vim.cmd(\"hi clear\")

    if vim.fn.exists(\"syntax_on\") then
        vim.cmd('syntax reset')
    end

    vim.o.background = \"{background}\"
    vim.o.termguicolors = true
    vim.g.colors_name = \"{name}\"

    set_hl_groups()
end

return M\n"
        )
        .as_bytes(),
    )?;

    Ok(())
}

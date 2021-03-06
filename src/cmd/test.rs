use clap::{App, Arg, ArgMatches, SubCommand};
use get_book_dir;
use mdbook::errors::Result;
use mdbook::MDBook;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("test")
        .about("Tests that a book's Rust code samples compile")
        .arg_from_usage(
            "-d, --dest-dir=[dest-dir] 'Output directory for the book{n}\
             (If omitted, uses build.build-dir from book.toml or defaults to ./book)'",
        )
        .arg_from_usage(
            "[dir] 'Root directory for the book{n}\
             (Defaults to the Current Directory when omitted)'",
        )
        .arg(Arg::with_name("library-path")
            .short("L")
            .long("library-path")
            .value_name("dir")
            .takes_value(true)
            .require_delimiter(true)
            .multiple(true)
            .empty_values(false)
            .help("A comma-separated list of directories to add to {n}the crate search path when building tests"))
}

// test command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let library_paths: Vec<&str> = args
        .values_of("library-path")
        .map(|v| v.collect())
        .unwrap_or_default();
    let book_dir = get_book_dir(args);
    let mut book = MDBook::load(&book_dir)?;

    if let Some(dest_dir) = args.value_of("dest-dir") {
        book.config.build.build_dir = dest_dir.into();
    }

    book.test(library_paths)?;

    Ok(())
}

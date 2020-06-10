#[macro_use]
extern crate self_update;

fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    let releases = self_update::backends::github::ReleaseList::configure()
            .repo_owner("vgmtv")
            .repo_name("test")
            .build()?
            .fetch()?;
    println!("Found release!:");
    println!("{:#?}\n", releases);

    let status = self_update::backends::github::Update::configure()
            .repo_owner("vgmtv")
            .repo_name("test")
            .bin_name("github")
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .build()?
            .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        println!("[Error] {}", e);
        ::std::process::exit(1);
    }
}

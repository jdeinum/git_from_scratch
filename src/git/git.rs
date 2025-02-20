use anyhow::Result;
use std::path::PathBuf;
use tracing::debug;

fn init_git_repo(p: PathBuf) -> Result<()> {
    let new_path = {
        let mut x = p.clone();
        x.push(".git/");
        x
    };
    debug!("creating git repo in {new_path:?}");
    std::fs::create_dir(new_path)?;
    Ok(())
}

pub fn init_git_repo_in_current_dir() -> Result<()> {
    init_git_repo(std::env::current_dir()?)
}

#[cfg(test)]
pub mod tests {
    use super::init_git_repo_in_current_dir;
    use anyhow::Result;
    use assert_cmd::assert;
    use std::path::Path;

    #[test]
    fn test_git_init() -> Result<()> {
        // create the test dir
        let test_dir = std::env::temp_dir();

        // change working directory to test dir
        std::env::set_current_dir(&test_dir)?;

        // create the initial repo
        init_git_repo_in_current_dir();

        // assert the new directory exists
        assert!(Path::new(format!("{}/{}", test_dir.to_str().unwrap(), ".git/").as_str()).exists());

        Ok(())
    }
}

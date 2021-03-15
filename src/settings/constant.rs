pub const CONFIG_PATH: &'static str = ".config/atlan/config.yaml";
pub const CONFIG_CONTENT: &'static str = r#"git_repo: https://github.com/user/repo.git
environment:
    GITHUB_USER: #If you want to override user's default
    GITHUB_TOKEN: #If you want to override user's default
files:
    - #filePath
"#;
# Atlan

A small Linux CLI to export and import configuration files from different hosts by syncing them to a common git repository.

## How to install

Download the last `atlan` CLI release from [Releases](/releases).

Make executable `chmod +x atlan`

Move the file to `/usr/bin` to make it available system wide.

## How to use

### On the origin
- Create a first empty configuration: `atlan config create`
- Edit the file created at `~/.config/atlan/config.yaml`
- Update the key: `git_repo` with your destination git repo
- Add the list of file you want to sync under the `files` key
- Create a first empty configuration: `atlan config create`
- Run `atlan backup` to sync the list of files on your repo

### On the destination
- Run `atlan config create --from: <your_git_repo>`
- Run `atlan restore` on the destination host to download the files from your repo

Note: You can now run `backup` and `restore` and every host using the CLI

## Commands
```
# Create configuration #
atlan config create

# Output configuration file #
atlan config view

# Delete configuration file #
atlan config delete

# Backup list of files specified in the confg file #
atlan backup

# Restore list of files specified in the confg file #
atlan restore

# Show version of CLI #
atlan --version

# Show help #
atlan --help
```

Notes: Every command's configuration is coming from `/.config/atlan/config.yaml`

## Todo

- Allow to sync previous configuration version
- List commits

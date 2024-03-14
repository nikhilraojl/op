## About

A simple program written in rust to quickly open projects in neovim. Works on both windows and linux.

## Usage

#### Basic

Just run `op` command and a list UI shows up, select a project to open with neovim.
Use `escape` to exit the UI
![](./media/op_nvim.png)

Below is the layout expected for this program to run by default. Only "project_dir" level in the layout is detected by this program

```
home
    |-Projects
        |-language-1
            |- project_dir
            |- project_dir
        |-language-2
            |- project_dir
            |- project_dir
        |-language-3
            |- project_dir
            |- project_dir
```

You can configure `Projects` location and also include additional directories with `.opconfig`. Example config below

```ini
# specifying the base `Projects` location (lines starting with `#` are ignored)
projects_dir=/path/to/dir

# specify additional `project_dir` which may not be a child of above `projects_dir`
# but want to be detected by this program anyway
include=/path/to/project_dir

# add additional `project_dir`s with a new include line
include=/path/to/project_dir_2
```

#### Main options

`op [--help|-h]`: to show available commands & options

`op [project_name]`: to directly open the project_dir in nvim.
This becomes more powerfull when combined with tab completion.
See below for setting up autocomplete for powershell & bash

`op [--create|-c]`: to create the above mentioned layout.
This creates five directories with names python, javascript, rust, go, plain_txt for organizing in your `Projects` directory\
_NOTE: No additional directories inside the language directories will be created, BYOProject_

```
home
    |-Projects
        |-python
        |-javascript
        |-rust
        |-go
        |-plain_txt
```

`op [--list|-l]`: to list all the project_dirs

`op [--print|-p]`: to print full path of the project_dir flag. The output can be piped in a shell. Example

```
op test_proj -p | cd    in powershell
or
cd `op tmp -p`          in bash
```

`op [--add|-a] <path>`: adds a new line `include=<path>` to `.opconfig`. Useful for quickly adding project_dirs from cli instead of doing it manually

`op [--git-status|-g]`: prints out git status of all the `project_dir`s detected. Example output

```
project_dir_1--------------->["DIRTY"] // some local uncommitted changes are present
project_dir_2--------------->["NOT IN SYNC"] // HEAD & remote are not at the same commit
project_dir_3--------------->["DIRTY", "NOT IN SYNC"] // both of above
```

_NOTE: Git uninitiated and git directories with clean worktrees are ignored in the output._\
_NOTE: Only the locally checked out branch status is considered_

## Autocomplete for shells

For tab completion in powershell you can add the below script to your pprofile

```powershell
$opCommandCompletion = {
    param($stringMatch)

    # using outupt from the `op --list` command to build autocomplete list
    $items = @(op --list | Where-Object {$_ -like "$wordToComplete*"})

    $items
}
Register-ArgumentCompleter -Native -CommandName op -ScriptBlock $opCommandCompletion
```

For tab completion in bash you can add the below script to your rc file

```bash
_op_completion() {
	if [ "${#COMP_WORDS[@]}" != "2" ]; then
		return
	fi

	COMPREPLY=($(compgen -W "$(op --list)" "${COMP_WORDS[1]}"))
}
complete -F _op_completion op
```

## Build

- requirements: rustc, cargo(you can have both by installing `rustup`), neovim
- clone the repo and cd into it
- `cargo test` for running tests
- run `cargo build --release --target_dir="somewhere/in/path"` to build and use binary

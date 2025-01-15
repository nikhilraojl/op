## About

A simple project management program which can quickly open projects in neovim. Works on both windows and linux. Written in rust btw.\
![](./media/op_nvim.png)

## Usage
**IMPORTANT: Below is the layout expected for this program to run. By default only 'project_dir' level in the layout is detected by this program**

```
# Reference layout
home
    |-Projects
        |- language_dir_1
            |- project_dir
            |- project_dir
        |- language_dir_2
            |- project_dir
            |- project_dir
        |- language_dir_3
            |- project_dir
            |- project_dir

# Actual layout
$HOME
    |-Projects
        |- rust
            |- op
            |- axum
        |- python
            |- django
            |- py_ds_kata
```

### Select UI
You can just run `op` command and an UI shows up. This displays all projects as a list. Navigate using arrow keys, fuzzy search by typing and select a project with `enter` to open it with neovim. Use `escape` to exit the UI

```shell
Find:
   axum_promodoro
   bdays
>> Blogs
   clip_history
   dist
   django_web
   dll_rust
   explorations
   f1gp
```

- `Arrow Up (or) J`: moves the selection up
- `Arrow Down (or) K`: moves the selection down
- `Esc`: exits the program
- `Enter`: opens selected project in neovim
- `Ctrl + Backspace`: clear current search

### Direct
Another way to use this program is to run `op <project_dir>` which opens said project directly in neovim. Additional CLI options are mentioned below

## Config

You can configure multiple `Projects` roots and also include additional directories outside of the layout with `.opconfig` in your home folder. Example config below

```ini
# specifying the base `Projects` location (lines starting with `#` are ignored)
projects_dir=/path/to/dir

# specifying the extra/additional `Projects`
extra_projects_root=/differentpath/to/dir

# ignore any `language_dir` level directory (NOTE: shared among all the `Project` roots)
ignore=/path/to/language_dir

# specify additional `project_dir` which may not be a child of above `projects_dir`
# but want to be detected by this program anyway
include=/path/to/project_dir

# add additional `project_dir`s with a new include line
include=/path/to/project_dir_2

# Compound Projects. To open multiple projects in neovim use `compound_projects`
# Each project is opened in a new wezterm tab
# compound_project_name: Give a name for this project list
# project_1_name & project_2_name: Names (not paths) of the projects to open
# NOTE: requires wezterm to be installed & currently only two are supported
compound_projects=compound_project_name,project_1_name,project_2_name
```
> __IMPORTANT__: Be consistent when using path separators. On windows use `\` and on linux use `/`.
This is currently not enforced and may cause unexpected issues if not followed.


## CLI options

`op [--help|-h]`: shows all available commands & options

`op [project_dir]`: directly opens the 'project_dir' in nvim. This becomes more powerfull when combined with tab completion. See below for setting up autocomplete for powershell & bash

`op [project_name] [--print|-p]`: prints full path of the 'project_dir'. The output can be piped in a shell. For example to quickly `cd` to a 'project_dir' you can do something like

```
op test_proj -p | cd    in powershell
or
cd `op tmp -p`          in bash
```

`op [project_name] [--uri|-u]`: prints git remote url of the project. Useful if your terminal supports clickable links. This option is not smart, it just performs a simple string substitution. May fail in some cases

```
op op --uri

# output
https://github.com/nikhilraojl/op_nvim
```

`op [--create|-c]`: creates a directory layout as mentioned in the beginning. This command creates five directories with names python, javascript, rust, go, plain_txt in a `Projects` directory

```
home
    |-Projects
        |-python
        |-javascript
        |-rust
        |-go
        |-plain_txt
```

`op [--list|-l]`: lists all the 'project_dir's

`op [--add|-a] <path>`: useful for quickly adding project_dirs from cli instead of doing it manually. Adds a new line `include=<path>` to `.opconfig`

`op [--git-status|-g]`: prints out git status of all the 'project_dir's detected. Example output below

```
# some local uncommitted changes are present
project_dir_1               : ["DIRTY"]

# HEAD & remote are not at the same commit
project_dir_2               : ["NOT IN SYNC"]

# both of the above
project_dir_3               : ["DIRTY", "NOT IN SYNC"]
```

_NOTE: Git uninitiated and git directories with clean worktrees are ignored in the output. Only the locally checked out branch status is considered_

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

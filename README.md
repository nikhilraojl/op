## About
A simple program written in rust to quickly open projects in neovim. Works on both windows and linux.

## Usage

#### Basic
Just run `op` command and a list UI shows up, select a project to open with neovim. 
Use `escape` to exit the UI
![](./media/op_nvim.png)

Below is the layout expected for this program to run. Only "project_dir" level in the layout is detected by this program
```
home
    |-Projects
        |-language-1
            |- project_dir
            |- project_dir
        |-language-2
            |- project_dir
            |- project_dir
        |-.opinclude
```
Use `.opinclude` file to add any directory which is not in the layout. Lines starting with `#` are ignored 
```
# /path/to/project1 
/path/to/project2
```

#### Main options
Use `op [project_name]`: to directly open the project in nvim. 
This becomes more powerfull when combined with tab completion.
See below for setting up autocomplete for powershell & bash

Use `op [--create|-c]`: to create the above mentioned layout.
This creates five directories with names python, javascript, rust, go, plain_txt for organizing & a file `.opinclude` in your `$HOME/Projects` directory\
*NOTE: No actual projects inside the language directories will be created, BYOProject*
```
home
    |-Projects
        |-python
        |-javascript
        |-rust
        |-go
        |-plain_txt
        |-.opinclude
```

Use `op [--list|-l]`: to list all the projects

Use `op [--print|-p]`: to print full path of the project flag. The output can be piped in a shell. Example
```
op test_proj -p | cd    in powershell 
or
cd `op tmp -p`          in bash 
```

Use `op [--help|-h]`: to show available commands & options

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

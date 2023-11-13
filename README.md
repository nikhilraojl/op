A simple program written in rust to quickly open projects in neovim. Works on both windows and linux.

![](./media/op_nvim.png)

## Run
Below is the layout expected for this program to run. Only "project" level in the layout is detected by this program
```
home
    |-Projects
        |-language
            |- project
            |- project
        |-language
            |- project
            |- project 
```

To list all the projects
```
op [--list|-l]
```

To directly open the project in nvim
```
op [project_name]
```
or print full path of the project using `--print` or `-p` flag. The output of `--print` or `-p` can be used to pipe in a shell
```
op test_proj -p | cd 
```

For auto complete you can add the below script to your powershell profile
```powershell
$PROJECTSPATH = "$HOME\Projects\"
$IGNOREDIR = "$HOME\Projects\deploys*"
$opCommandCompletion = {
    param($stringMatch)
    $items  = @(Get-ChildItem -Path "$PROJECTSPATH\*\$stringMatch*" -Directory |
        Where-Object {$_.fullname -notlike $IGNOREDIR } |
        Select-Object -ExpandProperty name )

    $items
}

Register-ArgumentCompleter -Native -CommandName op -ScriptBlock $opCommandCompletion
```

## Build
- requirements: rustc, cargo(you can have both by installing `rustup`), neovim
- clone the repo and cd into it
- run `cargo build --release --target_dir="somewhere/in/path"`

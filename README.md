A simple program written in rust to open projects in neovim. Works on both windows and linux.

![](./media/op_nvim.png)

## Run
Below is the layout expected for this program to run. 
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
Only projects are in the layout are detected by this program.

Check projects present
```
op [--list|-l]
```

If can also directly open the project or print full path using `--print` or `-p` flag
```
op [project_name] <--print|-p>
```
The output of `--print` or `-p` can be used to pipe in a shell. To change directory to a project
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
```

## Build
- Requirements: rustc, cargo(you can have both by installing `rustup`)
- clone the repo
- `cargo build --release --target_dir="somewhere/in/path"`


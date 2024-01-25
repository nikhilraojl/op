#[cfg(test)]
mod tests {
    use crate::{
        actions::{
            create_layout::CreateLayout,
            list_projects::ListAction,
            main_help::MainHelpAction,
            open_in_nvim::OpAction,
            opinclude_actions::{IncludeAction, PopAction},
        },
        process_arg_command,
        utils::get_profile_path,
        ArgAction,
    };

    #[test]
    fn test_process_main_help_action() {
        // main action
        let mut args = ["--help".to_owned()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let exp = ArgAction::MainHelp(MainHelpAction);
        assert_eq!(act, exp);
    }

    #[test]
    fn test_process_list_action() {
        // --list
        let mut args = ["--list".to_owned()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let exp = ArgAction::ListAllProjects(ListAction::default());
        assert_eq!(act, exp);

        // --list --help
        let mut args = ["--list".to_owned(), "--help".to_owned()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let list_args = ListAction { help: true };
        let exp = ArgAction::ListAllProjects(list_args);
        assert_eq!(act, exp);

        // --list --help <something more>
        let mut args = ["--list".to_owned(), "--help".to_owned(), "x".to_owned()].into_iter();
        match process_arg_command(&mut args) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_process_create_layout_action() {
        // --create
        let mut args = ["--create".to_owned()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let exp = ArgAction::CreateLayout(CreateLayout::new());
        assert_eq!(act, exp);

        // --create --help
        let mut args = ["--create".to_owned()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let exp = ArgAction::CreateLayout(CreateLayout::new());
        assert_eq!(act, exp);

        // --create --help <something more>
        let mut args = ["--list".to_owned(), "--help".to_owned(), "y".to_owned()].into_iter();
        match process_arg_command(&mut args) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_process_open_action() {
        // project
        let mut args = ["project".to_owned()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let op_args = OpAction {
            proj_name: "project".to_owned(),
            print_path: false,
            help: false,
        };
        let _exp = ArgAction::OpenProject(op_args);
        assert_eq!(act, _exp);

        // project --help
        let mut args = ["project".to_owned(), "--help".to_owned()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let op_args = OpAction {
            proj_name: "project".to_owned(),
            print_path: false,
            help: true,
        };
        let exp = ArgAction::OpenProject(op_args);
        assert_eq!(act, exp);

        // project --help <something more>
        let mut args = ["project".to_owned(), "--help".to_owned(), "x".to_owned()].into_iter();
        match process_arg_command(&mut args) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_process_open_action_print() {
        // project --print
        let mut args = ["project".to_owned(), "--print".to_owned()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let op_args = OpAction {
            proj_name: "project".to_owned(),
            print_path: true,
            help: false,
        };
        let exp = ArgAction::OpenProject(op_args);
        assert_eq!(act, exp);

        // project --print --help
        let mut args = [
            "project".to_owned(),
            "--print".to_owned(),
            "--help".to_owned(),
        ]
        .into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let op_args = OpAction {
            proj_name: "project".to_owned(),
            print_path: true,
            help: true,
        };
        let exp = ArgAction::OpenProject(op_args);
        assert_eq!(act, exp);

        // project --help --print
        let mut args = [
            "project".to_owned(),
            "--help".to_owned(),
            "--print".to_owned(),
        ]
        .into_iter();
        match process_arg_command(&mut args) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_add_to_opinclude_action() {
        // --add <some valid path>
        let some_existing_path = get_profile_path().unwrap();
        let mut args = ["--add".to_owned(), some_existing_path.clone()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let include_args = IncludeAction {
            path: some_existing_path,
            help: false,
        };
        let exp = ArgAction::AddToOpInclude(include_args);
        assert_eq!(act, exp);

        // project --add <invalid path>
        let some_existing_path = "/invalid/path".to_owned();
        let mut args = ["--add".to_owned(), some_existing_path.clone()].into_iter();
        let act = process_arg_command(&mut args).is_ok();
        assert_eq!(act, false);

        // --add --help
        let mut args = ["--add".to_owned(), "--help".to_owned()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let include_args = IncludeAction {
            path: String::new(),
            help: true,
        };
        let exp = ArgAction::AddToOpInclude(include_args);
        assert_eq!(act, exp);
        //
        // --add <some valid path> --help
        let some_existing_path = get_profile_path().unwrap();
        let mut args = [
            "--add".to_owned(),
            some_existing_path.clone(),
            "--help".to_owned(),
        ]
        .into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let include_args = IncludeAction {
            path: some_existing_path,
            help: true,
        };
        let exp = ArgAction::AddToOpInclude(include_args);
        assert_eq!(act, exp);
    }

    #[test]
    fn test_pop_from_opinclude_action() {
        // --pop
        let mut args = ["--pop".to_owned()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let pop_args = PopAction { help: false };
        let exp = ArgAction::PopFromOpInclude(pop_args);
        assert_eq!(act, exp);

        // -o
        let mut args = ["-o".to_owned()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let pop_args = PopAction { help: false };
        let exp = ArgAction::PopFromOpInclude(pop_args);
        assert_eq!(act, exp);

        // --pop --help
        let mut args = ["--pop".to_owned(), "--help".to_owned()].into_iter();
        let act = process_arg_command(&mut args).unwrap();
        let pop_args = PopAction { help: true };
        let exp = ArgAction::PopFromOpInclude(pop_args);
        assert_eq!(act, exp);
    }
}

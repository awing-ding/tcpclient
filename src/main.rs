mod app;

fn parse_args(args: &[String]) -> Result<Option<(String, String)>, String> {
    if args.len() == 3 {
        let args = (args[1].clone(), args[2].clone());
        Ok(Some(args))
    } else {
        Ok(None)
    }
}

fn main() {
    color_eyre::install().expect("Should install color_eyre for logging");
    let args = std::env::args().collect::<Vec<String>>();
    let args = parse_args(&args).unwrap();
    let mut terminal = ratatui::init();
    terminal.clear().expect("should clear terminal");
    let mut application: app::App;
    if args.is_none() {
        application = app::App::new(None);
    } else {
        application = app::App::new(args);
    }
    application.run(terminal).expect("app should not fail");
    
}
    
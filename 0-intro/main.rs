#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu is unavailable".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("{:?}", choice);
}

fn pick_choice(input: &str) -> Result<(),String>{
    let choice: MenuChoice = get_choice(input)?; 

    print_choice(&choice);

    Ok(())
}

fn main() {
   let _ =  pick_choice("start");
}


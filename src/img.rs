use viuer::{print_from_file, Config};

const EMPTY: PreviewError = PreviewError::Empty("No Image was presented.");

#[derive(PartialEq, Debug)]
pub enum PreviewError {
    Empty(&'static str),
}

pub fn preview(path: &Option<String>){
    let conf = Config {
        truecolor: true,
        width: Some(80),
        height: Some(25),
        ..Default::default()
    };

    match path {
        None => panic!("{:?}",EMPTY),
        Some(img) =>{
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // clear the terminal and put the cursor at row 1, col 1
            print_from_file(img, &conf).expect("Image printing failed.");
        }
    }
}

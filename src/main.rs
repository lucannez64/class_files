use std::fs;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let folder : &str;
    if args.len() == 2 {
        folder = &args[1];
        println!("{:?}", args);
    } else {
        folder = "C:/Downloads";
    }
    let paths = fs::read_dir(folder).unwrap();

    for path in paths {
        let patha = path.as_ref().unwrap().path();
        let ext_ostr = patha.extension();
        if ext_ostr == None {
            continue;
        }
        let ext = ext_ostr.unwrap().to_str().unwrap();
        match ext {
            "PDF" => {
                fs::rename(&patha, "C:/Users/mazav/OneDrive/Documents/PDF/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "pdf" => {
                fs::rename(&patha, "C:/Users/mazav/OneDrive/Documents/PDF/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "rar" => {
                fs::rename(&patha, "C:/ZIP/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "zip" => {
                fs::rename(&patha, "C:/ZIP/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "msi" => {
                fs::rename(&patha, "C:/installation/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "exe" => {
                fs::rename(&patha, "C:/installation/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "ai" => {
                fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/SVG/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "svg" => {
                fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/SVG/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "aseprite" => {
                fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/SVG/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "gif" => {
                fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/SVG/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "PNG" => {
                fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "png" => {
                fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "jpg" => {
                fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "jpeg" => {
                fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            "webp" => {
                fs::rename(&patha, "C:/Users/mazav/OneDrive/Images/".to_owned()+patha.file_name().unwrap().to_str().unwrap()).unwrap();
            },
            _ => {
            }
        }
    }
}
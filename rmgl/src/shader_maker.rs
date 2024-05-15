use std::ffi::CString;
use std::fs;

pub fn make_frag(models:&Vec<(String,String)>)->CString{
    let c_frag = CString::new(include_str!(".frag")).unwrap();
    let mut string_frag = c_frag.to_string_lossy().into_owned();

    let mut dis_funcans = String::from("");
    let mut else_ifs = String::from("");
    let mut i = 0;
    for model in models{
        let dis_funcan = &model.0;
        let object_name = &model.1;
        if dis_funcan != ""{
            dis_funcans = format!("{}{}",dis_funcans,dis_funcan);
            else_ifs = format!("{} else if(tran[i].type == {i}){}
                dis = sd{}(p);
            {}{}",else_ifs,"{",object_name,"}","");
        }
        i += 1;
    }

    string_frag = string_frag.replace("//#dis funcans here#", dis_funcans.as_str());
    string_frag = string_frag.replace("//#else if else if#", else_ifs.as_str());

    //println!("{}",string_frag);
    
    CString::new(string_frag.as_bytes()).expect("CString conversion failed")
}

pub fn get_dis_funcans_folder(folder_path: &str) -> Option<Vec<(String,String)>>{

    let mut models : Vec<(String,String)> = vec![];

    let objects_texts = get_files_in_folder(folder_path)?;

    for object_text in objects_texts {
        let g = add_object_from_file(&(String::from(folder_path) + "/" + object_text.as_str()));
        if let Some(model) = g {
            models.push(model);
        }
    }

    return Some(models);
}


fn get_files_in_folder(folder_path: &str) -> Option<Vec<String>> {
    // Read the directory
    let dir_entries = match fs::read_dir(folder_path) {
        Ok(entries) => entries,
        Err(_) => return None, // Return None if there's an error reading the directory
    };

    // Collect file names into a vector of Strings
    let mut files: Vec<String> = Vec::new();
    for entry in dir_entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy().into_owned();
            files.push(file_name_str);
        } else {
            return None; // Return None if there's an error reading an entry
        }
    }

    Some(files)
}

pub fn add_object_from_file(file_name:&str) -> Option<(String,String)>{

    let ra: Vec<&str> = file_name.split(".").collect();
    let file_type = ra.last().unwrap();
    if String::from(*file_type) != "sdf" {return None;}
    let file_text = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");
    Some(add_object(&file_text))
}

pub fn add_object(text:&str) -> (String,String){
    let lines = text.lines();
    let mut object_name = String::from("new_object");
    let mut g = String::from("");
    let mut i = 0;
    for line in lines{
        if i == 0{
            object_name = String::from(line);
            g = format!("
            float sd{}(vec3 p)",&object_name);
        }
        else if i > 1 {
            g = format!("{}{}\n",g,line);   
        }
        i += 1;
    }
    (g,object_name)
}

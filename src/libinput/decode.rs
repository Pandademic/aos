use arrform::{arrform, ArrForm};

// smartly remove the double characters that somehow appear
pub fn de_scramble(what: &str) -> ArrForm<555> {
    let mut last_char:char = ' ';
    let mut ret = arrform!(555,"{}","");

    for c in what.chars(){
        if last_char != ' ' {
            if c == last_char {
                ret = arrform!(555,"{}{}",ret.as_str(),c);
            }
        }
        
        last_char = c;
    }

    return ret;
}

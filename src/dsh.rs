// the dumb shell
use crate::println;
#[allow(unused_imports)]
use crate::panic;
use heapless::Vec;

mod libinput;


fn match_cmds(cmd: &str) -> i32 {
    let mut cmds:Vec<&str,4> = Vec::new();
    cmds.push("exit").unwrap();
    cmds.push("help").unwrap();
    cmds.push("version").unwrap();
    cmds.push("panic").unwrap();

    if cmds.contains(&cmd) {
            if cmd == "help" {
                println!("this is the very helpful help");
            }
            if cmd == "version" {
                println!("1.0.0");
            }
            if cmd == "panic" {
                panic!("dsh recived the panic command !");
            }
            if cmd == "exit" {
                println!("exiting...\n thank you for your patronage!");
                return 1;
            }
    }else{
        println!("dsh: {}: command not found",cmd);
    }
    return 0;
}
pub fn init() {
    println!("Welcome to dumb-shell(dsh) 1.0.0");
    println!("Enjoy your stay!");
    
    loop {
        println!("dsh>");
        match libinput::get_input_to_enter() {
            Err(why) => panic!("{:?}",why),
            Ok(result) => match match_cmds(libinput::decode::de_scramble(result.as_str()).as_str()){
                0 => continue,
                1 => break,
                _ => continue
            },
        }
    }
}



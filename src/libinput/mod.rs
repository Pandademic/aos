// aos libinput
// used for getting input

use ps2::{Controller, error::ControllerError, flags::ControllerConfigFlags};
use arrform::{arrform, ArrForm};

pub mod decode;

pub fn read_keycodes(cont: &mut Controller) -> ArrForm<555> {
    // the maximum input to enter is 555, to enter
    let mut got = arrform!(555,"{}","");
    loop {
        match Controller::read_data(cont) {
            Err(_) => continue,
            Ok(res) => match res {
                28 => got = arrform!(555,"{}{}",got.as_str(),"a"),
                50 => got = arrform!(555,"{}{}",got.as_str(),"b"),
                33 => got = arrform!(555,"{}{}",got.as_str(),"c"),
                35 => got = arrform!(555,"{}{}",got.as_str(),"d"),
                36 => got = arrform!(555,"{}{}",got.as_str(),"e"),
                43 => got = arrform!(555,"{}{}",got.as_str(),"f"),
                52 => got = arrform!(555,"{}{}",got.as_str(),"g"),
                51 => got = arrform!(555,"{}{}",got.as_str(),"h"),
                67 => got = arrform!(555,"{}{}",got.as_str(),"i"),
                59 => got = arrform!(555,"{}{}",got.as_str(),"j"),
                66 => got = arrform!(555,"{}{}",got.as_str(),"k"),
                75 => got = arrform!(555,"{}{}",got.as_str(),"l"),
                58 => got = arrform!(555,"{}{}",got.as_str(),"m"),
                49 => got = arrform!(555,"{}{}",got.as_str(),"n"),
                68 => got = arrform!(555,"{}{}",got.as_str(),"o"),
                77 => got = arrform!(555,"{}{}",got.as_str(),"p"),
                21 => got = arrform!(555,"{}{}",got.as_str(),"q"),
                45 => got = arrform!(555,"{}{}",got.as_str(),"r"),
                27 => got = arrform!(555,"{}{}",got.as_str(),"s"),
                44 => got = arrform!(555,"{}{}",got.as_str(),"t"),
                60 => got = arrform!(555,"{}{}",got.as_str(),"u"),
                42 => got = arrform!(555,"{}{}",got.as_str(),"v"),
                29 => got = arrform!(555,"{}{}",got.as_str(),"w"),
                34 => got = arrform!(555,"{}{}",got.as_str(),"x"),
                53 => got = arrform!(555,"{}{}",got.as_str(),"y"),
                26 => got = arrform!(555,"{}{}",got.as_str(),"z"),
                22 => got = arrform!(555,"{}{}",got.as_str(),"1"),
                30 => got = arrform!(555,"{}{}",got.as_str(),"2"),
                38 => got = arrform!(555,"{}{}",got.as_str(),"3"),
                37 => got = arrform!(555,"{}{}",got.as_str(),"4"),
                46 => got = arrform!(555,"{}{}",got.as_str(),"5"),
                54 => got = arrform!(555,"{}{}",got.as_str(),"6"),
                61 => got = arrform!(555,"{}{}",got.as_str(),"7"),
                62 => got = arrform!(555,"{}{}",got.as_str(),"8"),
                70 => got = arrform!(555,"{}{}",got.as_str(),"9"),
                69 => got = arrform!(555,"{}{}",got.as_str(),"0"),
                41 => got = arrform!(555,"{}{}",got.as_str(),"^"),
                90 => break,
                _ => continue,
            },
        }
    }


     return got;
}

pub fn get_input_to_enter() -> Result<ArrForm<555>, ControllerError> {
    let mut controller = unsafe { Controller::new() };

    controller.disable_keyboard()?;
    controller.disable_mouse()?;

    let _ = controller.read_data();

    let mut config = controller.read_config()?;
    // Disable interrupts and scancode translation
    config.set(
        ControllerConfigFlags::ENABLE_KEYBOARD_INTERRUPT
            | ControllerConfigFlags::ENABLE_MOUSE_INTERRUPT
            | ControllerConfigFlags::ENABLE_TRANSLATE,
        false,
    );
    controller.write_config(config)?;

    controller.test_controller()?;
    controller.write_config(config)?;

    let has_mouse = if config.contains(ControllerConfigFlags::DISABLE_MOUSE) {
        controller.enable_mouse()?;
        config = controller.read_config()?;
        !config.contains(ControllerConfigFlags::DISABLE_MOUSE)
    } else {
        false
    };
    controller.disable_mouse()?;

    let keyboard_works = controller.test_keyboard().is_ok();
    let mouse_works = has_mouse && controller.test_mouse().is_ok();

    config = controller.read_config()?;
    if keyboard_works {
        controller.enable_keyboard()?;
        config.set(ControllerConfigFlags::DISABLE_KEYBOARD, false);
        config.set(ControllerConfigFlags::ENABLE_KEYBOARD_INTERRUPT, true);
        controller.keyboard().reset_and_self_test().unwrap();
    }
    if mouse_works {
        controller.enable_mouse()?;
        config.set(ControllerConfigFlags::DISABLE_MOUSE, false);
        config.set(ControllerConfigFlags::ENABLE_MOUSE_INTERRUPT, true);
        controller.mouse().reset_and_self_test().unwrap();
        controller.mouse().enable_data_reporting().unwrap();
    }

    controller.write_config(config)?;
    
    
    Ok(read_keycodes(&mut controller))

}



use crate::sleep;
use crate::DEBUG;



pub fn tap(
    ch: char
) {
    let virtual_keycode: u16 = match ch {
        'W' => 17u16,
        'A' => 30u16,
        'S' => 31u16,
        'D' => 32u16,
        '\n' => 28u16,
        _ => panic!("Invalid character!")
    };

    // Initial input
    let mut initial_input: winapi::um::winuser::INPUT = Default::default();
    initial_input.type_ = winapi::um::winuser::INPUT_KEYBOARD;
    unsafe { 
        let ki: &mut winapi::um::winuser::KEYBDINPUT = &mut initial_input.u.ki_mut();
        ki.wVk = 0;
        ki.wScan = virtual_keycode;
        ki.dwFlags = 0 | winapi::um::winuser::KEYEVENTF_SCANCODE;
    };

    // Send the input
    unsafe {
        let result_byte = winapi::um::winuser::SendInput(
            1,
            &mut initial_input,
            std::mem::size_of::<winapi::um::winuser::INPUT>() as i32
        );

        if DEBUG {
            println!("Resulting byte from key down: {:?}", result_byte);
        }
    }

    // Sleep for a bit
    sleep!(50);

    // Key up
    let mut key_up: winapi::um::winuser::INPUT = Default::default();
    key_up.type_ = winapi::um::winuser::INPUT_KEYBOARD;
    unsafe { 
        key_up.u.ki_mut().wVk = 0;
        key_up.u.ki_mut().wScan = virtual_keycode;
        key_up.u.ki_mut().dwFlags = 0 | winapi::um::winuser::KEYEVENTF_SCANCODE | winapi::um::winuser::KEYEVENTF_KEYUP;
    };

    // Send the input
    unsafe {
        let result_byte = winapi::um::winuser::SendInput(
            1,
            &mut key_up,
            std::mem::size_of::<winapi::um::winuser::INPUT>() as i32
        );

        if DEBUG {
            println!("Resulting byte from key up: {:?}", result_byte);
        }
    }
}
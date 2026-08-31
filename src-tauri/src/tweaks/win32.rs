use std::mem::size_of;

#[repr(C)]
struct AnimationInfo {
    cb_size: u32,
    min_animate: i32,
}

#[link(name = "user32")]
extern "system" {
    fn SystemParametersInfoW(action: u32, ui_param: u32, pv_param: *mut AnimationInfo, f_win_ini: u32) -> i32;
}

const SPI_SETANIMATION: u32 = 0x0049;
const SPIF_UPDATEINIFILE: u32 = 0x01;
const SPIF_SENDCHANGE: u32 = 0x02;

pub fn set_min_animate(enabled: bool) {
    let mut info = AnimationInfo {
        cb_size: size_of::<AnimationInfo>() as u32,
        min_animate: i32::from(enabled),
    };
    unsafe {
        SystemParametersInfoW(
            SPI_SETANIMATION,
            info.cb_size,
            &mut info,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        );
    }
}

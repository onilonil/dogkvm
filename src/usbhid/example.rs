use esp_idf_sys::{tinyusb_config_t, tusb};
use esp_idf_sys;

unsafe extern "C" {
    fn get_hid_report_descriptor() -> *const u8;
    fn get_hid_report_descriptor_len() -> u16;
    fn get_hid_configuration_descriptor() -> *const u8;
    fn get_hid_configuration_descriptor_len() -> u16;
    fn get_tinyusb_default_config() -> esp_idf_sys::tinyusb_config_t;
}

const TUSB_DESC_TATLA_LEN : u32 = tusb::TUD_CONFIG_DESC_LEN + tusb::CFG_TUD_HID * tusb::TUD_HID_DESC_LEN;

const APP_BUTTON: esp_idf_sys::gpio_num_t = esp_idf_sys::gpio_num_t_GPIO_NUM_35;




#[unsafe(no_mangle)]
pub extern "C" fn tud_hid_descriptor_report_cb(
    _instance: u8,
) -> *const u8 {
    log::error!("HID report descriptor callback called");
    unsafe { get_hid_report_descriptor() }
}

#[unsafe(no_mangle)]
pub extern "C" fn tud_hid_get_report_cb(
    _instance: u8,
    _report: u8,
    _report_type: u8,
    _buffer: *mut u8,
    _reqlen: u16,
) -> u16 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn tud_hid_set_report_cb(
    _instance: u8,
    _report: u8,
    _report_type: u8,
    _buffer: *const u8,
    _bufsize: u16,
) {

}

enum MOUSEDIRE {
    MOUSEDIRERIGHT,
    MOUSEDIREDOWN,
    MOUSEDIRELEFT,
    MOUSEDIREUP,
}

const DISTANCE_MAX: i8 = 125;
const DELTA_SCALE: i8 = 5;

struct Mouse{
    cur_dir: MOUSEDIRE,
    distance: u32,
}

impl Mouse {
    fn new() -> Self {
        Mouse {
            cur_dir:MOUSEDIRE::MOUSEDIRERIGHT,
            distance: 0,
        }
    }

    fn mouse_draw_squre_next_delta(&mut self) -> (i8, i8){
        let mut delta_x_ret = 0i8;
        let mut delta_y_ret = 0i8;
        match self.cur_dir {
            MOUSEDIRE::MOUSEDIRERIGHT => { 
                delta_x_ret = DELTA_SCALE; 
                delta_y_ret = 0; 
            },
            MOUSEDIRE::MOUSEDIREDOWN => { 
                delta_x_ret = 0;
                delta_y_ret = DELTA_SCALE;
            },
            MOUSEDIRE::MOUSEDIRELEFT => { 
                delta_x_ret = -DELTA_SCALE;
                delta_y_ret = 0;
            },
            MOUSEDIRE::MOUSEDIREUP => { 
                delta_x_ret = 0;
                delta_y_ret = -DELTA_SCALE;
            },
        };

        self.distance += DELTA_SCALE as u32;
        if self.distance >= DISTANCE_MAX  as u32 {
            self.distance = 0;
            self.cur_dir = match self.cur_dir {
                MOUSEDIRE::MOUSEDIRERIGHT => { MOUSEDIRE::MOUSEDIREDOWN },
                MOUSEDIRE::MOUSEDIREDOWN => { MOUSEDIRE::MOUSEDIRELEFT },
                MOUSEDIRE::MOUSEDIRELEFT => { MOUSEDIRE::MOUSEDIREUP },
                MOUSEDIRE::MOUSEDIREUP => { MOUSEDIRE::MOUSEDIRERIGHT },
            }
        }
        (delta_x_ret, delta_y_ret)

    }
    
}



fn send_hid_demo() {
    log::info!("keyboard output send key a ");
    let mut keycode:[u8;6] = [0;6];
    keycode[0] = esp_idf_sys::HID_KEY_A as u8;
    unsafe {
        esp_idf_sys::tud_hid_n_keyboard_report(0,
            esp_idf_sys::hid_interface_protocol_enum_t_HID_ITF_PROTOCOL_KEYBOARD as u8,
            0,
            keycode.as_ptr(),
        )
    };
    vTaskDelay(50);

    unsafe {
        esp_idf_sys::tud_hid_n_keyboard_report(0,
        esp_idf_sys::hid_interface_protocol_enum_t_HID_ITF_PROTOCOL_KEYBOARD as u8,
        0,
        core::ptr::null(),
        )
    };

    let mut mouse = Mouse::new();
    log::info!("send mouse report and draw");
    for _ in 0..(DISTANCE_MAX/DELTA_SCALE) * 4 {
        let (delta_x, delta_y) = mouse.mouse_draw_squre_next_delta();
        unsafe {

            esp_idf_sys::tud_hid_n_mouse_report(
                0,
                esp_idf_sys::hid_interface_protocol_enum_t_HID_ITF_PROTOCOL_MOUSE as u8,
                0x00,
                delta_x,
                delta_y,
                0,
                0,
            )
        };
        vTaskDelay(20);
    };


}

#[allow(non_snake_case)]
fn vTaskDelay(ms: u32) {
    let ms = ((ms as u64 * esp_idf_sys::configTICK_RATE_HZ as u64) / 1000) as u32;
    unsafe {
        esp_idf_sys::vTaskDelay(ms);
    }
}

struct Wakeup {
    suspend: bool,
    wakeuphost: bool,
}

impl Wakeup {
    fn new() -> Self {
        Wakeup { suspend: false, wakeuphost: false }
    }

    fn tud_suspend_cb(&mut self, remote_wakeup_en: bool) {
        log::info!("usb device suspend");
        self.suspend = true;
        if remote_wakeup_en {
            log::info!("remote wakeup avaliable, press button to wake up the host");
            self.wakeuphost = true;
        } else {
            log::info!("remote wakeup not avaliable")
        };
    }

    fn tud_resume_cb(&mut self) {
        log::info!("usb device resumed");
        self.suspend = false;
    }
}

pub(crate) fn example() {
    let boot_button_config = esp_idf_sys::gpio_config_t {
        pin_bit_mask: 1u64 << APP_BUTTON,
        mode: esp_idf_sys::gpio_mode_t_GPIO_MODE_INPUT,
        pull_up_en: esp_idf_sys::gpio_pullup_t_GPIO_PULLUP_ENABLE,
        pull_down_en: esp_idf_sys::gpio_pulldown_t_GPIO_PULLDOWN_DISABLE,
        intr_type: esp_idf_sys::gpio_int_type_t_GPIO_INTR_DISABLE,
        ..Default::default()
    };

    let err = unsafe {
        esp_idf_sys::gpio_config(&boot_button_config)
    };

    if err != esp_idf_sys::ESP_OK {
        panic!("gpio_config falied : {}", err)
    }

    log::info!("usb is initialization");
    let tusb_cfg = unsafe {
        get_tinyusb_default_config()
    };

    log::info!("install driver");
    let err = unsafe {
        esp_idf_sys::tinyusb_driver_install(&tusb_cfg as *const tinyusb_config_t)
    };
    log::info!("err is {}", err);

    if err != esp_idf_sys::ESP_OK {
        panic!("tinyusb driver install error : {}", err)
    }

    log::info!("usb initialization done");
    loop {

        if unsafe { esp_idf_sys::tud_mounted() } {
  
            
            let gpio_level = unsafe {
                esp_idf_sys::gpio_get_level(APP_BUTTON)
            };
            match gpio_level {
                0 => { send_hid_demo()},
                _ => { }
            }
            log::info!("gpio_level is {}", gpio_level);
        }
        vTaskDelay(100);
    }

}
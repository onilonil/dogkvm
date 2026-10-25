#include <stdint.h>
#include "class/hid/hid_device.h"
#include "tinyusb.h"
#include "tinyusb_default_config.h"
#include "esp_log.h"

#define TUSB_DESC_TOTAL_LEN \
    (TUD_CONFIG_DESC_LEN + CFG_TUD_HID * TUD_HID_DESC_LEN)


static const char *TAG = "debug";

static const uint8_t hid_report_descriptor[] = {
    TUD_HID_REPORT_DESC_KEYBOARD(
        HID_REPORT_ID(HID_ITF_PROTOCOL_KEYBOARD)
    ),
    TUD_HID_REPORT_DESC_MOUSE(
        HID_REPORT_ID(HID_ITF_PROTOCOL_MOUSE)
    )
};

static const uint8_t hid_configuration_desc[] = {
    TUD_CONFIG_DESCRIPTOR(
        1, 1, 0,
        TUSB_DESC_TOTAL_LEN,
        TUSB_DESC_CONFIG_ATT_REMOTE_WAKEUP,
        100
    ),

    TUD_HID_DESCRIPTOR(
        0, 4, false,
        sizeof(hid_report_descriptor),
        0x81, 16, 10
    ),
};

const uint8_t *get_hid_configuration_descriptor(void)
{   
    return hid_configuration_desc;
}

uint16_t get_hid_configuration_descriptor_len(void)
{   

    return sizeof(hid_configuration_desc);
}

const uint8_t *get_hid_report_descriptor(void)
{
    return hid_report_descriptor;
}

uint16_t get_hid_report_descriptor_len(void)
{   
    return sizeof(hid_report_descriptor);
}


static const char *hid_string_descriptor[5] = {
    (char[]){0x09, 0x04},
    "TinyUSB",
    "TinyUSB Device",
    "esp32p4",
    "Dogkvm",
};

tinyusb_config_t get_tinyusb_default_config(void)
{
    ESP_LOGE(TAG, "ENTER get_tinyusb_default_config");
    tinyusb_config_t tusb_cfg = TINYUSB_CONFIG_HIGH_SPEED(NULL,NULL);
    tusb_cfg.descriptor.device = NULL;
    ESP_LOGE(TAG, "set full speed config");
    tusb_cfg.descriptor.full_speed_config = hid_configuration_desc;
    tusb_cfg.descriptor.string = hid_string_descriptor;
    tusb_cfg.descriptor.string_count = sizeof(hid_string_descriptor) / sizeof(hid_string_descriptor[0]);
#if (TUD_OPT_HIGH_SPEED)
    tusb_cfg.descriptor.high_speed_config = hid_configuration_desc;
    ESP_LOGE(TAG,"set high speed config");
#endif

    ESP_LOGE(TAG, "CFG_TUD_HID=%d", CFG_TUD_HID);
    ESP_LOGE(TAG, "descriptor=%p", (void *)hid_configuration_desc);
    ESP_LOGE(TAG, "descriptor length=%zu", sizeof(hid_configuration_desc));
    ESP_LOGE(TAG, "full_speed_config=%p",
         (void *)tusb_cfg.descriptor.full_speed_config);
    return tusb_cfg;
}

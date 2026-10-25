#ifndef HID_DESCRIPTOR_H
#define HID_DESCRIPTOR_H

#include <stdint.h>
#include "tinyusb_default_config.h"

const uint8_t *get_hid_report_descriptor(void);
uint16_t get_hid_report_descriptor_len(void);

const uint8_t *get_hid_configuration_descriptor(void);
uint16_t get_hid_configuration_descriptor_len(void);

tinyusb_config_t get_tinyusb_default_config(void);


#endif
#ifdef ESP_IDF_COMP_ESPRESSIF__ESP_TINYUSB_ENABLED
#include "tinyusb.h"
#include "tinyusb_net.h"
#include "tusb_config.h"
#if __has_include("tinyusb_types.h")
// esp_tinyusb < 2.0
#include "tinyusb_types.h"
#if CONFIG_TINYUSB_CDC_ENABLED
#include "tusb_cdc_acm.h"
#endif
#include "tusb_console.h"
#include "tusb_tasks.h"
#ifdef ESP_IDF_COMP_VFS_ENABLED
#include "tusb_msc_storage.h"
#include "vfs_tinyusb.h"
#endif
#else
// esp_tinyusb >= 2.0
#if CONFIG_TINYUSB_CDC_ENABLED
#include "tinyusb_cdc_acm.h"
#include "tinyusb_console.h"
#ifdef ESP_IDF_COMP_VFS_ENABLED
#include "vfs_tinyusb.h"
#endif
#endif
#if CONFIG_TINYUSB_MSC_ENABLED
#include "tinyusb_msc.h"
#endif
#endif
#endif

#include "hid_descriptor.h"
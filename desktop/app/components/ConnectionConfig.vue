<script setup lang="ts">
import type { ConnectionType } from '~/composables/usePrinterConfig'

const connectionType = defineModel<ConnectionType>('connectionType', { required: true })
const usbDevice = defineModel<string>('usbDevice', { required: true })
const ip = defineModel<string>('ip', { required: true })
const port = defineModel<number>('port', { required: true })

const connectionItems = [
  { value: 'ip', label: 'IP Address' },
  { value: 'usb', label: 'USB' },
]
</script>

<template>
  <div class="space-y-4">
    <URadioGroup
      v-model="connectionType"
      legend="Connection Type"
      :items="connectionItems"
      orientation="horizontal"
    />

    <div v-if="connectionType === 'usb'">
      <p class="text-sm font-medium mb-2">USB Device</p>
      <UsbDeviceDropdown v-model="usbDevice" />
    </div>

    <div v-else>
      <IpAddressInput v-model:ip="ip" v-model:port="port" />
    </div>
  </div>
</template>

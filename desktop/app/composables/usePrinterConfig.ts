import { ref, computed } from 'vue'

export type Protocol = 'lpd' | 'ipp'
export type ConnectionType = 'usb' | 'ip'

const protocol = ref<Protocol>('lpd')
const connectionType = ref<ConnectionType>('ip')
const usbDevice = ref<string>('')
const ipAddress = ref<string>('')
const port = ref<number>(515)
const queueName = ref<string>('')
const filePath = ref<string>('')

const host = computed(() =>
  connectionType.value === 'usb' ? usbDevice.value : ipAddress.value
)

export function usePrinterConfig() {
  return {
    protocol,
    connectionType,
    usbDevice,
    ipAddress,
    port,
    queueName,
    filePath,
    host,
  }
}

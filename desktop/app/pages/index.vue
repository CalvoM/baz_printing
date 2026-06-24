<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePrinterConfig } from '~/composables/usePrinterConfig'
import { useTauri } from '~/composables/useTauri'

const {
  protocol,
  connectionType,
  usbDevice,
  ipAddress,
  port,
  queueName,
  filePath,
  host,
} = usePrinterConfig()

const { lpdPrintFile, openFileDialog } = useTauri()

const printing = ref(false)
const printError = ref<string | null>(null)
const printSuccess = ref(false)
const isDragOver = ref(false)

const canPrint = computed(() => !!host.value && !!queueName.value && !!filePath.value)

function onDragOver() {
  isDragOver.value = true
}

function onDragLeave(e: DragEvent) {
  const target = e.currentTarget as HTMLElement
  if (!e.relatedTarget || !target.contains(e.relatedTarget as Node)) {
    isDragOver.value = false
  }
}

function onDrop(e: DragEvent) {
  isDragOver.value = false
  const file = e.dataTransfer?.files?.[0]
  if (file) {
    filePath.value = (file as any).path ?? file.name
  }
}

async function browse() {
  const path = await openFileDialog()
  if (path) filePath.value = path
}

async function print() {
  printing.value = true
  printError.value = null
  printSuccess.value = false
  try {
    await lpdPrintFile(host.value, port.value, queueName.value, filePath.value)
    printSuccess.value = true
  } catch (e: any) {
    printError.value = e?.message ?? String(e)
  } finally {
    printing.value = false
  }
}
</script>

<template>
  <div class="min-h-screen bg-gray-100 dark:bg-gray-900 flex items-center justify-center p-6 transition-colors">
    <div class="w-full max-w-sm rounded-2xl shadow-2xl overflow-hidden">
      <!-- Blue top — file drop zone -->
      <FileDrop
        v-model="filePath"
        :is-drag-over="isDragOver"
        @browse="browse"
        @clear="filePath = ''"
        @dragover="onDragOver"
        @dragleave="onDragLeave"
        @drop="onDrop"
      />

      <!-- White bottom — printer config + print button -->
      <div class="bg-white dark:bg-gray-800">
        <PrinterConfigPanel
          v-model:protocol="protocol"
          v-model:connection-type="connectionType"
          v-model:usb-device="usbDevice"
          v-model:ip-address="ipAddress"
          v-model:port="port"
          v-model:queue-name="queueName"
          :host="host"
        />

        <USeparator />

        <div class="px-5 py-4 space-y-2">
          <p v-if="printError" class="text-xs text-red-500 text-center">{{ printError }}</p>
          <p v-if="printSuccess" class="text-xs text-emerald-500 text-center font-medium">
            Job sent successfully.
          </p>
          <UButton
            block
            size="lg"
            :loading="printing"
            :disabled="!canPrint"
            color="primary"
            class="font-bold tracking-widest rounded-xl"
            label="PRINT"
            @click="print"
          />
        </div>
      </div>
    </div>
  </div>
</template>

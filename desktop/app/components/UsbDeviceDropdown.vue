<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useTauri, type UsbDevice } from '~/composables/useTauri'

const model = defineModel<string>({ required: true })

const { listUsbDevices } = useTauri()
const devices = ref<UsbDevice[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

const selectItems = computed(() =>
  devices.value.map(d => ({ label: d.name, value: d.path }))
)

async function refresh() {
  loading.value = true
  error.value = null
  try {
    devices.value = await listUsbDevices()
    if (devices.value.length && !model.value) {
      model.value = devices.value[0].path
    }
  } catch (e: any) {
    error.value = e?.message ?? String(e)
  } finally {
    loading.value = false
  }
}

onMounted(refresh)
</script>

<template>
  <div class="flex items-center gap-2">
    <USelect
      v-model="model"
      :items="selectItems"
      placeholder="Select USB device"
      :loading="loading"
      class="flex-1"
    />
    <UButton
      icon="i-lucide-refresh-cw"
      :loading="loading"
      color="neutral"
      variant="outline"
      aria-label="Refresh USB devices"
      @click="refresh"
    />
  </div>
  <p v-if="error" class="text-sm text-red-500 mt-1">{{ error }}</p>
  <p v-if="!loading && !error && devices.length === 0" class="text-sm text-gray-500 mt-1">
    No USB printers detected.
  </p>
</template>

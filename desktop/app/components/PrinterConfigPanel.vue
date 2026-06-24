<script setup lang="ts">
import type { Protocol, ConnectionType } from '~/composables/usePrinterConfig'

const props = defineProps<{
  protocol: Protocol
  connectionType: ConnectionType
  usbDevice: string
  ipAddress: string
  port: number
  queueName: string
  host: string
}>()

const emit = defineEmits<{
  'update:protocol': [v: Protocol]
  'update:connectionType': [v: ConnectionType]
  'update:usbDevice': [v: string]
  'update:ipAddress': [v: string]
  'update:port': [v: number]
  'update:queueName': [v: string]
}>()

const configOpen = ref(false)
const queueModalOpen = ref(false)

const summaryTitle = computed(() => {
  if (props.protocol === 'ipp') return 'IPP — Coming Soon'
  if (!props.host) return 'No printer configured'
  if (props.connectionType === 'usb') return 'LPD · USB'
  return `LPD · ${props.ipAddress}`
})

const summarySubtitle = computed(() => {
  if (!props.host || props.protocol === 'ipp') return 'Tap to configure'
  return props.queueName ? `Queue: ${props.queueName}` : 'No queue name set'
})
</script>

<template>
  <div class="bg-white dark:bg-gray-800">
    <UCollapsible v-model:open="configOpen">
      <!-- Trigger row -->
      <template #default="{ open }">
        <button
          class="w-full flex items-center justify-between px-5 py-4 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
        >
          <div class="flex items-center gap-3">
            <UIcon name="i-lucide-printer" class="size-5 text-blue-600 dark:text-blue-400 shrink-0" />
            <div class="text-left">
              <p class="text-sm font-semibold text-gray-900 dark:text-white leading-tight">
                {{ summaryTitle }}
              </p>
              <p class="text-xs text-gray-500 dark:text-gray-400 leading-tight mt-0.5">
                {{ summarySubtitle }}
              </p>
            </div>
          </div>
          <UIcon
            name="i-lucide-chevron-down"
            class="size-4 text-gray-400 transition-transform duration-200 shrink-0"
            :class="open ? 'rotate-180' : ''"
          />
        </button>
      </template>

      <!-- Expanded config -->
      <template #content>
        <div class="px-5 pb-5 pt-3 space-y-4 border-t border-gray-100 dark:border-gray-700">
          <!-- Protocol pill tabs -->
          <div class="flex gap-1 bg-gray-100 dark:bg-gray-700 p-1 rounded-lg">
            <button
              v-for="p in (['lpd', 'ipp'] as Protocol[])"
              :key="p"
              class="flex-1 py-1.5 text-sm font-medium rounded-md transition-colors"
              :class="protocol === p
                ? 'bg-white dark:bg-gray-600 text-blue-600 dark:text-blue-400 shadow-sm'
                : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200'"
              @click="emit('update:protocol', p)"
            >
              {{ p.toUpperCase() }}
            </button>
          </div>

          <!-- LPD config -->
          <template v-if="protocol === 'lpd'">
            <ConnectionConfig
              :connection-type="connectionType"
              :usb-device="usbDevice"
              :ip="ipAddress"
              :port="port"
              @update:connection-type="emit('update:connectionType', $event)"
              @update:usb-device="emit('update:usbDevice', $event)"
              @update:ip="emit('update:ipAddress', $event)"
              @update:port="emit('update:port', $event)"
            />
            <UFormField label="Queue Name">
              <UInput
                :model-value="queueName"
                placeholder="lp"
                class="w-full"
                @update:model-value="emit('update:queueName', $event as string)"
              />
            </UFormField>
            <div class="flex justify-end">
              <UButton
                variant="ghost"
                color="neutral"
                size="xs"
                icon="i-lucide-list"
                label="View Print Queue"
                @click="queueModalOpen = true"
              />
            </div>
          </template>

          <!-- IPP Coming Soon -->
          <template v-else>
            <div class="flex items-center gap-3 py-2">
              <UIcon name="i-lucide-clock" class="size-5 text-gray-400 shrink-0" />
              <div>
                <p class="text-sm font-medium text-gray-700 dark:text-gray-300">IPP — Coming Soon</p>
                <p class="text-xs text-gray-500 dark:text-gray-400">
                  Internet Printing Protocol support is not yet implemented.
                </p>
              </div>
            </div>
          </template>
        </div>
      </template>
    </UCollapsible>

    <!-- Queue Viewer Modal -->
    <UModal v-model:open="queueModalOpen" title="Print Queue Status">
      <template #body>
        <QueueViewer :host="host" :port="port" :queue="queueName" />
      </template>
    </UModal>
  </div>
</template>

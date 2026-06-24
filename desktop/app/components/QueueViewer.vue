<script setup lang="ts">
import { ref } from 'vue'
import { useTauri } from '~/composables/useTauri'

const props = defineProps<{
  host: string
  port: number
  queue: string
}>()

const { lpdQueryQueue } = useTauri()

const username = ref<string>('')
const jobNumber = ref<string>('')
const output = ref<string>('')
const loading = ref(false)
const error = ref<string | null>(null)

const canQuery = computed(() => !!props.host && !!props.queue)

async function viewQueue() {
  loading.value = true
  error.value = null
  output.value = ''
  try {
    output.value = await lpdQueryQueue(
      props.host,
      props.port,
      props.queue,
      username.value || undefined,
      jobNumber.value || undefined
    )
  } catch (e: any) {
    error.value = e?.message ?? String(e)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="space-y-3">
    <div class="flex gap-3 items-end">
      <UFormField label="Username" class="flex-1">
        <UInput v-model="username" placeholder="Optional" class="w-full" />
      </UFormField>
      <UFormField label="Job #" class="w-32">
        <UInput v-model="jobNumber" placeholder="Optional" />
      </UFormField>
      <UButton
        :loading="loading"
        :disabled="!canQuery"
        color="primary"
        @click="viewQueue"
      >
        View Queue
      </UButton>
    </div>

    <p v-if="!canQuery" class="text-sm text-gray-400">
      Configure host and queue name above to query status.
    </p>
    <p v-if="error" class="text-sm text-red-500">{{ error }}</p>

    <pre
      v-if="output"
      class="bg-gray-950 text-green-400 text-xs p-3 rounded-md overflow-auto max-h-48 font-mono"
    >{{ output }}</pre>
  </div>
</template>

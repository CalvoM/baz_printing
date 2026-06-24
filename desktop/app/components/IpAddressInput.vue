<script setup lang="ts">
const ip = defineModel<string>('ip', { required: true })
const port = defineModel<number>('port', { required: true })

const ipPattern = /^(\d{1,3}\.){3}\d{1,3}$/

const ipError = computed(() => {
  if (!ip.value) return null
  if (!ipPattern.test(ip.value)) return 'Enter a valid IPv4 address'
  const octets = ip.value.split('.').map(Number)
  if (octets.some(o => o > 255)) return 'Each octet must be 0–255'
  return null
})
</script>

<template>
  <div class="grid grid-cols-[1fr_auto] gap-3 items-start">
    <UFormField label="Host IP Address" :error="ipError ?? undefined">
      <UInput
        v-model="ip"
        placeholder="192.168.1.100"
        class="w-full"
      />
    </UFormField>
    <UFormField label="Port">
      <UInputNumber
        v-model="port"
        :min="1"
        :max="65535"
        class="w-24"
      />
    </UFormField>
  </div>
</template>

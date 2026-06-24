<script setup lang="ts">
import type { Protocol } from '~/composables/usePrinterConfig'

const model = defineModel<Protocol>({ required: true })

const items = [
  { value: 'lpd', label: 'LPD', description: 'Line Printer Daemon (RFC 1179)' },
  { value: 'ipp', label: 'IPP', description: 'Internet Printing Protocol' },
]
</script>

<template>
  <URadioGroup
    v-model="model"
    legend="Protocol"
    :items="items"
    orientation="horizontal"
  />
</template>

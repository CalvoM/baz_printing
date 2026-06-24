<script setup lang="ts">
const props = defineProps<{
  modelValue: string
  isDragOver: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
  browse: []
  clear: []
  dragover: [e: DragEvent]
  dragleave: [e: DragEvent]
  drop: [e: DragEvent]
}>()

const colorMode = useColorMode()

const fileName = computed(() => {
  const parts = props.modelValue.replace(/\\/g, '/').split('/')
  return parts[parts.length - 1] || props.modelValue
})

function toggleColorMode() {
  colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
}
</script>

<template>
  <div
    class="relative flex flex-col items-center justify-center min-h-[280px] select-none transition-colors duration-200"
    :class="isDragOver ? 'bg-blue-500' : 'bg-blue-600 dark:bg-blue-700'"
    @dragover.prevent="emit('dragover', $event)"
    @dragleave="emit('dragleave', $event)"
    @drop.prevent="emit('drop', $event)"
  >
    <!-- Dark mode toggle -->
    <button
      class="absolute top-3 right-3 p-1.5 rounded-lg bg-white/10 hover:bg-white/20 text-white transition-colors"
      @click="toggleColorMode"
    >
      <UIcon
        :name="colorMode.value === 'dark' ? 'i-lucide-sun' : 'i-lucide-moon'"
        class="size-4"
      />
    </button>

    <!-- App branding -->
    <div class="absolute top-4 left-4 flex items-center gap-2">
      <UIcon name="i-lucide-printer" class="size-5 text-white/80" />
      <span class="text-white/80 text-sm font-semibold tracking-wide">BAZ Print</span>
    </div>

    <!-- Printer illustration -->
    <PrinterIllustration
      :class="modelValue ? 'size-20 mb-4 mt-8' : 'size-32 mb-5 mt-10'"
    />

    <!-- Empty state -->
    <template v-if="!modelValue">
      <p class="text-white/75 text-sm text-center px-10 mb-5 leading-relaxed">
        <strong class="text-white font-semibold">Drag and drop</strong> your file here<br>to print
      </p>
      <UButton
        color="white"
        variant="solid"
        icon="i-lucide-plus"
        label="ADD FILE"
        class="font-bold tracking-widest mb-8"
        @click="emit('browse')"
      />
    </template>

    <!-- File selected state -->
    <template v-else>
      <div class="relative mb-4">
        <div
          class="bg-white/15 text-white text-sm font-medium px-5 py-2 rounded-full max-w-[240px] truncate"
          :title="modelValue"
        >
          {{ fileName }}
        </div>
        <button
          class="absolute -top-1.5 -right-1.5 size-5 rounded-full bg-red-500 text-white flex items-center justify-center text-sm font-bold leading-none hover:bg-red-600 transition-colors"
          @click.stop="emit('clear')"
        >
          −
        </button>
      </div>
      <UButton
        color="white"
        variant="outline"
        label="CHANGE FILE"
        class="text-white border-white/40 hover:bg-white/10 font-bold tracking-widest mb-8"
        @click="emit('browse')"
      />
    </template>
  </div>
</template>

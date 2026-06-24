<script setup lang="ts">
defineProps<{ class?: string }>()
</script>

<template>
  <svg
    :class="$props.class"
    viewBox="0 0 80 72"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    aria-hidden="true"
  >
    <!-- Paper tray (top) -->
    <rect x="22" y="4" width="36" height="20" rx="3" stroke="white" stroke-width="2.5" stroke-opacity="0.9" />
    <!-- Printer body -->
    <rect x="8" y="22" width="64" height="28" rx="5" stroke="white" stroke-width="2.5" stroke-opacity="0.9" />
    <!-- Paper output slot -->
    <rect x="22" y="50" width="36" height="18" rx="3" stroke="white" stroke-width="2.5" stroke-opacity="0.65" />
    <!-- Output paper line -->
    <line x1="28" y1="58" x2="52" y2="58" stroke="white" stroke-width="2" stroke-opacity="0.5" stroke-linecap="round" />
    <line x1="28" y1="63" x2="46" y2="63" stroke="white" stroke-width="2" stroke-opacity="0.35" stroke-linecap="round" />
    <!-- Status dots -->
    <circle cx="22" cy="37" r="3" fill="white" fill-opacity="0.5" />
    <circle cx="31" cy="37" r="3" fill="white" fill-opacity="0.75" />
    <!-- Ready indicator -->
    <circle cx="58" cy="37" r="4" fill="white" fill-opacity="0.2" stroke="white" stroke-width="1.5" stroke-opacity="0.7" />
    <circle cx="58" cy="37" r="2" fill="white" fill-opacity="0.8" />
  </svg>
</template>

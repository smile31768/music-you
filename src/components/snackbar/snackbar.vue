<template>
  <v-snackbar
    :color="color"
    :style="snackbarStyle"
    :model-value="show as Boolean"
    :timeout="options.timeout ?? 2000"
    :location="location"
  >
    {{ options.message }}
    <slot></slot>
    <template v-if="options.action" #actions>
      <v-btn color="indigo" variant="text" @click="handleAction"> Close </v-btn>
    </template>
  </v-snackbar>
</template>

<script setup lang="ts">
import { defineProps, PropType } from 'vue'

import type { SnackbarLocation, SnackbarOption } from '@/components/snackbar/interface'
const emit = defineEmits<{
  (event: 'close'): void
}>()
const props = defineProps<{
  show: boolean
  options: SnackbarOption
}>()

const location = computed<SnackbarLocation>(() => {
  return props.options.location ?? 'bottom'
})
const snackbarStyle = computed(() => {
  const offset = props.options.offset ?? 0
  return {
    bottom: {
      bottom: `${offset}px`,
    },
    top: {
      top: `${offset}px`,
    },
  }[location.value]
})

const color = computed(() => {
  return {
    info: '',
    success: 'primary',
    error: 'error',
    warning: 'yellow',
  }[props.options.type ?? 'info']
})

function handleAction() {
  if (typeof props.options.action === 'function') {
    props.options.action()
  }
  emit('close')
}
</script>

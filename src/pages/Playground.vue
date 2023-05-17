<script lang="ts" setup>
import { useSnackbar } from '@/components/snackbar'
import { usePlayer } from '@/player/player'
const wave = ref()
const player = usePlayer()
const snackbar = useSnackbar()
function showToast() {
  snackbar.info('233', {
    action: () => {
      console.log('2333')
    },
    timeout: 20000,
    offset: 86,
  })
}
</script>

<template>
  <v-container class="d-flex justify-center">
    <v-btn variant="tonal" rounded="md" color="primary" @click="showToast">show toast</v-btn>
  </v-container>
</template>

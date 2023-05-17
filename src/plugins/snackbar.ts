import type { App } from 'vue'

import { createSnackbar } from '@/components/snackbar'

export function useSnackbar(app: App) {
  const snackbar = createSnackbar()
  app.use(snackbar)
}

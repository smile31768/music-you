import type { App, AppContext } from 'vue'
import { h, render } from 'vue'

import SnackbarComp from './snackbar.vue'
import { genContainer } from './utils'
let appContext: AppContext

export interface SnackbarOption {
  message: string
  timeout?: number
  type?: TYPE
  action?: () => void
  _context?: AppContext
  offset?: number
  location?: SnackbarLocation
}

export type SnackbarLocation = 'bottom' | 'top'

export enum TYPE {
  SUCCESS = 'success',
  INFO = 'info',
  WARNING = 'warning',
  ERROR = 'error',
}

const renderSnackbar = (options: SnackbarOption, container: HTMLElement) => {
  const vNode = h(SnackbarComp, {
    options: options,
    show: true,
    onClose: () => {
      render(null, container)
    },
  })
  vNode.appContext = options._context ?? appContext
  render(vNode, container)
  // document.body.appendChild(container.firstElementChild as Node)
  return vNode.component
}
const _snackbar = (options: SnackbarOption) => {
  const container = genContainer()
  renderSnackbar(options ?? {}, container)
}

const buildSnackbar = () => {
  return {
    success: createSnackbarMethod(TYPE.SUCCESS),
    info: createSnackbarMethod(TYPE.INFO),
    warning: createSnackbarMethod(TYPE.WARNING),
    error: createSnackbarMethod(TYPE.ERROR),
  }
}

function createSnackbarMethod(type: TYPE) {
  return (message: string, options?: SnackbarOption) => _snackbar({ ...options, message, type })
}

function mixinAppContext(context: AppContext) {
  appContext = context
}

function createSnackbar() {
  const install = (app: App<Element>): void => {
    app.config.globalProperties.$snackbar = buildSnackbar()
    mixinAppContext(app._context)
  }
  return {
    install,
  }
}

const useSnackbar = () => {
  return buildSnackbar()
}
export { createSnackbar, useSnackbar }

export interface UsbDevice {
  name: string
  path: string
}

function getInvoke(): (cmd: string, args?: Record<string, unknown>) => Promise<unknown> {
  if (typeof window !== 'undefined' && (window as any).__TAURI__?.core?.invoke) {
    return (window as any).__TAURI__.core.invoke
  }
  // Graceful fallback during browser-only dev
  return () => Promise.reject(new Error('Tauri runtime not available'))
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  return getInvoke()(cmd, args) as Promise<T>
}

export function useTauri() {
  async function listUsbDevices(): Promise<UsbDevice[]> {
    return invoke<UsbDevice[]>('list_usb_devices')
  }

  async function lpdPrintFile(
    host: string,
    port: number,
    queue: string,
    filePath: string
  ): Promise<void> {
    return invoke<void>('lpd_print_file', { host, port, queue, file_path: filePath })
  }

  async function lpdQueryQueue(
    host: string,
    port: number,
    queue: string,
    username?: string,
    jobNumber?: string
  ): Promise<string> {
    return invoke<string>('lpd_query_queue', {
      host,
      port,
      queue,
      username: username || null,
      job_number: jobNumber || null,
    })
  }

  async function openFileDialog(): Promise<string | null> {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog')
      const result = await open({ multiple: false })
      if (!result) return null
      return typeof result === 'string' ? result : (result as any).path ?? null
    } catch {
      return null
    }
  }

  return { listUsbDevices, lpdPrintFile, lpdQueryQueue, openFileDialog }
}

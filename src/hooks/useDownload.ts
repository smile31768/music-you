import { downloadFile } from '@/util/fn'

export default function useDownload(url: string, fileName?: string) {
  downloadFile(url, fileName)
}

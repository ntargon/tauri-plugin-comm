import { invoke } from '@tauri-apps/api/tauri'

export async function execute() : Promise<string> {
  return await invoke<string>('plugin:comm|execute')
}

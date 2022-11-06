import { invoke } from '@tauri-apps/api/tauri'

export async function connect() : Promise<unknown> {
  return await invoke('plugin:comm|connect')
}

export async function disconnect() : Promise<unknown> {
  return await invoke('plugin:comm|disconnect')
}

export async function sendAndReceive(request: string) : Promise<string> {
  return await invoke('plugin:comm|send_and_receive', {request})
}

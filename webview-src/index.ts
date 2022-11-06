import { invoke } from '@tauri-apps/api/tauri'

export async function connect(addr: string) : Promise<unknown> {
  return await invoke('plugin:comm|connect', {addr})
}

export async function connectTimeout(addr: string, timeoutMs: number) : Promise<unknown> {
  return await invoke('plugin:comm|connect', {addr, timeoutMs})
}

export async function disconnect() : Promise<unknown> {
  return await invoke('plugin:comm|disconnect')
}

export async function sendAndReceive(request: string) : Promise<string> {
  return await invoke('plugin:comm|send_and_receive', {request})
}

import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { LoginParams, OAuthResult } from '../types/login'

export function openLogin(params: LoginParams): Promise<void> {
  return invoke('open_login', { params })
}

export function onResult(cb: (result: OAuthResult) => void): Promise<UnlistenFn> {
  return listen<OAuthResult>('oauth://result', e => cb(e.payload))
}

export function onLog(cb: (message: string) => void): Promise<UnlistenFn> {
  return listen<string>('oauth://log', e => cb(e.payload))
}

export function onCancelled(cb: () => void): Promise<UnlistenFn> {
  return listen('oauth://cancelled', () => cb())
}

export type Provider = 'wechat' | 'qq'

export interface LoginParams {
  provider: Provider
  appId: string
  redirectUri: string
  state: string
}

export interface OAuthResult {
  provider: string
  code: string | null
  state: string | null
  rawQuery: string
  isError: boolean
  url: string
}

export interface LogEntry {
  time: string
  message: string
}

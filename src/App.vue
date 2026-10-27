<script setup lang="ts">
import { ref } from 'vue'
import LoginPanel from './components/LoginPanel.vue'
import ResultPanel from './components/ResultPanel.vue'
import EventLogPanel from './components/EventLogPanel.vue'
import type { LogEntry, OAuthResult, Provider } from './types/login'

const provider = ref<Provider>('wechat')
const result = ref<OAuthResult | null>(null)
const expectedState = ref('')
const logs = ref<LogEntry[]>([])

function addLog(message: string) {
  logs.value.push({ time: new Date().toLocaleTimeString(), message })
}
</script>

<template>
  <div class="shell">
    <header class="header">
      <h1>扫码登录 Demo</h1>
      <span class="subtitle">微信开放平台 / QQ 互联 OAuth 二维码登录</span>
    </header>

    <nav class="tabs">
      <button
        :class="['tab', provider === 'wechat' && 'active']"
        @click="provider = 'wechat'; result = null"
      >微信登录</button>
      <button
        :class="['tab', provider === 'qq' && 'active']"
        @click="provider = 'qq'; result = null"
      >QQ 登录</button>
    </nav>

    <main class="content">
      <LoginPanel
        :provider="provider"
        v-model:result="result"
        v-model:state="expectedState"
        @log="addLog"
      />
      <ResultPanel :result="result" :expected-state="expectedState" />
      <EventLogPanel :logs="logs" />
    </main>
  </div>
</template>

<style scoped>
.shell {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 20px 24px;
  gap: 16px;
  overflow: auto;
}

.header h1 {
  font-size: 20px;
  font-weight: 600;
}

.subtitle {
  color: var(--text-dim);
  font-size: 12px;
}

.tabs {
  display: flex;
  gap: 8px;
}

.tab {
  padding: 8px 20px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--panel);
  color: var(--text-dim);
  font-size: 14px;
}

.tab.active {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
}

.content {
  display: flex;
  flex-direction: column;
  gap: 16px;
  flex: 1;
  min-height: 0;
}
</style>

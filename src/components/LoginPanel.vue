<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { openLogin, onCancelled, onLog, onResult } from '../services/login'
import type { OAuthResult, Provider } from '../types/login'

const props = defineProps<{ provider: Provider }>()
const emit = defineEmits<{ log: [message: string] }>()
const result = defineModel<OAuthResult | null>('result', { required: true })
const state = defineModel<string>('state', { required: true })

const appId = ref('')
const redirectUri = ref('https://example.com/callback')
const error = ref('')

// state 仅在无值时自动生成（首次挂载）；重新生成由用户手动触发
if (!state.value) state.value = crypto.randomUUID()

function regenerateState() {
  state.value = crypto.randomUUID()
}

const canStart = computed(() => appId.value.trim() !== '' && redirectUri.value.trim() !== '')

const providerLabel = computed(() => (props.provider === 'wechat' ? '微信' : 'QQ'))

async function start() {
  error.value = ''
  result.value = null
  try {
    await openLogin({
      provider: props.provider,
      appId: appId.value.trim(),
      redirectUri: redirectUri.value.trim(),
      state: state.value,
    })
  } catch (e) {
    error.value = String(e)
  }
}

let unlisteners: Array<() => void> = []

onMounted(async () => {
  const unsubs = await Promise.all([
    onResult(r => (result.value = r)),
    onLog(m => emit('log', m)),
    onCancelled(() => emit('log', '已取消登录（窗口被关闭）')),
  ])
  unlisteners = unsubs
})

onUnmounted(() => unlisteners.forEach(u => u()))
</script>

<template>
  <section class="panel">
    <h2 class="title">{{ providerLabel }}扫码登录</h2>

    <label class="field">
      <span class="label">AppID{{ provider === 'qq' ? '（client_id）' : '' }}</span>
      <input v-model="appId" :placeholder="provider === 'wechat' ? '开放平台 AppID，如 wx1234567890abcdef' : 'QQ 互联 AppID，如 101234567'" />
    </label>

    <label class="field">
      <span class="label">Redirect URI</span>
      <input v-model="redirectUri" placeholder="https://your.domain/callback" />
      <span class="hint">必须与开放平台/互联后台配置的授权回调域一致，扫码确认后由本应用拦截，不会真正跳转</span>
    </label>

    <label class="field">
      <span class="label">State</span>
      <div class="row">
        <input v-model="state" class="state-input" readonly />
        <button class="btn ghost" @click="regenerateState">重新生成</button>
      </div>
    </label>

    <div class="row">
      <button class="btn primary" :disabled="!canStart" @click="start">开始扫码登录</button>
      <span v-if="error" class="error">{{ error }}</span>
    </div>
  </section>
</template>

<style scoped>
.panel {
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.title {
  font-size: 15px;
  font-weight: 600;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.label {
  color: var(--text-dim);
  font-size: 12px;
}

.hint {
  color: var(--text-dim);
  font-size: 11px;
}

input {
  background: var(--panel-2);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 8px 10px;
  color: var(--text);
  font-size: 13px;
  outline: none;
}

input:focus {
  border-color: var(--accent);
}

.state-input {
  flex: 1;
  color: var(--text-dim);
}

.row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn {
  padding: 8px 16px;
  border-radius: 6px;
  border: 1px solid var(--border);
  background: var(--panel-2);
  color: var(--text);
  font-size: 13px;
}

.btn.ghost {
  white-space: nowrap;
}

.btn.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
  font-size: 14px;
}

.btn.primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn.primary:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.error {
  color: var(--err);
  font-size: 12px;
}
</style>

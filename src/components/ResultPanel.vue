<script setup lang="ts">
import { computed, ref } from 'vue'
import type { OAuthResult } from '../types/login'

const props = defineProps<{ result: OAuthResult | null; expectedState: string }>()

const copied = ref('')

const stateMismatch = computed(
  () => props.result != null && props.result.state !== props.expectedState,
)

async function copy(text: string, tag: string) {
  try {
    await navigator.clipboard.writeText(text)
    copied.value = tag
    setTimeout(() => (copied.value = ''), 1500)
  } catch {
    /* 剪贴板不可用时忽略 */
  }
}
</script>

<template>
  <section v-if="result" class="panel" :class="result.isError && 'is-error'">
    <h2 class="title">
      登录结果
      <span v-if="result.isError" class="badge err">回调带错误标记</span>
      <span v-else class="badge ok">已拦截回调</span>
    </h2>

    <div class="row">
      <span class="label">code</span>
      <code class="value">{{ result.code ?? '（无）' }}</code>
      <button v-if="result.code" class="btn" @click="copy(result.code!, 'code')">
        {{ copied === 'code' ? '已复制' : '复制' }}
      </button>
    </div>

    <div class="row">
      <span class="label">state</span>
      <code class="value">{{ result.state ?? '（无）' }}</code>
      <span v-if="stateMismatch" class="warn">⚠ 与发起时的 state 不一致</span>
    </div>

    <div class="row">
      <span class="label">原始查询串</span>
      <code class="value small">{{ result.rawQuery || '（空）' }}</code>
      <button v-if="result.rawQuery" class="btn" @click="copy(result.rawQuery, 'query')">
        {{ copied === 'query' ? '已复制' : '复制' }}
      </button>
    </div>

    <div class="row">
      <span class="label">完整回调 URL</span>
      <code class="value small">{{ result.url }}</code>
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
  gap: 10px;
}

.panel.is-error {
  border-color: var(--err);
}

.title {
  font-size: 15px;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 8px;
}

.badge {
  font-size: 11px;
  font-weight: 400;
  padding: 2px 8px;
  border-radius: 10px;
}

.badge.ok {
  color: var(--ok);
  background: rgba(63, 191, 127, 0.12);
}

.badge.err {
  color: var(--err);
  background: rgba(224, 90, 90, 0.12);
}

.row {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  font-size: 13px;
}

.label {
  color: var(--text-dim);
  min-width: 88px;
  padding-top: 4px;
}

.value {
  background: var(--panel-2);
  border-radius: 4px;
  padding: 4px 8px;
  word-break: break-all;
}

.value.small {
  font-size: 11px;
}

.warn {
  color: var(--warn);
  font-size: 11px;
  padding-top: 4px;
  white-space: nowrap;
}

.btn {
  padding: 4px 10px;
  border-radius: 4px;
  border: 1px solid var(--border);
  background: var(--panel-2);
  color: var(--text-dim);
  font-size: 11px;
  white-space: nowrap;
}
</style>

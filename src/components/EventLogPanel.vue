<script setup lang="ts">
import type { LogEntry } from '../types/login'

defineProps<{ logs: LogEntry[] }>()
</script>

<template>
  <section class="panel">
    <h2 class="title">事件日志</h2>
    <div class="log">
      <p v-if="logs.length === 0" class="empty">暂无事件。点击「开始扫码登录」后，这里会显示登录窗口的导航与拦截记录。</p>
      <p v-for="(entry, i) in logs" :key="i" class="entry">
        <span class="time">{{ entry.time }}</span>
        {{ entry.message }}
      </p>
    </div>
  </section>
</template>

<style scoped>
.panel {
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 16px;
  flex: 1;
  min-height: 120px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.title {
  font-size: 15px;
  font-weight: 600;
}

.log {
  flex: 1;
  overflow: auto;
  background: var(--panel-2);
  border-radius: 6px;
  padding: 8px 10px;
  font-size: 12px;
  line-height: 1.7;
}

.empty {
  color: var(--text-dim);
}

.time {
  color: var(--text-dim);
  margin-right: 8px;
}

.entry {
  word-break: break-all;
}
</style>

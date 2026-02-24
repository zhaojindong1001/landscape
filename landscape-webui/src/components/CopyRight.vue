<script setup lang="ts">
import { Information, Api } from "@vicons/carbon";

const icon = defineModel<boolean>("icon", { default: false });
</script>
<template>
  <n-flex
    align="center"
    :justify="icon ? 'start' : 'center'"
    :style="{ padding: '0 10px', width: icon ? '100%' : undefined }"
  >
    <n-flex
      v-if="icon"
      align="center"
      justify="space-between"
      style="width: 100%"
    >
      <n-popover trigger="hover">
        <template #trigger>
          <n-button text>
            <n-icon>
              <Information />
            </n-icon>
          </n-button>
        </template>
        <n-flex>
          <n-button
            text
            tag="a"
            href="https://github.com/ThisSeanZhang/landscape"
            target="_blank"
            type="primary"
          >
            Landscape Router
          </n-button>
          is licensed under the
          <n-button
            text
            tag="a"
            href="https://github.com/ThisSeanZhang/landscape/blob/main/LICENSE"
            target="_blank"
            type="primary"
            >GPL-3.0 License</n-button
          >.
        </n-flex>
      </n-popover>
      <n-tooltip trigger="hover">
        <template #trigger>
          <n-button text tag="a" href="/api/docs" target="_blank">
            <n-icon>
              <Api />
            </n-icon>
          </n-button>
        </template>
        API Docs
      </n-tooltip>
    </n-flex>
    <n-flex v-else>
      <n-button
        text
        tag="a"
        href="https://github.com/ThisSeanZhang/landscape"
        target="_blank"
        type="primary"
      >
        Landscape Router
      </n-button>
      is licensed under the
      <n-button
        text
        tag="a"
        href="https://github.com/ThisSeanZhang/landscape/blob/main/LICENSE"
        target="_blank"
        type="primary"
        >GPL-3.0 License</n-button
      >.
    </n-flex>
  </n-flex>
</template>

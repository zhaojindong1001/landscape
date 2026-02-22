<script setup lang="ts">
import { ref, computed } from "vue";
import { ChangeCatalog } from "@vicons/carbon";
import { trace_flow_match, trace_verdict } from "@/api/route/trace";
import { check_domain } from "@/api/dns_service";
import { useEnrolledDeviceStore } from "@/stores/enrolled_device";
import FlowExhibit from "@/components/flow/FlowExhibit.vue";
import type { FlowMatchResult } from "landscape-types/common/route_trace";
import type { FlowVerdictResult } from "landscape-types/common/route_trace";

const show = defineModel<boolean>("show", { required: true });

const enrolledDeviceStore = useEnrolledDeviceStore();

// Step 1 state
const selectMode = ref(true);
const selectedDevice = ref<string | null>(null);
const srcIp = ref("");
const srcMac = ref("");
const matchLoading = ref(false);
const matchResult = ref<FlowMatchResult | null>(null);

// Step 2 state
const queryMode = ref<"domain" | "ip">("domain");
const domainInput = ref("");
const ipInput = ref("");
const verdictLoading = ref(false);
const verdictResult = ref<FlowVerdictResult | null>(null);
const resolvedDomain = ref("");

const deviceOptions = computed(() =>
  enrolledDeviceStore.bindings
    .filter((d) => d.ipv4)
    .map((d) => ({
      label: `${d.name} (${d.ipv4})`,
      value: d.mac,
    })),
);

function onDeviceSelect(mac: string) {
  selectedDevice.value = mac;
  const device = enrolledDeviceStore.bindings.find((d) => d.mac === mac);
  if (device) {
    srcIp.value = device.ipv4 || "";
    srcMac.value = device.mac || "";
  }
}

async function doFlowMatch() {
  if (!srcIp.value) return;
  matchLoading.value = true;
  matchResult.value = null;
  verdictResult.value = null;
  try {
    matchResult.value = await trace_flow_match({
      src_ip: srcIp.value,
      src_mac: srcMac.value || undefined,
    } as any);
  } finally {
    matchLoading.value = false;
  }
}

async function doVerdictByDomain() {
  if (!domainInput.value || !matchResult.value) return;
  verdictLoading.value = true;
  verdictResult.value = null;
  resolvedDomain.value = domainInput.value;
  try {
    const dnsResult = await check_domain({
      flow_id: matchResult.value.effective_flow_id,
      domain: domainInput.value,
      record_type: "A" as any,
    });

    const ips: string[] = [];
    if (dnsResult.records) {
      for (const r of dnsResult.records) {
        if (r.rr_type === "A") {
          ips.push(r.data);
        }
      }
    }

    if (ips.length === 0) {
      window.$message?.warning("DNS 解析未返回 A 记录");
      return;
    }

    verdictResult.value = await trace_verdict({
      flow_id: matchResult.value.effective_flow_id,
      src_ip: srcIp.value,
      dst_ips: ips,
    } as any);
  } finally {
    verdictLoading.value = false;
  }
}

async function doVerdictByIp() {
  if (!ipInput.value || !matchResult.value) return;
  verdictLoading.value = true;
  verdictResult.value = null;
  resolvedDomain.value = "";
  try {
    verdictResult.value = await trace_verdict({
      flow_id: matchResult.value.effective_flow_id,
      src_ip: srcIp.value,
      dst_ips: [ipInput.value],
    } as any);
  } finally {
    verdictLoading.value = false;
  }
}

function onOpen() {
  enrolledDeviceStore.UPDATE_INFO();
}

function formatAction(mark: { action: { t: string }; flow_id: number }) {
  switch (mark.action.t) {
    case "keep_going":
      return "KeepGoing (继续)";
    case "direct":
      return "Direct (直连)";
    case "drop":
      return "Drop (丢弃)";
    case "redirect":
      return `Redirect → Flow ${mark.flow_id}`;
    default:
      return mark.action.t;
  }
}
</script>

<template>
  <n-drawer
    v-model:show="show"
    width="500px"
    placement="right"
    @after-enter="onOpen"
  >
    <n-drawer-content
      title="分流追踪"
      closable
      :native-scrollbar="false"
      body-content-style="padding: 14px 16px"
    >
      <n-flex vertical :size="16">
        <!-- Step 1: Source client -->
        <n-card size="small" title="第一步：匹配源客户端">
          <n-flex vertical :size="8">
            <n-flex :wrap="false" align="center">
              <n-button size="small" @click="selectMode = !selectMode">
                <template #icon>
                  <n-icon><ChangeCatalog /></n-icon>
                </template>
              </n-button>
              <template v-if="selectMode">
                <n-select
                  :options="deviceOptions"
                  :value="selectedDevice"
                  @update:value="onDeviceSelect"
                  placeholder="选择已登记设备"
                  clearable
                  filterable
                  style="flex: 1"
                />
              </template>
              <template v-else>
                <n-input
                  v-model:value="srcIp"
                  placeholder="源 IP"
                  style="flex: 1"
                />
              </template>
            </n-flex>
            <n-input
              v-if="!selectMode"
              v-model:value="srcMac"
              placeholder="源 MAC (可选)"
            />
            <n-text
              v-if="selectMode && srcIp"
              depth="3"
              style="font-size: 12px"
            >
              IP: {{ srcIp }} &nbsp; MAC: {{ srcMac || "无" }}
            </n-text>
            <n-button
              type="primary"
              :loading="matchLoading"
              :disabled="!srcIp"
              @click="doFlowMatch"
              block
              size="small"
            >
              查询 Flow 匹配
            </n-button>
          </n-flex>
        </n-card>

        <!-- Flow match result -->
        <n-card v-if="matchResult" size="small" title="Flow 匹配结果">
          <n-descriptions
            :column="1"
            label-placement="left"
            bordered
            size="small"
          >
            <n-descriptions-item label="MAC 匹配">
              <FlowExhibit
                v-if="matchResult.flow_id_by_mac != null"
                :flow_id="matchResult.flow_id_by_mac"
              />
              <n-tag v-else type="default" size="small">无匹配</n-tag>
            </n-descriptions-item>
            <n-descriptions-item label="IP 匹配">
              <FlowExhibit
                v-if="matchResult.flow_id_by_ip != null"
                :flow_id="matchResult.flow_id_by_ip"
              />
              <n-tag v-else type="default" size="small">无匹配</n-tag>
            </n-descriptions-item>
            <n-descriptions-item label="生效 Flow">
              <FlowExhibit :flow_id="matchResult.effective_flow_id" />
            </n-descriptions-item>
          </n-descriptions>
        </n-card>

        <!-- Step 2: Verdict query (shown after flow match) -->
        <template v-if="matchResult">
          <n-card size="small" title="第二步：查询目标">
            <n-flex vertical :size="8">
              <n-radio-group v-model:value="queryMode" size="small">
                <n-radio-button value="domain">域名查询</n-radio-button>
                <n-radio-button value="ip">IP 查询</n-radio-button>
              </n-radio-group>

              <!-- Domain mode -->
              <template v-if="queryMode === 'domain'">
                <n-input
                  v-model:value="domainInput"
                  placeholder="输入域名，如 www.baidu.com"
                />
                <n-button
                  type="primary"
                  :loading="verdictLoading"
                  :disabled="!domainInput"
                  @click="doVerdictByDomain"
                  block
                  size="small"
                >
                  DNS 解析并查询
                </n-button>
              </template>

              <!-- IP mode -->
              <template v-else>
                <n-input
                  v-model:value="ipInput"
                  placeholder="输入目标 IP 地址"
                />
                <n-button
                  type="primary"
                  :loading="verdictLoading"
                  :disabled="!ipInput"
                  @click="doVerdictByIp"
                  block
                  size="small"
                >
                  查询
                </n-button>
              </template>
            </n-flex>
          </n-card>
        </template>

        <!-- Verdict results -->
        <template v-if="verdictResult">
          <n-text v-if="resolvedDomain" depth="3" style="font-size: 12px">
            域名 {{ resolvedDomain }} 解析到
            {{ verdictResult.verdicts.length }} 个 IP
          </n-text>
          <n-card
            v-for="(v, idx) in verdictResult.verdicts"
            :key="idx"
            size="small"
            :title="v.dst_ip"
          >
            <n-descriptions
              :column="1"
              label-placement="left"
              bordered
              size="small"
            >
              <n-descriptions-item label="IP 规则">
                <template v-if="v.ip_rule_match">
                  mark={{ v.ip_rule_match.mark }}, priority={{
                    v.ip_rule_match.priority
                  }}
                </template>
                <n-tag v-else type="default" size="small">无匹配</n-tag>
              </n-descriptions-item>
              <n-descriptions-item label="DNS 规则">
                <template v-if="v.dns_rule_match">
                  mark={{ v.dns_rule_match.mark }}, priority={{
                    v.dns_rule_match.priority
                  }}
                </template>
                <n-tag v-else type="default" size="small">无匹配</n-tag>
              </n-descriptions-item>
              <n-descriptions-item label="最终动作">
                <n-tag type="info" size="small">
                  {{ formatAction(v.effective_mark as any) }}
                </n-tag>
              </n-descriptions-item>
              <n-descriptions-item label="缓存">
                <template v-if="!v.has_cache">
                  <n-tag type="default" size="small">无缓存</n-tag>
                </template>
                <template v-else>
                  <n-flex align="center" :size="4">
                    <n-tag
                      :type="v.cache_consistent ? 'success' : 'warning'"
                      size="small"
                    >
                      {{ v.cache_consistent ? "一致" : "不一致" }}
                    </n-tag>
                    <n-text depth="3" style="font-size: 12px">
                      cached={{ v.cached_mark }}
                    </n-text>
                  </n-flex>
                </template>
              </n-descriptions-item>
            </n-descriptions>
            <n-alert
              v-if="v.has_cache && !v.cache_consistent"
              type="warning"
              style="margin-top: 8px"
            >
              缓存中的 mark 值 ({{ v.cached_mark }})
              与当前配置计算的结果不一致，可能需要清理路由缓存。
            </n-alert>
          </n-card>
        </template>
      </n-flex>
    </n-drawer-content>
  </n-drawer>
</template>

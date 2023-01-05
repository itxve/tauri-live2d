<script setup lang="ts">
import { readConfig, writeConfig } from "@/util";
import { autostart, checkupdate } from "@/plugins";
import {
  WebviewWindow,
  PhysicalPosition,
  appWindow,
} from "@tauri-apps/api/window";
import {
  NButton,
  NForm,
  NSwitch,
  NTabPane,
  NFormItem,
  NSpace,
  NTabs,
} from "naive-ui";
import { onMounted, ref } from "vue";
import Models from "./Models.vue";

const autoStartRef = ref(false);
const checkUpdateRef = ref(true);
const tabRef = ref();

async function Close() {
  const live2d = WebviewWindow.getByLabel("main");
  if (live2d) {
    await live2d.close();
  }
}

async function reset() {
  const config = await readConfig();
  config.x = 100;
  config.y = 120;
  await writeConfig(JSON.stringify(config));
}

async function openNew() {
  const mainview = WebviewWindow.getByLabel("main");
  if (mainview) {
    await mainview.show();
    return;
  }
  const config = await readConfig();
  const x = config.x || 100;
  const y = config.y || 120;
  const width = config.width || 215;
  const height = config.height || 200;
  const webview = new WebviewWindow("main", {
    url: "/live2d.html",
    transparent: true,
    width,
    height,
    minWidth: 215,
    minHeight: 200,
    resizable: false,
    decorations: false,
    skipTaskbar: true,
    alwaysOnTop: true,
  });
  const factor = await appWindow.scaleFactor();
  webview.setPosition(new PhysicalPosition(x, y).toLogical(factor));
  webview.once("tauri://sucess", function (e) {
    alert("桌宠创建成功");
  });
  webview.once("tauri://error", function (e) {
    alert("桌宠已经存在");
  });
}
async function enableMouse() {
  const live2d = WebviewWindow.getByLabel("main");
  await live2d?.setIgnoreCursorEvents(false);
  alert("鼠标事件开启成功");
}

async function switchCheckUpdate(e: boolean) {
  if (e) {
    await checkupdate.check_version_update();
  }
  checkUpdateRef.value = e;
  const config = await readConfig();
  config.check_update = e;
  await writeConfig(JSON.stringify(config));
}

async function switchAutoStart(e: boolean) {
  if (e) {
    await autostart.enable();
  } else {
    await autostart.disable();
  }
  autoStartRef.value = e;
  const config = await readConfig();
  config.auto_start = e;
  await writeConfig(JSON.stringify(config));
}

async function beforeLeave(name: string, oldName: string) {
  localStorage.setItem("tab_switch", name);
  tabRef.value = name;
  return true;
}

onMounted(async () => {
  const config = await readConfig();
  tabRef.value = localStorage.getItem("tab_switch") || "config";
  autoStartRef.value = await autostart.isEnabled();
  checkUpdateRef.value = config.check_update!;
});
</script>

<template>
  <div class="card">
    <n-tabs
      type="card"
      animated
      @before-leave="beforeLeave"
      v-model:value="tabRef"
    >
      <n-tab-pane name="config" tab="配置">
        <n-form
          ref="formRef"
          label-placement="left"
          label-width="120px"
          require-mark-placement="right-hanging"
        >
          <n-form-item label="开机启动">
            <n-switch
              @update-value="switchAutoStart"
              v-model:value="autoStartRef"
              size="medium"
            >
              <template #icon> 🌈 </template>
            </n-switch>
          </n-form-item>
          <n-form-item label="检查更新">
            <n-switch
              @update-value="switchCheckUpdate"
              v-model:value="checkUpdateRef"
              size="medium"
            >
              <template #icon> ⬆️ </template>
            </n-switch>
          </n-form-item>
          <n-form-item label="live2d操作">
            <n-space>
              <n-button type="info" tertiary @click="openNew()">
                打开桌宠
              </n-button>
              <n-button type="info" tertiary @click="Close()">
                关闭桌宠
              </n-button>
              <n-button type="info" tertiary @click="reset()">
                重置位置
              </n-button>
              <n-button type="info" tertiary @click="enableMouse()">
                开启捕获鼠标事件
              </n-button>
            </n-space>
          </n-form-item>
        </n-form>
      </n-tab-pane>
      <n-tab-pane name="live2d" tab="live2d模型"> <Models /> </n-tab-pane>
    </n-tabs>
  </div>
</template>

<style scoped>
.search {
  width: 100%;
  display: flex;
  flex-direction: row;
  justify-content: center;
  margin: 0 15px 15px;
}
.draft-div {
  margin: 0 20px;
}
.card {
  margin: 15px;
}
</style>

<script setup lang="ts">
import { modelConfig } from "@/hooks/useModel";
import { JY_FILE_NAME, readConfig, writeConfig } from "@/util";
import { RustCallResult } from "@/types";
import { enable, disable } from "@/plugins";
import {
  WebviewWindow,
  PhysicalPosition,
  appWindow,
} from "@tauri-apps/api/window";
import { NButton, NForm, NInput, NSwitch } from "naive-ui";
import { onMounted, ref } from "vue";

const autoStartRef = ref(false);

async function Close() {
  const live2d = WebviewWindow.getByLabel("main");
  if (live2d) {
    await live2d.close();
  }
}

async function save() {
  if (modelConfig.value) {
    const config = await readConfig();
    config.api = modelConfig.value;
    await writeConfig(JSON.stringify(config));
    alert(modelConfig.value);
    await Close();
    await OpenNew();
  }
}

async function reset() {
  const config = await readConfig();
  config.x = 100;
  config.y = 120;
  await writeConfig(JSON.stringify(config));
}

async function OpenNew() {
  const config = await readConfig();
  const x = config.x || 100;
  const y = config.y || 120;
  const width = config.width || 280;
  const height = config.height || 300;

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

async function switchAutoStart(e: boolean) {
  if (e) {
    await enable();
  } else {
    await disable();
  }
  autoStartRef.value = e;
  const config = await readConfig();
  config.auto_start = e;
  await writeConfig(JSON.stringify(config));
}
onMounted(async () => {
  const config = await readConfig();
  autoStartRef.value = config.auto_start || false;
  modelConfig.value = config.api || "https://live2d.fghrsh.net/api/";
});
</script>

<template>
  <div class="card">
    <n-form
      ref="formRef"
      label-placement="left"
      label-width="120px"
      require-mark-placement="right-hanging"
    >
      <!-- <span>
        https://live2d.fghrsh.net/api/
        <br />
        https://api.zsq.im/live2d/
      </span> -->
      <n-form-item label="模型API地址">
        <n-space>
          <n-input style="width: 300px" v-model:value="modelConfig" />
          <n-button type="success" @click="save">保存</n-button>
        </n-space>
      </n-form-item>
      <n-form-item label="开机启动">
        <n-switch
          @update-value="switchAutoStart"
          v-model:value="autoStartRef"
          size="medium"
        >
          <template #icon> 🌈 </template>
        </n-switch>
      </n-form-item>
      <!-- <n-form-item label="一言API">
        <n-select
          @update-value="switchAutoStart"
          v-model:value="autoStartRef"
          size="medium"
        >
         
        </n-select>
      </n-form-item> -->
      <n-form-item label="live2d操作">
        <n-space>
          <n-button type="info" tertiary @click="OpenNew"> 打开桌宠 </n-button>
          <n-button type="info" tertiary @click="Close()"> 关闭桌宠 </n-button>
          <n-button type="info" tertiary @click="reset()"> 重置位置 </n-button>
          <n-button type="info" tertiary @click="enableMouse()">
            开启捕获鼠标事件
          </n-button>
        </n-space>
      </n-form-item>
    </n-form>
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

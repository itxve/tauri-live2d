<script setup lang="ts">
import "@/assets/waifu.css";
import "@/assets/live2d";
import "@/assets/waifu-tips";
import { initModel } from "@/assets/waifu-tips";
import { onMounted, onUnmounted, ref } from "vue";
import waifuTip from "@/assets/waifu-tips.json";
import {
  WebviewWindow,
  appWindow,
  PhysicalPosition,
} from "@tauri-apps/api/window";

import { readConfig, writeConfig, throttle } from "@/util";
const winSize = ref({ width: 500, height: 400 });
const factorRef = ref(1);
const onMoveRef = ref();
const onResizeRef = ref();
const isResizeRef = ref();

onMounted(async () => {
  const live2dwebview = WebviewWindow.getByLabel("main");
  onMoveRef.value = await live2dwebview?.onMoved(
    throttle(async ({ payload }) => {
      const config = await readConfig();
      config.x = payload.x;
      config.y = payload.y;
      console.log("positioon:", payload);
      await writeConfig(JSON.stringify(config));
    }, 100)
  );
});

onMounted(async () => {
  const live2dwebview = WebviewWindow.getByLabel("main");
  const size = await live2dwebview?.innerSize();
  winSize.value.width = size?.width!;
  winSize.value.height = size?.height!;
  const factor = await appWindow.scaleFactor();
  factorRef.value = factor;
  onResizeRef.value = await live2dwebview?.onResized(
    throttle(async ({ payload }) => {
      winSize.value.width = payload.width;
      winSize.value.height = payload.height;
      const config = await readConfig();
      config.width = payload.width / factorRef.value;
      config.height = payload.height / factorRef.value;
      await writeConfig(JSON.stringify(config));
      window.live2d_settings["waifuSize"] = `${config.width}x${config.height}`;
      window.live2d_settings["waifuTipsSize"] = `${config.width}x40`;
    }, 100)
  );
});

onMounted(() => {
  const live2dwebview = WebviewWindow.getByLabel("main");
  live2dwebview?.onFocusChanged(({ payload }) => {
    if (!payload) {
      isResizeRef.value = false;
      live2dwebview.setResizable(false);
    }
  });
});

onMounted(async () => {
  const config = await readConfig();
  window.live2d_settings["modelAPI"] =
    config.api || "https://live2d.fghrsh.net/api/"; // 自建 API 修改这里
    
  window.live2d_settings["modelId"] = 3;
  window.live2d_settings["waifuSize"] = `${config.width}x${config.height}`;
  window.live2d_settings["waifuTipsSize"] = `${config.width}x40`;
  initModel(waifuTip);
});

onUnmounted(() => {
  if (onMoveRef.value) {
    onMoveRef.value();
  }
  if (onResizeRef.value) {
    onResizeRef.value();
  }
});

async function closeIt() {
  const live2dwebview = WebviewWindow.getByLabel("main");
  setTimeout(() => {
    live2dwebview?.close();
  }, 1300);
}

async function openConfigWin() {
  const config = await readConfig();
  const x = config.x || 100;
  const y = config.y || 120;

  const webview = new WebviewWindow("config", {
    url: "/index.html",
    center: true,
    resizable: true,
    title: "live2d看板娘配置",
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

async function resizeStart() {
  const live2dwebview = WebviewWindow.getByLabel("main");

  await live2dwebview?.setResizable(true);
  const isResizable = await live2dwebview?.isResizable();
  isResizeRef.value = isResizable;
}
async function ignoreCursorEvents() {
  const live2dwebview = WebviewWindow.getByLabel("main");
  live2dwebview?.setIgnoreCursorEvents(true);
  alert("鼠标事件穿透开启");
}
</script>

<template>
  <div
    :class="{ 'live2d-view': true, edit: isResizeRef }"
    :style="{
      width: (winSize.width - 16 * factorRef) / factorRef + 'px',
      height: (winSize.height - 16 * factorRef) / factorRef + 'px',
    }"
  >
    <div class="waifu">
      <div class="waifu-tips"></div>
      <canvas
        data-tauri-drag-region
        :style="{
          width: winSize.width / factorRef + 'px',
          height: winSize.height / factorRef + 'px',
        }"
        id="live2d"
        class="live2d"
      ></canvas>
      <div class="waifu-tool">
        <span class="fui-gear" @click="openConfigWin"></span>
        <span class="fui-chat"></span>
        <span class="fui-eye"></span>
        <span class="fui-user"></span>
        <span class="fui-window" @click="resizeStart"></span>
        <span class="fui-lock" @click="ignoreCursorEvents"></span>
        <span class="fui-cross" @click="closeIt"></span>
      </div>
    </div>
  </div>
</template>
<!-- https://designmodo.github.io/Flat-UI/#fakelink -->
<style scoped>
.fui-window:before {
  content: "\e621";
}
.fui-lock:before {
  content: "\e633";
}
.fui-gear:before {
  content: "\e636";
}
.edit {
  border: 2px dashed rgb(208, 200, 200);
  cursor: move;
}
canvas {
  cursor: move;
}
.waifu-tool span {
  cursor: pointer;
}
body {
  overflow: hidden;
}
.waifu-tips {
  margin-top: 0;
}
</style>

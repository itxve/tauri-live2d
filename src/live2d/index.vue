<script setup lang="ts">
import "@/assets/waifu.css";
import * as PIXI from "pixi.js";
import { InternalModel, Live2DModel } from "pixi-live2d-display";
import { onMounted, onUnmounted, ref } from "vue";
import {
  WebviewWindow,
  appWindow,
  PhysicalPosition,
  PhysicalSize,
} from "@tauri-apps/api/window";

import { readConfig, writeConfig, throttle } from "@/util";
import { model_list } from "@/plugins";
import useListenEvent from "@/hooks/useListenEvent";
window.PIXI = PIXI;
const winSize = ref({ width: 500, height: 400 });
const factorRef = ref(1);
const onMoveRef = ref();
const onFocusRef = ref();
const onResizeRef = ref();
const isResizeRef = ref();
const buttonModeRef = ref(false);
const canvasRef = ref<HTMLCanvasElement>();
const modelRef = ref<Live2DModel<InternalModel>>();
const modelUrlRef = ref();

useListenEvent("loadOrtherModel", async ({ windowLabel, payload }) => {
  if (windowLabel == "config" && payload) {
    modelUrlRef.value = payload;
    await loadModel(payload);
    await reloadPositionScale();
  }
});

onMounted(async () => {
  const config = await readConfig();
  const live2dwebview = WebviewWindow.getByLabel("main");
  const factor = await appWindow.scaleFactor();
  factorRef.value = factor;
  live2dwebview?.setPosition(
    new PhysicalPosition(config.x!, config.y!).toLogical(factor)
  );

  live2dwebview?.setSize(
    new PhysicalSize(config.width!, config.height!).toLogical(factor)
  );

  onFocusRef.value = await live2dwebview?.onFocusChanged(
    async ({ payload }) => {
      if (!payload) {
        isResizeRef.value = false;
        await live2dwebview.setResizable(false);
        // 禁止移动
        buttonModeRef.value = false;
        offdraggable(modelRef.value);
        appRef.value?.screen;
      } else {
        canvasRef.value?.setAttribute("data-tauri-drag-region", "true");
      }
    }
  );
  onMoveRef.value = await live2dwebview?.onMoved(
    throttle(async ({ payload }) => {
      const config = await readConfig();
      config.x = payload.x;
      config.y = payload.y;
      await writeConfig(JSON.stringify(config));
    }, 100)
  );

  onResizeRef.value = await live2dwebview?.onResized(
    throttle(async ({ payload }) => {
      winSize.value.width = payload.width / factorRef.value;
      winSize.value.height = payload.height / factorRef.value;
      modelRef.value!.width = payload.width / factorRef.value;
      modelRef.value!.height = payload.height / factorRef.value;
      const config = await readConfig();
      config.width = payload.width;
      config.height = payload.height;
      await writeConfig(JSON.stringify(config));
    }, 100)
  );
});
// 组合写法
async function keydownEvent(event) {
  // 当然还要组织浏览器的默认事件
  event.preventDefault();
  var key = event.keyCode || event.which;
  var shiftKey = event.shiftKey || event.metaKey;
  if (shiftKey) {
    if (key == 189) {
      modelRef.value?.scale.set(
        modelRef.value?.scale.x - 0.01,
        modelRef.value?.scale.y - 0.01
      );
    }
    if (key == 187) {
      modelRef.value?.scale.set(
        modelRef.value?.scale.x + 0.01,
        modelRef.value?.scale.y + 0.01
      );
    }
    const config = await readConfig();
    if (!config[modelUrlRef.value]) {
      config[modelUrlRef.value] = {};
    }
    config[modelUrlRef.value].scale = {
      x: modelRef.value?.scale.x,
      y: modelRef.value?.scale.y,
    };
    await writeConfig(JSON.stringify(config));
  }
}

function on_keydownEvent() {
  document.addEventListener("keydown", keydownEvent);
}
function off_keydownEvent() {
  document.removeEventListener("keydown", keydownEvent);
}

onUnmounted(() => {
  if (onMoveRef.value) {
    onMoveRef.value();
  }
  if (onResizeRef.value) {
    onResizeRef.value();
  }
  if (onFocusRef.value) {
    onFocusRef.value();
  }

  off_keydownEvent();
});

async function closeIt() {
  const live2dwebview = WebviewWindow.getByLabel("main");
  setTimeout(() => {
    live2dwebview?.close();
  }, 1300);
}

async function openConfigWin() {
  const configwebview = WebviewWindow.getByLabel("config");
  if (configwebview) {
    configwebview.show();
    return;
  }
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
  webview.once("tauri://sucess", function (e) {});
  webview.once("tauri://error", function (e) {
    webview.show();
  });
}
/**
 * 开启大小调整
 */
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

const appRef = ref<PIXI.Application>();
onMounted(async () => {
  const app = new PIXI.Application({
    view: document.getElementById("live2d") as HTMLCanvasElement,
    resizeTo: window,
    // backgroundColor: 0x333333,
    backgroundAlpha: 0,
  });
  appRef.value = app;

  if (localStorage.getItem("murl")) {
    modelUrlRef.value = localStorage.getItem("murl");
  } else {
    await getModelUrl();
  }
  await loadModel(modelUrlRef.value);
  await reloadPositionScale();
  canvasRef.value?.setAttribute("data-tauri-drag-region", "true");
});

async function reloadPositionScale() {
  const config = await readConfig();
  // 还原位置
  if (config[modelUrlRef.value]?.position) {
    const { x, y } = config[modelUrlRef.value].position;
    modelRef.value?.position.set(x, y);
  }
  // 还原缩放
  if (config[modelUrlRef.value]?.scale) {
    const { x, y } = config[modelUrlRef.value].scale;
    modelRef.value?.scale.set(x, y);
  }
}

async function getModelUrl() {
  let arr = await model_list();
  const modelUrl = arr[Math.floor(Math.random() * arr.length)];
  modelUrlRef.value = modelUrl.url;
}

async function loadModel(uri) {
  if (modelRef.value) {
    await appRef.value?.stage.removeChild(modelRef.value);
  }
  const model = await Live2DModel.from(uri);
  appRef.value?.stage.addChild(model);
  modelRef.value = model;
  const live2dwebview = WebviewWindow.getByLabel("main");
  const size = await live2dwebview?.innerSize();
  const factor = await appWindow.scaleFactor();
  model.width = size?.width! / factor;
  model.height = size?.height! / factor;
  model.on("hit", async (hitAreas) => {
    if (hitAreas.includes("body")) {
      model.motion("tap_body");
    }
    console.log("hitAreas", hitAreas);

    if (hitAreas.includes("head")) {
      model.expression();
    }
  });
  localStorage.setItem("murl", uri);
}

function draggable(model) {
  model.on("pointerdown", (e) => {
    model.dragging = true;
    model._pointerX = e.data.global.x - model.x;
    model._pointerY = e.data.global.y - model.y;
  });
  model.on("pointermove", async (e) => {
    if (model.dragging) {
      model.position.x = e.data.global.x - model._pointerX;
      model.position.y = e.data.global.y - model._pointerY;
      const config = await readConfig();
      if (!config[modelUrlRef.value]) {
        config[modelUrlRef.value] = {};
      }
      config[modelUrlRef.value].position = {
        x: model.position.x,
        y: model.position.y,
      };
      await writeConfig(JSON.stringify(config));
    }
  });
  model.on("pointerupoutside", () => (model.dragging = false));
  model.on("pointerup", () => (model.dragging = false));
}

function offdraggable(model) {
  if (!model) {
    return;
  }
  model.off("pointerdown");
  model.off("pointermove");
  model.off("pointerupoutside");
  model.off("pointerup");
}

/**
 * 更换模型
 */
async function nextModel() {
  await getModelUrl();
  await loadModel(modelUrlRef.value);
  await reloadPositionScale();
}
/**
 *
 * @param model 模型位置移动
 */
async function positionMove(model) {
  buttonModeRef.value = !buttonModeRef.value;
  model.buttonMode = buttonModeRef.value;
  if (buttonModeRef.value) {
    canvasRef.value?.removeAttribute("data-tauri-drag-region");
    draggable(model);
    on_keydownEvent();
  } else {
    canvasRef.value?.setAttribute("data-tauri-drag-region", "true");
    offdraggable(model);
    off_keydownEvent();
  }
}
</script>

<template>
  <div
    :width="winSize.width - 32 + 'px'"
    :height="winSize.height - 40 + 'px'"
    :class="{ 'live2d-view': true, edit: isResizeRef }"
  >
    <div class="waifu">
      <canvas
        ref="canvasRef"
        :width="winSize.width"
        :height="winSize.height"
        id="live2d"
        class="live2d"
      />

      <div class="waifu-tool">
        <span class="fui-gear" @click="openConfigWin"></span>
        <!-- <span class="fui-chat"></span> -->
        <span class="fui-eye" @click="nextModel"></span>
        <span
          class="fui-location"
          :style="{
            color: buttonModeRef ? 'green' : '',
          }"
          @click="positionMove(modelRef)"
        ></span>
        <span class="fui-window" @click="resizeStart"></span>
        <span class="fui-lock" @click="ignoreCursorEvents"></span>
        <span class="fui-cross" @click="closeIt"></span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.live2d-view {
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0;
  left: 0;
}

.waifu-tool span {
  font-size: 18px;
  margin-right: 10px;
  margin-top: 5px;
}
.edit {
  border: 2px dashed rgb(208, 200, 200);
  cursor: move;
}
.waifu-tool {
  margin: 0;
}

.waifu-tool span {
  cursor: pointer;
}

body,
html {
  overflow: hidden;
}
</style>

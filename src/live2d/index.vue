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
import { writeText } from "@tauri-apps/api/clipboard";
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
const modelBlockRef = ref(false);
const appRef = ref<PIXI.Application>();
const blockForeGroundRef = ref<PIXI.Sprite>();

useListenEvent("load-model", async ({ windowLabel, payload }) => {
  if (windowLabel == "config" && payload) {
    modelUrlRef.value = payload;
    await loadModel(payload);
    await reloadPositionScale();
  }
});

useListenEvent("refresh-model", async ({ windowLabel }) => {
  if (windowLabel == "config") {
    await getModelUrl();
    await loadModel(modelUrlRef.value);
    await reloadPositionScale();
  }
});
/**
 * 初始化
 */
async function init() {
  const app = new PIXI.Application({
    view: document.getElementById("live2d") as HTMLCanvasElement,
    resizeTo: window,
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
  await loadModelBlockState();
  canvasRef.value?.setAttribute("data-tauri-drag-region", "true");
}

/**
 * 一些监听事件
 */
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

  await init();
});
/**
 * 缩放按键 在可移动模型下生效
 * @param event
 */
async function keydownEvent(event) {
  // 浏览器的默认事件
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
/**
 * 卸载监听器
 */
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
    title: "配置",
    alwaysOnTop: true,
  });
  const factor = await appWindow.scaleFactor();
  webview.setPosition(new PhysicalPosition(x, y).toLogical(factor));
  webview.once("tauri://sucess", function (e) {});
  webview.once("tauri://error", function (e) {});
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
  const ok = await confirm("确认开启鼠标事件穿透吗？");
  if (ok) {
    const live2dwebview = WebviewWindow.getByLabel("main");
    live2dwebview?.setIgnoreCursorEvents(true);
    alert("鼠标事件穿透开启");
  }
}

/**
 * 还原模型的位置和缩放
 */
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

/**
 * 获取一个模型的url
 */
async function getModelUrl() {
  let arr = await model_list();
  const modelUrl = arr[Math.floor(Math.random() * arr.length)];
  modelUrlRef.value =
    modelUrl?.url ||
    "https://cdn.jsdelivr.net/gh/guansss/pixi-live2d-display/test/assets/shizuku/shizuku.model.json";
}

/**
 * 加载模型
 * @param uri 模型url
 */
async function loadModel(uri) {
  if (modelRef.value) {
    await modelRef.value.destroy();
    modelRef.value = undefined;
  }
  const model = await Live2DModel.from(uri)
    .then((m) => m)
    .catch((e) => {
      alert(`模型加载失败 : [ ${e}]`);
      return undefined;
    });
  if (!model) {
    return;
  }
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
  tryAddFrame();
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
 * 更换模型 mode:随机
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

async function tryAddFrame() {
  if (blockForeGroundRef.value) {
    await blockForeGroundRef.value.destroy();
  }
  const mianview = WebviewWindow.getByLabel("main");
  const innerSize = await mianview?.innerSize();
  const foreground = PIXI.Sprite.from(PIXI.Texture.WHITE);
  foreground.width = innerSize?.width!;
  foreground.height = innerSize?.height!;
  foreground.alpha = 0;
  blockForeGroundRef.value = foreground;
  appRef.value?.stage.addChild(foreground);
}

/**
 * 显示背景块
 */
async function changeModelBlock() {
  modelBlockRef.value = !modelBlockRef.value;
  if (modelBlockRef.value) {
    if (blockForeGroundRef.value) {
      blockForeGroundRef.value.alpha = 0.2;
    }
  } else {
    if (blockForeGroundRef.value) {
      blockForeGroundRef.value.alpha = 0;
    }
  }
  const config = await readConfig();
  config.model_block = modelBlockRef.value;
  await writeConfig(JSON.stringify(config));
}
/**
 * 初始化背景块
 */
async function loadModelBlockState() {
  const config = await readConfig();
  modelBlockRef.value = config.model_block === false ? false : true;
  if (modelBlockRef.value) {
    if (blockForeGroundRef.value) {
      blockForeGroundRef.value.alpha = 0.2;
    }
  } else {
    if (blockForeGroundRef.value) {
      blockForeGroundRef.value.alpha = 0;
    }
  }
}

async function openModel() {
  const murl = localStorage.getItem("murl");
  if (murl) {
    alert(`复制当前模型成功,可在浏览器查看`);
    await writeText(murl);
  }
}
</script>

<template>
  <div
    :width="winSize.width"
    :height="winSize.height"
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
        <span
          :class="{
            'fui-checkbox-unchecked': true,
            block: modelBlockRef,
          }"
          @click="changeModelBlock"
        >
        </span>
        <!-- <span class="fui-chat"></span> -->
        <span class="fui-eye" @click="nextModel"></span>
        <span
          class="fui-location"
          :style="{
            color: buttonModeRef ? '#117be6' : '',
          }"
          @click="positionMove(modelRef)"
        ></span>
        <span class="fui-window" @click="resizeStart"></span>

        <span class="fui-alert-circle" @click="openModel"></span>
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
  font-size: 17px;
  margin-right: 10px;
  margin-top: 5px;
}
.edit {
  border: 2px dashed rgb(208, 200, 200);
  cursor: move;
}
.waifu-tool {
  margin: 0;
  top: 15px;
}
.block {
  color: #117be6;
}

.waifu-tool span {
  cursor: pointer;
}

body,
html {
  overflow: hidden;
}
</style>

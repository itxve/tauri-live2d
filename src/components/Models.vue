<template>
  <n-space vertical>
    <div>
      <n-space>
        <n-input
          style="width: 400px"
          readonly
          v-model:value="servePathRef"
          placeholder="本地模型路径"
        ></n-input>
        <n-button size="small" round @click="fileSelect()">
          选择本地模型
        </n-button>
        <n-button size="small" round @click="reloadIt"> 刷新 </n-button>
        <n-button size="small" round @click="showModalRef = true">
          添加网络模型
        </n-button>
      </n-space>
    </div>
    <n-table striped>
      <thead>
        <tr>
          <th>类型</th>
          <th>模型位置</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="it in listRef" :key="it.url">
          <td>{{ it.type }}</td>
          <td>{{ it.url }}</td>
          <td>
            <n-space>
              <n-button @click="loadModel(it.url)"> 加载 </n-button>
              <n-button
                v-if="it.type == 'remote'"
                @click="removeLoadModel(it.url)"
              >
                删除
              </n-button>
            </n-space>
          </td>
        </tr>
      </tbody>
    </n-table>

    <n-modal
      v-model:show="showModalRef"
      :bordered="false"
      title="添加网络模型"
      preset="dialog"
      content="你确认"
      positive-text="确认"
      negative-text="取消"
      @positive-click="addRemote"
      @negative-click="showModalRef = false"
    >
      <n-input
        v-model:value="textareaRef"
        placeholder="请输入模型地址"
        type="textarea"
      ></n-input>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import * as dialog from "@tauri-apps/api/dialog";
import { emit } from "@tauri-apps/api/event";
import { relaunch } from "@tauri-apps/api/process";
import { readConfig, writeConfig } from "@/util";
import { NSpace, NTable, NButton, NInput, NModal } from "naive-ui";
import { modelserve } from "@/plugins";

const servePathRef = ref();
const listRef = ref<Array<modelserve.Live2dModelItem>>([]);
const showModalRef = ref(false);
const textareaRef = ref<string>();

async function reloadIt() {
  window.location.reload();
}

async function addRemote() {
  let errors: string[] = [];
  if (textareaRef.value) {
    const textareaValue = textareaRef.value.split(",") || [];
    textareaValue.forEach((it) => {
      if (listRef.value.findIndex((mod) => mod.url.trim() == it.trim()) == -1) {
        listRef.value.push({
          url: it.trim(),
          type: "remote",
        } as modelserve.Live2dModelItem);
      }
    });
    await saveLocalModels();
  }
  textareaRef.value = "";
}

async function saveLocalModels() {
  const config = await readConfig();
  config.remote_list = listRef.value
    .map((it) => it.url)
    .filter((it) => !it.startsWith("http://localhost:13004"));
  await writeConfig(JSON.stringify(config));
  await reloadModels();
}

async function removeLoadModel(url: string) {
  listRef.value = listRef.value.filter((it) => it.url != url);
  await saveLocalModels();
}

async function loadModel(path: string) {
  if (path.startsWith("https://") || path.startsWith("http://")) {
    await emit("load-model", path);
  } else {
    alert("错误的模型");
    return;
  }
}
async function fileSelect() {
  const selected = await dialog.open({
    title: "选择本地model",
    multiple: false,
    directory: true,
  });

  if (selected) {
    const config = await readConfig();
    const serve_path = config.serve_path;
    config.serve_path = selected as string;
    servePathRef.value = selected;
    await writeConfig(JSON.stringify(config));
    // 第一次
    if (!serve_path && selected) {
      await emit("refresh-model", "");
      window.location.reload();
    } else if (selected != serve_path) {
      alert("模型目录变更，需要重启软件!!!");
      setTimeout(async () => {
        await relaunch();
      }, 1300);
    }
  }
}

onMounted(async () => {
  await reloadModels();
});

async function reloadModels() {
  const modelList = await modelserve.model_list();
  if (modelList) {
    listRef.value = modelList;
    const config = await readConfig();
    servePathRef.value = config.serve_path;
  }
}
</script>

<style scoped></style>

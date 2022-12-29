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
          <td><n-button @click="loadModel(it.url)"> 加载 </n-button></td>
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
      <n-input v-model:value="textareaRef" type="textarea"></n-input>
    </n-modal>
  </n-space>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import * as dialog from "@tauri-apps/api/dialog";
import { emit } from "@tauri-apps/api/event";
import { readConfig, writeConfig } from "@/util";
import { NSpace, NTable, NButton, NInput, NModal } from "naive-ui";
import { model_list, Live2dModelItem } from "@/plugins";

const servePathRef = ref();
const listRef = ref<Array<Live2dModelItem>>([]);
const showModalRef = ref(false);
const textareaRef = ref<string>();

async function reloadIt() {
  window.location.reload();
}

async function addRemote() {
  if (textareaRef.value) {
    const textareaValue = textareaRef.value.split(",") || [];
    textareaValue.map((it) => {
      if (listRef.value.findIndex((mod) => mod.url.trim() == it.trim()) == -1) {
        listRef.value.push({
          url: it.trim(),
          type: "remote",
        } as Live2dModelItem);
      }
    });
    showModalRef.value = false;
    const config = await readConfig();
    config.remote_list = listRef.value.map((it) => it.url);
    await writeConfig(JSON.stringify(config));
  }
}

async function loadModel(path: string) {
  const patt =
    path.indexOf("http") != -1 ? path : `http://localhost:3004${path}`;
  await emit("loadOrtherModel", patt);
}
async function fileSelect() {
  const selected = await dialog.open({
    title: "选择本地model",
    multiple: false,
    directory: true,
  });

  if (selected) {
    const config = await readConfig();
    config.serve_path = selected as string;
    servePathRef.value = selected;
    await writeConfig(JSON.stringify(config));
  }
}

onMounted(async () => {
  const modelList = await model_list();
  if (modelList) {
    listRef.value = modelList;
    const config = await readConfig();
    servePathRef.value = config.serve_path;
  }
});
</script>

<style scoped></style>

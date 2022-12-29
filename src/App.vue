<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Live2dConfig from "./components/Live2dConfig.vue";
import { repo, version } from "../package.json";
</script>

<template>
  <div class="container">
    <Live2dConfig />
    <footer class="footer">
      <div>
        © 2022
        <a target="_blank" :href="repo">
          <b>GitHub</b>
        </a>
        V{{ version }}
      </div>
    </footer>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>

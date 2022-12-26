import { createApp } from "vue";
import Live2dApp from "./index.vue";

declare global {
  interface Window {
    live2d_settings: any;
  }
}

createApp(Live2dApp).mount("#live2d-app");

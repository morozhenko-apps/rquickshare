import { createApp } from 'vue'
import { createPinia } from 'pinia'

import './vue_lib/assets/main.postcss'

import App from './App.vue'

const pinia = createPinia();
const app = createApp(App)

app.use(pinia);

app.mount('#app')

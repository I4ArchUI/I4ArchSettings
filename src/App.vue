<script setup lang="ts">
import { RouterView } from "vue-router";
import MainLayout from "./layouts/MainLayout.vue";
import ToastContainer from "@/components/common/ToastContainer.vue";
import { onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const updateTheme = async () => {
  try {
    const theme = await invoke('get_gtk_theme');
    if (theme === 'dark') {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  } catch (e) {}
};

let themeInterval: any;

onMounted(() => {
  updateTheme();
  // Poll every 3 seconds for theme changes
  themeInterval = setInterval(updateTheme, 3000);
});

onUnmounted(() => {
  if (themeInterval) clearInterval(themeInterval);
});
</script>

<template>
  <MainLayout>
    <RouterView />
  </MainLayout>
  <ToastContainer />
</template>
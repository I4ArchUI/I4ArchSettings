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
    <RouterView v-slot="{ Component }">
      <Transition name="page-fade" mode="out-in">
        <component :is="Component" />
      </Transition>
    </RouterView>
  </MainLayout>
  <ToastContainer />
</template>

<style>
/* Page transition: gentle fade + subtle upward slide */
.page-fade-enter-active {
  transition: opacity 0.35s cubic-bezier(0.25, 0.46, 0.45, 0.94),
              transform 0.35s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.page-fade-leave-active {
  transition: opacity 0.2s cubic-bezier(0.55, 0, 1, 0.45),
              transform 0.2s cubic-bezier(0.55, 0, 1, 0.45);
}

.page-fade-enter-from {
  opacity: 0;
  transform: translateY(12px);
}

.page-fade-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
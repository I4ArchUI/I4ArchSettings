<script setup lang="ts">
import AppSidebar from '../components/AppSidebar.vue';
</script>

<template>
    <div class="main-window">
        <aside class="sidebar-pane">
            <AppSidebar />
        </aside>
        <main class="content-pane">
            <slot></slot>
        </main>
    </div>
</template>

<style scoped>
.main-window {
    display: flex;
    width: 100%;
    height: 100%;
    border-radius: var(--window-border-radius);
    overflow: hidden;
    background: var(--content-bg); /* Fallback */
    box-shadow: 0 0 20px rgba(0,0,0,0.2);
}

.sidebar-pane {
    width: var(--sidebar-width);
    background-color: var(--sidebar-bg);
    border-right: 1px solid var(--separator-color);
    padding: 10px 2px;
    display: flex;
    flex-direction: column;
    backdrop-filter: blur(20px); /* Glass effect */
}

.content-pane {
    flex: 1;
    padding-top: 20px;
    background-color: var(--content-bg);
    overflow-y: auto;
    display: flex;
    flex-direction: column;
}
</style>

<script setup lang="ts">
import { RouterView } from "vue-router";
import MainLayout from "./layouts/MainLayout.vue";
import ToastContainer from "@/components/common/ToastContainer.vue";
import ModalDialog from "@/components/common/ModalDialog.vue";
import { useKeyboardShortcuts } from "./composables/useKeyboardShortcuts";
import { onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

// Register global keyboard shortcuts
const { showShortcutsHelp } = useKeyboardShortcuts();

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

  <!-- STUNNING GLOBAL KEYBOARD SHORTCUTS CHEATSHEET MODAL -->
  <ModalDialog 
      v-model="showShortcutsHelp" 
      title="Bảng Phím tắt Hệ thống (Shortcuts)"
      @close="showShortcutsHelp = false"
  >
      <div class="shortcuts-help-container">
          <!-- Navigation Section -->
          <div class="shortcuts-section">
              <h4 class="section-title"><i class="pi pi-compass" style="margin-right: 6px;"></i> Di chuyển nhanh (Alt + Phím)</h4>
              <div class="shortcuts-grid">
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>1</kbd> / <kbd>W</kbd></span>
                      <span class="shortcut-desc">Cài đặt Wi-Fi</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>2</kbd> / <kbd>V</kbd></span>
                      <span class="shortcut-desc">Cấu hình VPN</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>3</kbd> / <kbd>B</kbd></span>
                      <span class="shortcut-desc">Quản lý Bluetooth</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>4</kbd> / <kbd>P</kbd></span>
                      <span class="shortcut-desc">Hình nền & Màu sắc</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>5</kbd> / <kbd>T</kbd></span>
                      <span class="shortcut-desc">Tùy biến Giao diện</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>6</kbd> / <kbd>D</kbd></span>
                      <span class="shortcut-desc">Cấu hình Màn hình</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>7</kbd> / <kbd>A</kbd></span>
                      <span class="shortcut-desc">Ứng dụng đã cài đặt</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>8</kbd></span>
                      <span class="shortcut-desc">Ứng dụng Khởi chạy</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>9</kbd></span>
                      <span class="shortcut-desc">Danh sách Phím tắt</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>0</kbd></span>
                      <span class="shortcut-desc">Biến Môi trường</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>-</kbd> / <kbd>U</kbd></span>
                      <span class="shortcut-desc">Cập nhật Hệ thống</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>=</kbd> / <kbd>I</kbd></span>
                      <span class="shortcut-desc">Thông tin Hệ thống</span>
                  </div>
              </div>
          </div>
          
          <!-- In-page Actions Section -->
          <div class="shortcuts-section">
              <h4 class="section-title"><i class="pi pi-bolt" style="margin-right: 6px;"></i> Thao tác nhanh trong trang</h4>
              <div class="shortcuts-grid">
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>S</kbd></span>
                      <span class="shortcut-desc">Bật/Tắt công tắc nguồn (Wi-Fi / Bluetooth)</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>R</kbd></span>
                      <span class="shortcut-desc">Làm mới / Quét thiết bị (Wi-Fi / Bluetooth)</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Alt</kbd>+<kbd>N</kbd></span>
                      <span class="shortcut-desc">Tạo mới cấu hình (Trang VPN)</span>
                  </div>
              </div>
          </div>

          <!-- Global Controls Section -->
          <div class="shortcuts-section">
              <h4 class="section-title"><i class="pi pi-cog" style="margin-right: 6px;"></i> Phím tắt hệ thống</h4>
              <div class="shortcuts-grid">
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>?</kbd> / <kbd>Alt</kbd>+<kbd>H</kbd></span>
                      <span class="shortcut-desc">Mở / Đóng bảng tra cứu phím tắt này</span>
                  </div>
                  <div class="shortcut-item">
                      <span class="shortcut-keys"><kbd>Esc</kbd></span>
                      <span class="shortcut-desc">Đóng nhanh các hộp thoại đang mở</span>
                  </div>
              </div>
          </div>
      </div>
      <template #footer>
          <button class="btn-primary" @click="showShortcutsHelp = false" style="width: 100%;">Tôi đã rõ!</button>
      </template>
  </ModalDialog>
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

/* Beautiful Keycap & Shortcuts Layout */
.shortcuts-help-container {
    display: flex;
    flex-direction: column;
    gap: 20px;
    padding: 5px 0;
    text-align: left;
}

.shortcuts-section {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.section-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--accent-color);
    margin: 0;
    display: flex;
    align-items: center;
    opacity: 0.95;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    padding-bottom: 6px;
}

.shortcuts-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 8px;
}

@media (min-width: 480px) {
    .shortcuts-grid {
        grid-template-columns: 1fr 1fr;
        gap: 10px 24px;
    }
}

.shortcut-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12.5px;
    gap: 12px;
}

.shortcut-keys {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
}

.shortcut-desc {
    color: var(--text-secondary);
    opacity: 0.85;
    text-align: right;
    font-size: 12px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

/* Keycaps styling */
kbd {
    background-color: rgba(255, 255, 255, 0.07);
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 5px;
    padding: 3px 6px;
    font-family: inherit;
    font-size: 10.5px;
    font-weight: 700;
    color: var(--accent-color);
    box-shadow: 0 1.5px 0 rgba(0, 0, 0, 0.3);
    text-transform: uppercase;
    display: inline-block;
    line-height: 1;
}

/* Dim Visual Keycap Hint next to buttons */
.kbd-hint {
    margin-left: 8px;
    display: inline-flex;
    gap: 2px;
    opacity: 0.45;
    font-size: 10px;
    pointer-events: none;
    vertical-align: middle;
}

.kbd-hint kbd {
    background: rgba(255, 255, 255, 0.04) !important;
    border: 1px solid rgba(255, 255, 255, 0.1) !important;
    color: var(--text-secondary) !important;
    box-shadow: none !important;
    padding: 2px 4px !important;
    border-radius: 4px !important;
    font-size: 9px !important;
}

.btn-primary {
    background-color: var(--accent-color);
    color: white;
    font-weight: 600;
    border: none;
    padding: 10px 20px;
    border-radius: 8px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s;
}

.btn-primary:hover {
    opacity: 0.9;
    transform: translateY(-1px);
}
</style>
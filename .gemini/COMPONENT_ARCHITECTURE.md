# Component-Based Architecture - Display Management

## Tổng Quan Kiến Trúc Component

Hệ thống Display Management đã được refactor sang kiến trúc component-based để tăng khả năng tái sử dụng và bảo trì.

```
DisplaysView (Container)
├── DisplaysHeader (Presentational)
├── LoadingState (Presentational)
└── MonitorCard[] (Presentational)
```

## Cấu Trúc Thư Mục

```
src/
├── components/              # Các component tái sử dụng
│   ├── DisplaysHeader.vue   # Header với actions
│   ├── LoadingState.vue     # Loading indicator
│   └── MonitorCard.vue      # Card cấu hình monitor
├── models/                  # Data structures
│   └── display.model.ts
├── viewmodels/              # Business logic
│   └── display.viewmodel.ts
└── views/                   # Container components
    ├── DisplaysView.vue     # Main view (old - monolithic)
    └── DisplaysView_New.vue # New view (component-based)
```

## Chi Tiết Các Component

### 1. DisplaysHeader Component

**Mục đích**: Hiển thị tiêu đề và các action buttons

**Props**: Không có

**Events**:
- `@add-monitor` - Khi click nút "Fake"
- `@save` - Khi click nút "Save"

**Sử dụng**:
```vue
<DisplaysHeader 
    @add-monitor="addFakeMonitor"
    @save="saveSettings"
/>
```

**Trách nhiệm**:
- ✅ Hiển thị tiêu đề trang
- ✅ Hiển thị subtitle
- ✅ Render action buttons
- ✅ Emit events khi user tương tác
- ❌ KHÔNG chứa business logic

---

### 2. LoadingState Component

**Mục đích**: Hiển thị loading indicator

**Props**: Không có

**Events**: Không có

**Sử dụng**:
```vue
<LoadingState v-if="loading && monitors.length === 0" />
```

**Trách nhiệm**:
- ✅ Hiển thị spinner animation
- ✅ Hiển thị loading text
- ❌ KHÔNG chứa logic điều kiện hiển thị (do parent quyết định)

---

### 3. MonitorCard Component

**Mục đích**: Hiển thị và cho phép chỉnh sửa cấu hình của một monitor

**Props**:
```typescript
interface Props {
    monitor: Monitor;              // Dữ liệu monitor
    index: number;                 // Index trong danh sách
    isDragging: boolean;           // Đang được drag?
    isDragOver: boolean;           // Đang hover khi drag?
    availableMonitors: Monitor[];  // Monitors có thể mirror
}
```

**Events**:
```typescript
interface Emits {
    (e: 'dragstart', event: DragEvent, index: number): void;
    (e: 'dragenter', index: number): void;
    (e: 'dragover', event: DragEvent): void;
    (e: 'drop', event: DragEvent, index: number): void;
    (e: 'dragend'): void;
    (e: 'remove', index: number): void;
    (e: 'mirror-focus', monitor: Monitor): void;
}
```

**Sử dụng**:
```vue
<MonitorCard
    v-for="(monitor, index) in monitors"
    :key="monitor.id"
    :monitor="monitor"
    :index="index"
    :is-dragging="draggingCardIndex === index"
    :is-drag-over="dragOverIndex === index"
    :available-monitors="getAvailableMonitorsForMirror(monitor)"
    @dragstart="onCardDragStart"
    @dragenter="onCardDragEnter"
    @dragover="(e) => e.preventDefault()"
    @drop="onCardDrop"
    @dragend="onCardDragEnd"
    @remove="removeMonitor"
    @mirror-focus="onMirrorDropdownFocus"
/>
```

**Trách nhiệm**:
- ✅ Hiển thị thông tin monitor
- ✅ Render form inputs
- ✅ Handle drag events (emit to parent)
- ✅ Hiển thị enable/disable toggle
- ✅ Hiển thị display mode selector
- ✅ Hiển thị orientation selector
- ❌ KHÔNG chứa business logic phức tạp
- ❌ KHÔNG trực tiếp gọi backend

---

### 4. DisplaysView (Container Component)

**Mục đích**: Kết nối ViewModel với các presentational components

**Trách nhiệm**:
- ✅ Khởi tạo ViewModel
- ✅ Quản lý lifecycle (onMounted)
- ✅ Pass data xuống child components
- ✅ Handle events từ child components
- ✅ Orchestrate các components
- ❌ KHÔNG chứa business logic (delegate to ViewModel)
- ❌ KHÔNG render UI phức tạp (delegate to child components)

## Luồng Dữ Liệu

### 1. Khởi Tạo
```
DisplaysView
  └─> ViewModel.fetchMonitors()
       └─> Backend API
            └─> Update monitors state
                 └─> Re-render components
```

### 2. User Interaction
```
User clicks button in MonitorCard
  └─> Emit event to DisplaysView
       └─> Call ViewModel method
            └─> Update state
                 └─> Re-render affected components
```

### 3. Save Settings
```
User clicks Save in DisplaysHeader
  └─> Emit 'save' event
       └─> DisplaysView calls ViewModel.saveSettings()
            └─> Backend API
                 └─> Show success/error message
```

## Ưu Điểm Của Kiến Trúc Component-Based

### 1. **Tái Sử Dụng (Reusability)**
- Các component như `LoadingState` có thể dùng ở nhiều nơi
- `MonitorCard` có thể dùng trong các context khác nhau

### 2. **Dễ Test (Testability)**
- Test từng component độc lập
- Mock props và events dễ dàng
- Không cần test toàn bộ view

### 3. **Dễ Bảo Trì (Maintainability)**
- Mỗi component có trách nhiệm rõ ràng
- Thay đổi một component không ảnh hưởng các component khác
- Code ngắn gọn, dễ đọc

### 4. **Hiệu Suất (Performance)**
- Vue chỉ re-render component bị thay đổi
- Không re-render toàn bộ view

### 5. **Phát Triển Song Song (Parallel Development)**
- Team members có thể làm việc trên các component khác nhau
- Giảm conflict khi merge code

## So Sánh: Trước và Sau

### Trước (Monolithic)
```vue
<!-- DisplaysView.vue - 800+ lines -->
<template>
  <div>
    <!-- Header inline -->
    <div class="header">...</div>
    
    <!-- Loading inline -->
    <div v-if="loading">...</div>
    
    <!-- Monitor cards inline -->
    <div v-for="monitor in monitors">
      <!-- 200+ lines of form fields -->
    </div>
  </div>
</template>
```

**Vấn đề**:
- ❌ File quá dài (800+ lines)
- ❌ Khó tìm code
- ❌ Khó test
- ❌ Khó tái sử dụng

### Sau (Component-Based)
```vue
<!-- DisplaysView_New.vue - 95 lines -->
<template>
  <div>
    <DisplaysHeader @add-monitor="..." @save="..." />
    <LoadingState v-if="loading" />
    <MonitorCard v-for="..." :monitor="..." @remove="..." />
  </div>
</template>
```

**Ưu điểm**:
- ✅ File ngắn gọn (95 lines)
- ✅ Dễ đọc, dễ hiểu
- ✅ Dễ test từng phần
- ✅ Components có thể tái sử dụng

## Best Practices

### 1. **Props Down, Events Up**
- Parent truyền data xuống child qua props
- Child emit events lên parent
- Không modify props trực tiếp

### 2. **Single Responsibility**
- Mỗi component làm một việc
- Nếu component quá phức tạp → chia nhỏ

### 3. **Presentational vs Container**
- **Presentational**: Chỉ hiển thị UI, nhận props, emit events
- **Container**: Kết nối với ViewModel, quản lý state

### 4. **Naming Convention**
- Component names: PascalCase (`MonitorCard.vue`)
- Props: camelCase (`availableMonitors`)
- Events: kebab-case (`@mirror-focus`)

## Migration Guide

### Cách Chuyển Đổi

1. **Backup file cũ**:
   ```bash
   cp DisplaysView.vue DisplaysView_Old.vue
   ```

2. **Thay thế file mới**:
   ```bash
   mv DisplaysView_New.vue DisplaysView.vue
   ```

3. **Test kỹ lưỡng**:
   - Kiểm tra tất cả chức năng
   - Kiểm tra drag & drop
   - Kiểm tra save settings

4. **Xóa file cũ** (sau khi đã test xong):
   ```bash
   rm DisplaysView_Old.vue
   ```

## Tương Lai

### Các Cải Tiến Tiếp Theo

1. **Thêm Form Validation Component**
   - Validate input fields
   - Show error messages

2. **Thêm Confirmation Dialog Component**
   - Confirm trước khi remove monitor
   - Confirm trước khi save

3. **Thêm Toast Notification Component**
   - Thay thế `alert()` bằng toast đẹp hơn

4. **Thêm Keyboard Shortcuts**
   - Ctrl+S để save
   - Delete để remove selected monitor

5. **Thêm Undo/Redo**
   - Hoàn tác thay đổi
   - Làm lại thay đổi đã hoàn tác

## Tài Liệu Tham Khảo

- [Vue 3 Components](https://vuejs.org/guide/essentials/component-basics.html)
- [Component Communication](https://vuejs.org/guide/components/events.html)
- [Props Validation](https://vuejs.org/guide/components/props.html)
- [Component Design Patterns](https://www.patterns.dev/posts/presentational-container-pattern)

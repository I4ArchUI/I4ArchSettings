# MVVM Architecture - Display Management

This document explains the MVVM (Model-View-ViewModel) architecture implementation for the Display Management feature.

## Architecture Overview

```
┌─────────────────┐
│     View        │  DisplaysView.vue
│  (Presentation) │  - UI Rendering
│                 │  - User Interactions
└────────┬────────┘
         │
         │ Binds to
         ▼
┌─────────────────┐
│   ViewModel     │  display.viewmodel.ts
│ (Business Logic)│  - State Management
│                 │  - Business Rules
│                 │  - Data Transformation
└────────┬────────┘
         │
         │ Uses
         ▼
┌─────────────────┐
│     Model       │  display.model.ts
│   (Data Layer)  │  - Data Structures
│                 │  - Interfaces
│                 │  - Constants
└─────────────────┘
```

## Layer Responsibilities

### 1. Model Layer (`src/models/display.model.ts`)

**Purpose**: Define data structures and types

**Contains**:
- `Monitor` interface - Monitor data structure
- `ActiveWorkspace` interface - Workspace data structure
- `MonitorConfig` interface - Configuration structure
- `TransformOption` interface - Transform option structure
- `TRANSFORM_OPTIONS` constant - All 8 Hyprland transform values

**Responsibilities**:
- ✅ Define data types
- ✅ Define constants
- ❌ NO business logic
- ❌ NO UI logic

### 2. ViewModel Layer (`src/viewmodels/display.viewmodel.ts`)

**Purpose**: Handle all business logic and state management

**Contains**:
- State management (monitors, loading, drag state)
- Business methods:
  - `fetchMonitors()` - Load monitors from backend
  - `saveSettings()` - Save configuration
  - `addFakeMonitor()` - Add test monitor
  - `removeMonitor()` - Remove monitor
  - Drag & drop logic
  - Position calculations
  - Mirror mode logic

**Responsibilities**:
- ✅ Manage application state
- ✅ Implement business rules
- ✅ Handle data transformation
- ✅ Communicate with backend (Tauri API)
- ❌ NO UI rendering
- ❌ NO direct DOM manipulation

### 3. View Layer (`src/views/DisplaysView.vue`)

**Purpose**: Handle UI rendering and user interactions

**Contains**:
- Template (HTML structure)
- Styles (CSS)
- Minimal script (ViewModel initialization and binding)

**Responsibilities**:
- ✅ Render UI based on ViewModel state
- ✅ Capture user interactions
- ✅ Delegate actions to ViewModel
- ❌ NO business logic
- ❌ NO direct backend calls

## Benefits of MVVM

### 1. **Separation of Concerns**
- Each layer has a single, well-defined responsibility
- Easier to understand and maintain

### 2. **Testability**
- ViewModel can be unit tested independently
- No need for UI testing frameworks for business logic

### 3. **Reusability**
- ViewModel can be reused across different views
- Model can be shared across features

### 4. **Maintainability**
- Changes to business logic don't affect UI
- Changes to UI don't affect business logic
- Clear boundaries between layers

### 5. **Scalability**
- Easy to add new features
- Easy to refactor existing code
- Team members can work on different layers independently

## Usage Example

```typescript
// In DisplaysView.vue

// 1. Import ViewModel
import { DisplayViewModel } from '../viewmodels/display.viewmodel';

// 2. Initialize ViewModel
const viewModel = new DisplayViewModel();

// 3. Lifecycle hook
onMounted(async () => {
    await viewModel.fetchMonitors();
});

// 4. Expose to template
const { monitors, loading } = viewModel;
const { saveSettings, formatModelName } = viewModel;
```

```vue
<!-- In template -->
<template>
    <div v-if="loading">Loading...</div>
    <div v-else>
        <div v-for="monitor in monitors" :key="monitor.id">
            {{ formatModelName(monitor) }}
        </div>
        <button @click="saveSettings">Save</button>
    </div>
</template>
```

## File Structure

```
src/
├── models/
│   └── display.model.ts          # Data structures
├── viewmodels/
│   └── display.viewmodel.ts      # Business logic
└── views/
    └── DisplaysView.vue           # UI presentation
```

## Best Practices

1. **Keep View Thin**: Minimal logic in Vue component
2. **ViewModel Owns State**: All reactive state in ViewModel
3. **Model is Pure**: No logic, only types and constants
4. **Single Responsibility**: Each layer does one thing well
5. **Dependency Direction**: View → ViewModel → Model (one way)

## Migration Guide

### Before (Mixed Architecture)
```vue
<script setup>
// Everything mixed together
const monitors = ref([]);
const loading = ref(true);

const fetchMonitors = async () => {
    // Backend call
    // State update
    // Business logic
};

const saveSettings = async () => {
    // Validation
    // Backend call
    // Error handling
};
</script>
```

### After (MVVM)
```vue
<script setup>
// Clean separation
import { DisplayViewModel } from '../viewmodels/display.viewmodel';

const viewModel = new DisplayViewModel();
const { monitors, loading } = viewModel;
const { fetchMonitors, saveSettings } = viewModel;
</script>
```

## Future Enhancements

1. **Add Services Layer**: For complex backend interactions
2. **Add Repository Pattern**: For data persistence
3. **Add State Management**: Pinia/Vuex for global state
4. **Add Dependency Injection**: For better testability
5. **Add Composables**: For shared reactive logic

## References

- [Vue 3 Composition API](https://vuejs.org/guide/extras/composition-api-faq.html)
- [MVVM Pattern](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93viewmodel)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)

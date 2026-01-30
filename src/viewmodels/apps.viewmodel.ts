
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from '../composables/useToast';
import { ask } from '@tauri-apps/plugin-dialog';

export interface AppItem {
    name: string;
    icon: string;
    description: string;
    desktop_file: string;
    full_path: string;
}

export interface PackageItem {
    name: string;
    version: string;
}

export function useAppsViewModel() {
    const apps = ref<AppItem[]>([]);
    const packages = ref<PackageItem[]>([]);
    const loading = ref(false);
    const searchQuery = ref('');
    const activeTab = ref<'apps' | 'packages'>('apps');

    const { showToast } = useToast();

    const fetchApps = async () => {
        try {
            loading.value = true;
            apps.value = await invoke('get_installed_apps');
        } catch (e) {
            console.error(e);
            showToast('Failed to fetch apps', 'error');
        } finally {
            loading.value = false;
        }
    };

    const fetchPackages = async () => {
        try {
            loading.value = true;
            packages.value = await invoke('get_installed_packages');
        } catch (e) {
            console.error(e);
            showToast('Failed to fetch packages', 'error');
        } finally {
            loading.value = false;
        }
    };


    const uninstallApp = async (app: AppItem) => {
        const confirmed = await ask(`Are you sure you want to uninstall ${app.name}?`, {
            title: 'Confirm Uninstall',
            kind: 'warning'
        });

        if (!confirmed) return;

        try {
            await invoke('uninstall_app', { fullPath: app.full_path });
            showToast(`Uninstall process finished for ${app.name}`, 'success');
            // Refresh apps list
            await fetchApps();
        } catch (e) {
            showToast(`Failed to launch uninstall: ${e}`, 'error');
        }
    };

    const uninstallPackage = async (name: string) => {
        const confirmed = await ask(`Are you sure you want to uninstall package '${name}'?`, {
            title: 'Confirm Uninstall',
            kind: 'warning'
        });

        if (!confirmed) return;

        try {
            await invoke('uninstall_package', { name });
            showToast(`Uninstall process finished for ${name}`, 'success');
            // Refresh packages if on packages tab
            if (activeTab.value === 'packages') {
                await fetchPackages();
            }
        } catch (e) {
            showToast(`Failed to launch uninstall: ${e}`, 'error');
        }
    };

    const filteredApps = computed(() => {
        if (!searchQuery.value) return apps.value;
        const q = searchQuery.value.toLowerCase();
        return apps.value.filter(a =>
            a.name.toLowerCase().includes(q) ||
            a.description.toLowerCase().includes(q)
        );
    });

    const filteredPackages = computed(() => {
        if (!searchQuery.value) return packages.value;
        const q = searchQuery.value.toLowerCase();
        return packages.value.filter(p =>
            p.name.toLowerCase().includes(q)
        );
    });

    onMounted(() => {
        fetchApps();
        // Delay packages fetch a bit or fetch on tab switch because it might be heavy
    });

    const switchTab = (tab: 'apps' | 'packages') => {
        activeTab.value = tab;
        if (tab === 'packages' && packages.value.length === 0) {
            fetchPackages();
        }
    };

    return {
        apps,
        packages,
        loading,
        searchQuery,
        activeTab,
        filteredApps,
        filteredPackages,
        switchTab,
        uninstallApp,
        uninstallPackage,
        fetchApps,
        fetchPackages
    };
}

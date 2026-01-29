import { createRouter, createWebHistory } from 'vue-router';
import DisplaysView from '../views/DisplaysView.vue';
import WifiView from '../views/WifiView.vue';
import BluetoothView from '../views/BluetoothView.vue';
import VpnView from '../views/VpnView.vue';
import AboutView from '../views/AboutView.vue';
import AppearanceView from '../views/AppearanceView.vue';
import StartupView from '../views/StartupView.vue';
import KeybindsView from '../views/KeybindsView.vue';

import EnvView from '../views/EnvView.vue';

const routes = [
    { path: '/', redirect: '/about' },
    { path: '/about', name: 'About System', component: AboutView },
    { path: '/wifi', name: 'Wi-Fi', component: WifiView },
    { path: '/vpn', name: 'VPN', component: VpnView },
    { path: '/bluetooth', name: 'Bluetooth', component: BluetoothView },
    { path: '/appearance', name: 'Appearance', component: AppearanceView },
    { path: '/displays', name: 'Displays', component: DisplaysView },
    { path: '/env', name: 'Environment', component: EnvView },
    { path: '/startup', name: 'Startup Apps', component: StartupView },
    { path: '/shortcuts', name: 'Keybinds', component: KeybindsView },
];

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes,
});

export default router;

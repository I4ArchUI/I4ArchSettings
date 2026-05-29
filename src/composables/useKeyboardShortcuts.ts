import { onMounted, onUnmounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import { useToast } from './useToast';

export function useKeyboardShortcuts() {
    const router = useRouter();
    const { showToast } = useToast();
    const showShortcutsHelp = ref(false);

    const handleKeyDown = (e: KeyboardEvent) => {
        // Prevent triggering shortcuts when typing inside form input fields, selects or textareas
        const activeElement = document.activeElement;
        const isTyping = activeElement && (
            activeElement.tagName === 'INPUT' ||
            activeElement.tagName === 'TEXTAREA' ||
            activeElement.tagName === 'SELECT' ||
            activeElement.getAttribute('contenteditable') === 'true'
        );

        if (isTyping && e.key !== 'Escape') {
            return;
        }

        // Global keybinds
        
        // Escape closes any active modals or help sheet
        if (e.key === 'Escape') {
            if (showShortcutsHelp.value) {
                showShortcutsHelp.value = false;
                e.preventDefault();
            }
            return;
        }

        // Alt + H or "?" opens the help modal dialog
        if ((e.altKey && e.key.toLowerCase() === 'h') || (e.key === '?' && !isTyping)) {
            showShortcutsHelp.value = !showShortcutsHelp.value;
            e.preventDefault();
            return;
        }

        // Navigation & In-page action shortcuts
        if (e.altKey) {
            const currentPath = router.currentRoute.value.path;
            const keyLower = e.key.toLowerCase();
            
            // In-page actions
            if (keyLower === 's') {
                if (currentPath === '/wifi' || currentPath === '/bluetooth') {
                    window.dispatchEvent(new CustomEvent('shortcut-toggle'));
                    e.preventDefault();
                    return;
                }
            }
            if (keyLower === 'r') {
                if (currentPath === '/wifi' || currentPath === '/bluetooth') {
                    window.dispatchEvent(new CustomEvent('shortcut-refresh'));
                    e.preventDefault();
                    return;
                }
            }
            if (keyLower === 'n') {
                if (currentPath === '/vpn') {
                    window.dispatchEvent(new CustomEvent('shortcut-add'));
                    e.preventDefault();
                    return;
                }
            }

            let path = '';
            let pageName = '';
            
            switch (e.key) {
                // Number navigation (Alt + 1, Alt + 2...)
                case '1': path = '/wifi'; pageName = 'Wi-Fi'; break;
                case '2': path = '/vpn'; pageName = 'VPN'; break;
                case '3': path = '/bluetooth'; pageName = 'Bluetooth'; break;
                case '4': path = '/appearance'; pageName = 'Wallpaper'; break;
                case '5': path = '/themes'; pageName = 'Themes'; break;
                case '6': path = '/displays'; pageName = 'Displays'; break;
                case '7': path = '/apps'; pageName = 'Installed Apps'; break;
                case '8': path = '/startup'; pageName = 'Startup Apps'; break;
                case '9': path = '/shortcuts'; pageName = 'Keybinds'; break;
                case '0': path = '/env'; pageName = 'Environment'; break;
                case '-': path = '/system-update'; pageName = 'System Update'; break;
                case '=': path = '/about'; pageName = 'About System'; break;

                // Letter mnemonics (Alt + Key)
                case 'w': case 'W': path = '/wifi'; pageName = 'Wi-Fi'; break;
                case 'v': case 'V': path = '/vpn'; pageName = 'VPN'; break;
                case 'b': case 'B': path = '/bluetooth'; pageName = 'Bluetooth'; break;
                case 'p': case 'P': path = '/appearance'; pageName = 'Wallpaper'; break;
                case 't': case 'T': path = '/themes'; pageName = 'Themes'; break;
                case 'd': case 'D': path = '/displays'; pageName = 'Displays'; break;
                case 'a': case 'A': path = '/apps'; pageName = 'Installed Apps'; break;
                case 'u': case 'U': path = '/system-update'; pageName = 'System Update'; break;
                case 'i': case 'I': path = '/about'; pageName = 'About System'; break;
                
                default:
                    return;
            }

            if (path) {
                router.push(path);
                showToast(`Navigated to ${pageName}`, 'info');
                e.preventDefault();
            }
        }
    };

    onMounted(() => {
        window.addEventListener('keydown', handleKeyDown);
    });

    onUnmounted(() => {
        window.removeEventListener('keydown', handleKeyDown);
    });

    return {
        showShortcutsHelp
    };
}

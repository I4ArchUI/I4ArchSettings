import { onMounted, onUnmounted, ref } from 'vue';
import { useRouter } from 'vue-router';

export function useKeyboardShortcuts() {
    const router = useRouter();
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
            
            switch (e.key) {
                // Number navigation (Alt + 1, Alt + 2...)
                case '1': path = '/wifi'; break;
                case '2': path = '/vpn'; break;
                case '3': path = '/bluetooth'; break;
                case '4': path = '/appearance'; break;
                case '5': path = '/themes'; break;
                case '6': path = '/displays'; break;
                case '7': path = '/apps'; break;
                case '8': path = '/startup'; break;
                case '9': path = '/shortcuts'; break;
                case '0': path = '/env'; break;
                case '-': path = '/system-update'; break;
                case '=': path = '/about'; break;

                // Letter mnemonics (Alt + Key)
                case 'w': case 'W': path = '/wifi'; break;
                case 'v': case 'V': path = '/vpn'; break;
                case 'b': case 'B': path = '/bluetooth'; break;
                case 'p': case 'P': path = '/appearance'; break;
                case 't': case 'T': path = '/themes'; break;
                case 'd': case 'D': path = '/displays'; break;
                case 'a': case 'A': path = '/apps'; break;
                case 'u': case 'U': path = '/system-update'; break;
                case 'i': case 'I': path = '/about'; break;
                
                default:
                    return;
            }

            if (path) {
                router.push(path);
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

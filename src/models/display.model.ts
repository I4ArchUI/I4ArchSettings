/**
 * Display Model
 * Contains all data structures and types for display management
 */

export interface ActiveWorkspace {
    id: number;
    name: string;
}

export interface Monitor {
    id: number;
    name: string;
    model: string;
    width: number;
    height: number;
    refreshRate: number;
    x: number;
    y: number;
    scale: number;
    transform: number;
    focused: boolean;
    activeWorkspace: ActiveWorkspace;
    enabled: boolean;
    mirror: string | null;
}

export interface MonitorConfig {
    monitors: Monitor[];
}

export interface TransformOption {
    value: number;
    label: string;
    title: string;
    icon: string;
    iconStyle?: string;
}

export const TRANSFORM_OPTIONS: TransformOption[] = [
    { value: 0, label: 'Normal', title: 'Normal (0°)', icon: 'pi-desktop' },
    { value: 1, label: '90°', title: '90° Rotation', icon: 'pi-mobile', iconStyle: 'transform: rotate(90deg)' },
    { value: 2, label: '180°', title: '180° Rotation', icon: 'pi-desktop', iconStyle: 'transform: rotate(180deg)' },
    { value: 3, label: '270°', title: '270° Rotation', icon: 'pi-mobile', iconStyle: 'transform: rotate(-90deg)' },
    { value: 4, label: 'Flip', title: 'Flipped', icon: 'pi-desktop', iconStyle: 'transform: scaleX(-1)' },
    { value: 5, label: 'Flip+90°', title: 'Flipped + 90°', icon: 'pi-mobile', iconStyle: 'transform: rotate(90deg) scaleX(-1)' },
    { value: 6, label: 'Flip+180°', title: 'Flipped + 180°', icon: 'pi-desktop', iconStyle: 'transform: rotate(180deg) scaleX(-1)' },
    { value: 7, label: 'Flip+270°', title: 'Flipped + 270°', icon: 'pi-mobile', iconStyle: 'transform: rotate(-90deg) scaleX(-1)' }
];

# Hyprland Monitor Configuration Guide

## Overview
The Displays settings now generates Hyprland-compatible monitor configuration that can be used directly in your `hyprland.conf` file.

## Transform Values (Orientation)
The application supports all 8 Hyprland transform values:

| Value | Description | Visual Representation |
|-------|-------------|----------------------|
| 0 | Normal (no transform) | Standard landscape |
| 1 | 90° rotation | Portrait mode |
| 2 | 180° rotation | Upside down |
| 3 | 270° rotation | Portrait inverted |
| 4 | Flipped | Horizontal mirror |
| 5 | Flipped + 90° | Horizontal mirror + 90° |
| 6 | Flipped + 180° | Horizontal mirror + 180° |
| 7 | Flipped + 270° | Horizontal mirror + 270° |

## Generated Configuration Format

When you click "Save", the application generates a configuration file in the following format:

```
monitor=name,WIDTHxHEIGHT@REFRESHRATE,XxY,scale,transform,N
```

### Example Output:
```
monitor=DP-1,1920x1080@60.056,0x0,1,transform,0
monitor=HDMI-A-1,2560x1440@144,1920x0,1.5,transform,1
```

## File Location

The application saves monitor configuration directly to:

**Path**: `~/.config/hypr/configs/monitors.conf`

This file is written in Hyprland's native format and can be sourced directly in your main config.

## Performance Optimizations

To prevent lag during file generation:

1. **Atomic Write**: Uses a temporary file + rename operation (atomic on most filesystems)
   - Writes to `monitors.tmp` first
   - Renames to `monitors.conf` (instant operation)
   - Prevents corruption if write is interrupted
   - Eliminates file system lag

2. **Direct Generation**: Config is generated in Rust backend
   - No intermediate JSON serialization
   - Single write operation
   - Minimal memory allocation

3. **No Redundant Storage**: Only one file is maintained
   - Removed separate JSON storage
   - Hyprland config is the source of truth

## How to Use

1. **Configure your monitors** in the Displays settings:
   - Set resolution and refresh rate
   - Adjust position (X, Y coordinates)
   - Set scale factor
   - Choose orientation (transform value)

2. **Click "Save"** to write the configuration

3. **Add to Hyprland config**:
   ```conf
   # In your ~/.config/hypr/hyprland.conf
   source = ~/.config/hypr/configs/monitors.conf
   ```

4. **Reload Hyprland** to apply changes:
   ```bash
   hyprctl reload
   ```

## UI Features

### Orientation Selector
- Visual grid with 8 buttons (2 rows × 4 columns)
- Icons show the orientation visually
- Active selection highlighted in blue
- Hover effects for better UX
- Theme-aware (adapts to light/dark mode)

### Monitor Cards
- Drag and drop to reorder monitors
- Auto-calculates positions based on order
- Shows active/focused monitor
- All settings in one card

## Technical Details

### Position Calculation
- Positions are calculated based on scaled resolution
- Example: 4K monitor (3840×2160) with scale 2.0 = 1920×1080 logical size
- Next monitor's X position = previous monitor's X + logical width

### Transform in Hyprland
The `transform` parameter in Hyprland follows the Wayland transform specification:
- Applied after scaling
- Affects logical dimensions for positioning
- Rotated monitors swap width/height for layout calculations

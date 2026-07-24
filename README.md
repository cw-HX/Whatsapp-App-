# Whatsapp App

A Tauri desktop app for WhatsApp Web with a lightweight desktop experience.

## Features

- System tray icon with Open and Exit actions
- Single-instance behavior so a second launch focuses the existing window
- Download interception into a WhatsApp-specific folder in the user downloads directory
- Basic download history tracking
- Lightweight webview settings for reduced overhead while keeping GPU acceleration enabled

## Development

Install dependencies:

```bash
npm install
```

Start the app in development mode:

```bash
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

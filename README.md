# Whatsapp App

A desktop wrapper for WhatsApp Web built with Tauri. It provides a focused app experience with tray integration, single-instance behavior, and automatic downloads management.

## Overview

This project combines a lightweight webview frontend with a Rust backend powered by Tauri. The app opens WhatsApp Web in a native desktop window while adding convenient desktop behaviors such as tray controls, download interception, and a cleaner app lifecycle.

## Features

- System tray icon with Open and Exit actions
- Double-click tray icon to show, restore, and focus the main window
- Single-instance behavior so launching the app again focuses the existing window instead of opening a duplicate
- Download interception that saves files into a WhatsApp-specific folder under the user downloads directory
- Basic download history tracking for recently intercepted downloads
- Lightweight webview settings that reduce unnecessary overhead while keeping GPU acceleration enabled for smooth rendering

## Requirements

Before running the project, make sure you have:

- Node.js and npm
- Rust and Cargo
- A working Tauri development environment for your OS

## Installation

Install the frontend dependencies:

```bash
npm install
```

## Development

Start the app in development mode:

```bash
npm run tauri dev
```

This launches the Tauri app with the current source files so you can test changes live.

## Build

Create a production build:

```bash
npm run tauri build
```

## Project Structure

- src/: frontend files for the app UI
- src-tauri/: Rust backend, Tauri configuration, and app lifecycle logic
- src-tauri/src/lib.rs: tray integration, window behavior, download handling, and app setup
- src-tauri/tauri.conf.json: application metadata and desktop window configuration

## Notes

The app uses WhatsApp Web as its content source and adds desktop-oriented conveniences around it. If you want to extend it further, the main place to start is the Tauri backend in the src-tauri folder.

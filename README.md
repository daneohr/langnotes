# LangNotes: Language Learning Note-taking App with VIM Motions

> [!NOTE]
> Work in progress

Langnotes aims to solve a problem I have which is that I want access to language learning tools quickly while taking notes. Things like translators, word lists/trackers, etc. The purpose of this app is that it combines these tools in a blazing-fast editor written using the Tauri framework. 

## Why Tauri?
I chose Tauri because it's lighter and more memory-efficient than alternatives like Electron. It combines HTML, CSS, and JavaScript (or in this case, Svelte/SvelteKit with TypeScript) along with a Rust backend for a mix of development speed and memory safety.

The Electron framework bundles an entire Chromium browser along with Node.js to run an app. This size may be worth it for larger apps, but for smaller ones (and ones that want to avoid CORS via backend request routing), Tauri is usually the better option. It compiles the backend into native binaries and runs the frontend in a native webview client. 

The main tradeoff with Tauri is that each OS uses a differnet webview, meaning rendering can be slightly different. 
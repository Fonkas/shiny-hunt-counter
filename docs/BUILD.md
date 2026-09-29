# Notas de desenvolvimento

## Publicar uma nova versão

1. Faz as alterações (a app está toda em `src/index.html`).
2. Aumenta a versão em `src-tauri/tauri.conf.json` e em `package.json` (por exemplo `0.1.0` → `0.1.1`).
3. No GitHub: **Actions → Build Windows app → Run workflow**.
4. Ao fim de 10–15 minutos, a nova versão aparece em **Releases**.

## Compilar localmente (opcional)

Requisitos: Rust (https://rustup.rs), Node.js e os pré-requisitos do Tauri para Windows (https://v2.tauri.app/start/prerequisites/).

```bash
npm install
npm run dev     # modo de desenvolvimento
npm run build   # gera o .exe em src-tauri/target/release/
```

## Detalhes técnicos

- Tauri 2, com a interface em HTML/JS num único ficheiro (`src/index.html`).
- Hotkeys globais através de `tauri-plugin-global-shortcut`.
- Os dados ficam no armazenamento local do WebView2, na pasta de dados da app.

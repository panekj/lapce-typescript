# Lapce plugin for Typescript/Javascript (`typescript-language-server`)

## Prerequisites

Install `typescript-language-server` and `typescript` (Requires NodeJS) via `npm` or your system package manager

```shell
npm i --global typescript-language-server typescript
```

## Configuration

In `settings.toml`:

```toml
[lapce-typescript.volt]
serverPath = "<custom executable>"
serverArgs = ["--stdio"] # --stdio is required for all LSPs made in nodejs
```

# Lapce plugin for Typescript/Javascript (`typescript-language-server`)

## Prerequisites

Install `typescript-language-server` and `typescript` (Requires NodeJS) via `npm` or your system package manager  
Server needs to be in one of the paths included in `PATH` environment variable

```shell
npm i --global typescript-language-server@2 typescript
```

## Available configuration

```toml
[lapce-typescript.volt]
serverPath = "<custom executable>"
serverArgs = ["--stdio"] # --stdio is required for all LSPs written in nodejs
```

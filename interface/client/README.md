# Hardware Brainfuck Interpreter Interface Client

## Setup

### Prerequisites

- [NodeJS](https://nodejs.org/en) (for Linux, consider [nvm](https://github.com/nvm-sh/nvm))
- [pnpm](https://pnpm.io/) (when NodeJS is installed, run `corepack enable pnpm`)

### Install Dependencies

To install all required dependencies, run `pnpm install` in the projects root folder (`interface/client`)

## Development

To run a development server, which is configured to forward all `/api`-calls to `http://localhost:8000`, run `pnpm run dev`. The server configuration file is `interface/client/vite.config.ts`.

## Building

To build a production-ready website, run `pnpm run build`. The files will be saved under `interface/client/dist`.
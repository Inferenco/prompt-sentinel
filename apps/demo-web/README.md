# Prompt Sentinel Demo Web App

Standalone React + TypeScript frontend for demonstrating Prompt Sentinel runtime capabilities with web/mobile parity.

## Features

- Live API mode against Prompt Sentinel backend
- Deterministic mock mode for offline demo reliability
- Full workflow visibility:
  - firewall
  - bias
  - moderation
  - generation outcome
  - audit proof
- Raw request/response JSON inspector
- Request history (last 10)
- Responsive parity layout across mobile, tablet, and desktop

## Routes Used

- `GET /health`
- `GET /api/mistral/health`
- `POST /api/compliance/check`

## Run

```bash
cd apps/demo-web
npm install
npm run dev
```

Optional env:

```bash
VITE_API_BASE_URL=http://localhost:3000
```

## Test

```bash
npm run test
npm run test:e2e
```

Notes:

- E2E tests run in mock mode and do not require backend availability.
- Playwright browsers may need installation once:

```bash
npx playwright install
```

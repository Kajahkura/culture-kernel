# Culture Kernel 🌍🚀

> **The Operating System for High-Performance African Teams.**
> _Reverse-engineered from centuries of indigenous governance, recompiled for the digital age._

![License](https://img.shields.io/badge/license-MIT-green)
![Language](https://img.shields.io/badge/language-Rust-orange)
![Framework](https://img.shields.io/badge/framework-Axum-red)
![Database](https://img.shields.io/badge/database-Redb-blue)

---

## 📖 The Philosophy

Silicon Valley management protocols often fail in African contexts because they ignore the underlying "social operating system"—high power distance, communal trust mechanisms, and oral tradition.

**Culture Kernel** is a backend engine that replaces these default behaviors with **Engineered Rituals** derived from African history (Igbo, Hausa, Zulu, Akan, Rwanda, etc.). It serves these protocols via a high-performance API to install governance, accountability, and psychological safety into modern tech teams.

---

## ⚡ Key Engineering Features

* **🦀 100% Rust Architecture:** Built on `Axum` and `Tokio` for sub-millisecond latency and memory safety.
* **💾 Embedded ACID Database:** Uses `Redb` (Pure Rust). No external SQL servers (Postgres/MySQL) required. The database lives inside the binary.
* **🧠 Self-Healing Kernel:** The system automatically detects if the database is empty or corrupt and "re-seeds" itself with the core 21 African Protocols on startup.
* **Output Polymorphism (Dual-Mode API):**
    * **For Humans (CLI):** Detects `curl`/`wget` and renders a beautiful ASCII Terminal UI.
    * **For Machines (Web):** Returns rich, structured JSON for frontends.
* **🐳 Micro-Container:** Dockerized into a tiny, scratch-based image for instant deployment on Railway, Fly.io, or AWS.

---

## 🚀 Quick Start (Docker)

The fastest way to run the Kernel is via Docker.

### 1. Build the Image
```bash
docker build -t culture-kernel .
```

### 2. Run the Container
```bash
docker run -p 8080:8080 culture-kernel
```

The API is now live at http://localhost:8080/rituals.

---

## 🛠️ Local Development (Rust)

If you want to modify the source code:

### Prerequisites

* Rust & Cargo (v1.75+)

### Commands
```bash
# Install Dependencies
cargo build

# Run the Server (Auto-seeds DB)
cargo run -- serve --port 8080

# List Protocols via CLI (Internal Tool)
cargo run -- list

# Force Re-Seed Database
cargo run -- seed
```

---

## 🔌 API Documentation

The Kernel exposes a RESTful API for integration with dashboards, Slack bots, or internal tools.

### GET /rituals

Fetches the full library of active protocols.

#### Request (Terminal Mode)
```bash
curl https://your-deployment-url.app/rituals
```

**Response:** Returns a colored ASCII table of protocols (Hacker UI).

#### Request (Frontend Mode)
```javascript
fetch('https://your-deployment-url.app/rituals')
  .then(res => res.json())
  .then(data => console.log(data));
```

#### Response (JSON Structure):
```json
[
  {
    "protocol_id": "IGBO_01_IGBA_BOI",
    "name": "Venture Apprenticeship Settlement",
    "origin_culture": "Igbo (Nigeria)",
    "category": "Talent Development",
    "bug_fixed": "Principal-Agent Problem & IP Theft",
    "mechanism": "Deferred Capital Vesting",
    "modern_script": {
      "trigger": "Day 1 of High-Potential Hire",
      "contract": "Shadow Bonus (20%) + Match.",
      "vesting": "48 Months",
      "ritual": "Settlement Day Ceremonial Handover."
    },
    "ethical_guardrails": [
      "No Indentured Labor",
      "Clear Exit Terms"
    ]
  }
  // ... 20 more protocols
]
```

---

## 🏛️ The Protocol Library (Sneak Peek)

The Kernel comes pre-loaded with 21 protocols engineered from pan-African history:

| Protocol ID | Indigenous Origin | Solves Bug |
|-------------|-------------------|------------|
| Igba Boi | Igbo (Nigeria) | Employee Churn & Cap Table disputes |
| Imihigo | Rwanda | Weak OKRs & Lack of Accountability |
| Gandu | Hausa (Nigeria) | Moonlighting & Productivity Stagnation |
| Sankofa | Akan (Ghana) | Technical Debt & Repeating Mistakes |
| Pulaaku | Fulani (Sahel) | Panic during Server Outages |
| Ukusisa | Zulu (South Africa) | Resource Hoarding by Managers |

---

## 🌐 Live Frontend & API Demo

To see the Kernel in action immediately, you can access our reference implementation.

### 1. View the Live Dashboard

We have built a full **"Culture OS" Dashboard** that visualizes the JSON data. It features local storage (Arsenal), search, and detail views.

* **👉 [View Live App on BlinkHost](https://culture-kernel.blinkhost.me/ck)**

### 2. Test the Live API

You can build your own tools right now using our public production endpoint.

* **Endpoint:** `https://culture-kernel.up.railway.app/rituals`
* **Test it:**
```bash
  curl https://culture-kernel.up.railway.app/rituals
```

### 3. Host Your Own

We recommend BlinkHost for static frontend hosting.. You can fork our code and host your own customized version instantly on [BlinkHost](https://blink.blinkhost.me/home) (or any static host).

---

## 🚢 Deployment

This repo includes a production-ready Dockerfile.

### Deploy to Railway / Fly.io / Render

1. Push this repo to GitHub.
2. Connect your repo to Railway/Render.
3. The Dockerfile will automatically build the Rust binary.
4. No Environment Variables needed (The DB is embedded).
5. Your API will be live in ~2 minutes.

---

## 🤝 Contributing

We welcome pull requests that:

* **Add new Protocols:** Must include `origin_culture`, `bug_fixed`, and `ethical_guardrails`.
* **Optimize the Kernel:** Rust performance improvements.
* **Expand the API:** GraphQL support, Filtering, etc.

---

## 📜 License & Credits

* **Author:** Obed Inya
* **Contact:** obedkajah@gmail.com
* **License:** MIT Open Source License.

_"The logic of the ancestors, recompiled for the digital age."_
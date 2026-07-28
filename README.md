# Suffragio

## 📖 [**View the live documentation →**](https://suffragio.github.io/suffragio-spec/)

Suffragio is a proposal for a modern, open, and verifiable electoral system. It specifies an e-voting protocol built around blind-signature ballots, a public append-only vote log, and a decentralized I2P/Freenet network overlay, so that any citizen can independently verify an election's outcome without having to trust a single government or vendor.

This repository holds the specification: the [motivation & requirements](https://suffragio.github.io/suffragio-spec/motivation/), the [system architecture](https://suffragio.github.io/suffragio-spec/architecture/), the canonical [gRPC protocol definitions](proto/), and the rendered [Starlight](https://starlight.astro.build) documentation site itself.

[![Built with Starlight](https://astro.badg.es/v2/built-with-starlight/tiny.svg)](https://starlight.astro.build)

## 🧞 Commands

All commands are run from the root of the project, from a terminal:

| Command                   | Action                                           |
| :------------------------ | :----------------------------------------------- |
| `bun install`             | Installs dependencies                            |
| `bun dev`             | Starts local dev server at `localhost:4321`      |
| `bun build`           | Build your production site to `./dist/`          |
| `bun preview`         | Preview your build locally, before deploying     |
| `bun astro ...`       | Run CLI commands like `astro add`, `astro check` |
| `bun astro -- --help` | Get help using the Astro CLI                     |

## 🤝 Contributing

This is an early-stage proposal, and feedback is very welcome — please [open an issue](https://github.com/Suffragio/suffragio-spec/issues) with questions, objections, or suggestions about the protocol or the write-up.

**We're also looking for translators.** The docs currently ship in English and Polish (`src/content/docs/pl/`); help adding or improving translations into other languages is appreciated.

To submit a change:

1. Fork this repository and clone your fork.
2. Create a branch for your change (`git checkout -b my-change`).
3. Make your edits and commit them.
4. Push the branch to your fork (`git push origin my-change`).
5. Open a pull request against `Suffragio/suffragio-spec` on GitHub and describe what you changed and why.

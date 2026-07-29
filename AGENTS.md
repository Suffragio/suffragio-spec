## Development

When starting the dev server, use background mode:

```
astro dev --background
```

Manage the background server with `astro dev stop`, `astro dev status`, and `astro dev logs`.

## Suffragio specification (this repo)

When implementing or changing the electoral protocol/backend, treat these as source of truth:

1. **Normative behaviour:** `src/content/docs/protocol-v1.md` (PL: `src/content/docs/pl/protocol-v1.md`)
2. **Wire protocol:** `proto/suffragio/v1/*.proto`
3. **Human API tables:** `src/content/docs/api-reference.md`
4. **Architecture overview:** `src/content/docs/architecture.md`

If docs conflict, **protocol-v1.md wins** for v1 behaviour.

## Documentation (Astro/Starlight)

Full documentation: https://docs.astro.build

Consult these guides before working on related tasks:

- [Adding pages, dynamic routes, or middleware](https://docs.astro.build/en/guides/routing/)
- [Working with Astro components](https://docs.astro.build/en/basics/astro-components/)
- [Using React, Vue, Svelte, or other framework components](https://docs.astro.build/en/guides/framework-components/)
- [Adding or managing content](https://docs.astro.build/en/guides/content-collections/)
- [Adding styles or using Tailwind](https://docs.astro.build/en/guides/styling/)
- [Supporting multiple languages](https://docs.astro.build/en/guides/internationalization/)

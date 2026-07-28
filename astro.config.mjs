// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import mermaid from 'astro-mermaid';

const base = '/suffragio-spec';

// https://astro.build/config
export default defineConfig({
	site: 'https://suffragio.github.io',
	base,
	integrations: [
		mermaid(),
		starlight({
			title: 'My Docs',
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/Suffragio/suffragio-spec' }],
			head: [{ tag: 'script', attrs: { src: `${base}/mermaid-zoom.js`, defer: true } }],
			defaultLocale: 'root',
			locales: {
				root: { label: 'English', lang: 'en' },
				pl: { label: 'Polski', lang: 'pl' },
			},
			sidebar: [
				{
					label: 'Motivation & Requirements',
					translations: { pl: 'Motywacja i wymagania' },
					link: '/motivation/',
				},
				{
					label: 'System Architecture',
					translations: { pl: 'Architektura systemu' },
					link: '/architecture/',
				},
				{
					label: 'gRPC API Reference',
					translations: { pl: 'Specyfikacja API gRPC' },
					link: '/api-reference/',
				},
			],
		}),
	],
});

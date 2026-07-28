// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://suffragio.github.io',
	base: '/suffragio-spec',
	integrations: [
		starlight({
			title: 'My Docs',
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/withastro/starlight' }],
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
					label: 'Guides',
					translations: { pl: 'Przewodniki' },
					items: [
						// Each item here is one entry in the navigation menu.
						{ label: 'Example Guide', slug: 'guides/example' },
					],
				},
				{
					label: 'Reference',
					translations: { pl: 'Referencje' },
					items: [{ autogenerate: { directory: 'reference' } }],
				},
			],
		}),
	],
});

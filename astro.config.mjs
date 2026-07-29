// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import mermaid from 'astro-mermaid';
import md3Theme from 'starlight-theme-md3';

const base = '/suffragio-spec';

// https://astro.build/config
export default defineConfig({
	site: 'https://suffragio.github.io',
	base,
	integrations: [
		mermaid(),
		starlight({
			title: 'Suffragio Docs',
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/Suffragio/suffragio-spec' }],
			head: [{ tag: 'script', attrs: { src: `${base}/mermaid-zoom.js`, defer: true } }],
			customCss: ['./src/styles/custom.css'],
			plugins: [md3Theme()],
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
					label: 'Vote Anonymity',
					translations: { pl: 'Anonimowość głosu' },
					link: '/vote-anonymity/',
				},
				{
					label: 'Why Not Blockchain?',
					translations: { pl: 'Dlaczego nie blockchain?' },
					link: '/why-not-blockchain/',
				},
				{
					label: 'Comparing Anonymizing Networks',
					translations: { pl: 'Porównanie sieci anonimizujących' },
					link: '/network-comparison/',
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
				{
					label: 'License',
					translations: { pl: 'Licencja' },
					link: '/license/',
				},
				{
					label: 'Market Research',
					translations: { pl: 'Badanie rynku' },
					items: [
						{
							label: 'Summary',
							translations: { pl: 'Podsumowanie' },
							link: '/market-research/summary/',
						},
						{
							label: 'Findings',
							translations: { pl: 'Wyniki' },
							link: '/market-research/findings/',
						},
						{
							label: 'Comparison Matrix',
							translations: { pl: 'Macierz porównawcza' },
							link: '/market-research/matrix/',
						},
					],
				},
			],
		}),
	],
});

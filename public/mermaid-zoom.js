/**
 * Adds a click-to-enlarge overlay for Mermaid diagrams rendered by astro-mermaid.
 * No external dependencies; pure DOM/CSS.
 */
(function () {
	function ensureOverlay() {
		let overlay = document.getElementById('mermaid-zoom-overlay');
		if (overlay) return overlay;

		overlay = document.createElement('div');
		overlay.id = 'mermaid-zoom-overlay';
		overlay.innerHTML = '<button type="button" id="mermaid-zoom-close" aria-label="Close">&times;</button><div id="mermaid-zoom-content"></div>';
		document.body.appendChild(overlay);

		const style = document.createElement('style');
		style.textContent = `
			#mermaid-zoom-overlay {
				position: fixed;
				inset: 0;
				background: rgba(0, 0, 0, 0.85);
				display: none;
				align-items: center;
				justify-content: center;
				z-index: 9999;
				cursor: zoom-out;
				overflow: auto;
				padding: 2rem;
				box-sizing: border-box;
			}
			#mermaid-zoom-overlay.mermaid-zoom-open {
				display: flex;
			}
			#mermaid-zoom-content {
				max-width: none;
				cursor: default;
			}
			#mermaid-zoom-content svg {
				width: auto;
				height: auto;
				min-width: min(90vw, 1200px);
				max-width: none;
				background: var(--sl-color-bg, #fff);
				border-radius: 0.5rem;
			}
			#mermaid-zoom-close {
				position: fixed;
				top: 1rem;
				right: 1.5rem;
				background: transparent;
				border: none;
				color: #fff;
				font-size: 2.5rem;
				line-height: 1;
				cursor: pointer;
				z-index: 10000;
			}
			pre.mermaid[data-processed] svg {
				cursor: zoom-in;
			}
		`;
		document.head.appendChild(style);

		function close() {
			overlay.classList.remove('mermaid-zoom-open');
			overlay.querySelector('#mermaid-zoom-content').innerHTML = '';
		}

		overlay.addEventListener('click', (e) => {
			if (e.target === overlay) close();
		});
		document.getElementById('mermaid-zoom-close').addEventListener('click', close);
		document.addEventListener('keydown', (e) => {
			if (e.key === 'Escape') close();
		});

		return overlay;
	}

	function openZoom(svg) {
		const overlay = ensureOverlay();
		const content = overlay.querySelector('#mermaid-zoom-content');
		content.innerHTML = '';
		content.appendChild(svg.cloneNode(true));
		overlay.classList.add('mermaid-zoom-open');
	}

	function attachHandlers() {
		document.querySelectorAll('pre.mermaid[data-processed] svg:not([data-zoom-bound])').forEach((svg) => {
			svg.setAttribute('data-zoom-bound', 'true');
			svg.setAttribute('tabindex', '0');
			svg.setAttribute('role', 'button');
			svg.setAttribute('aria-label', 'Click to enlarge diagram');
			svg.addEventListener('click', () => openZoom(svg));
			svg.addEventListener('keydown', (e) => {
				if (e.key === 'Enter' || e.key === ' ') {
					e.preventDefault();
					openZoom(svg);
				}
			});
		});
	}

	function watch() {
		attachHandlers();
		const observer = new MutationObserver(() => attachHandlers());
		observer.observe(document.body, { childList: true, subtree: true, attributes: true });
	}

	document.addEventListener('DOMContentLoaded', watch);
	document.addEventListener('astro:page-load', watch);
})();

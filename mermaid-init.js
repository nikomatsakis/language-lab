// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

(() => {
    const darkThemes = ['ayu', 'navy', 'coal'];
    const lightThemes = ['light', 'rust'];

    const classList = document.getElementsByTagName('html')[0].classList;

    let lastThemeWasLight = true;
    for (const cssClass of classList) {
        if (darkThemes.includes(cssClass)) {
            lastThemeWasLight = false;
            break;
        }
    }

    const theme = lastThemeWasLight ? 'default' : 'dark';
    mermaid.initialize({ startOnLoad: false, theme });

    window.addEventListener('load', async () => {
        try {
            await mermaid.run();
        } catch (error) {
            console.error('Mermaid diagram rendering failed', error);
        }

        document.querySelectorAll('pre.mermaid > svg').forEach(makeExpandable);
    });

    function makeExpandable(svg) {
        const button = document.createElement('button');
        button.type = 'button';
        button.className = 'mermaid-expand';
        button.setAttribute('aria-haspopup', 'dialog');
        button.setAttribute('aria-label', 'Expand diagram');
        button.title = 'Expand diagram';
        svg.before(button);
        button.append(svg);

        const dialog = document.createElement('dialog');
        dialog.className = 'mermaid-dialog';
        dialog.setAttribute('aria-label', 'Expanded diagram');
        const close = document.createElement('button');
        close.type = 'button';
        close.textContent = 'Close diagram';
        close.autofocus = true;
        const viewport = document.createElement('div');
        viewport.className = 'mermaid-dialog-viewport';
        const { width } = svg.viewBox.baseVal;
        if (width > 0) {
            viewport.style.setProperty('--diagram-width', `${width}px`);
        }
        dialog.append(close, viewport);
        document.body.append(dialog);

        button.addEventListener('click', () => {
            button.style.minHeight = `${button.offsetHeight}px`;
            viewport.append(svg);
            dialog.showModal();
            viewport.scrollTo(0, 0);
        });
        close.addEventListener('click', () => dialog.close());
        dialog.addEventListener('close', () => {
            button.append(svg);
            button.style.minHeight = '';
            button.focus();
        });
    }

    // Simplest way to make mermaid re-render the diagrams in the new theme is via refreshing the page

    for (const darkTheme of darkThemes) {
        document.getElementById('mdbook-theme-' + darkTheme).addEventListener('click', () => {
            if (lastThemeWasLight) {
                window.location.reload();
            }
        });
    }

    for (const lightTheme of lightThemes) {
        document.getElementById('mdbook-theme-' + lightTheme).addEventListener('click', () => {
            if (!lastThemeWasLight) {
                window.location.reload();
            }
        });
    }
})();

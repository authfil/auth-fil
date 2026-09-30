function openZoom(svg) {
  const overlay = document.createElement('div');
  overlay.className = 'mermaid-zoom-overlay';
  overlay.appendChild(svg.cloneNode(true));
  overlay.addEventListener('click', () => overlay.remove());
  document.addEventListener(
    'keydown',
    function onKey(event) {
      if (event.key === 'Escape') {
        overlay.remove();
        document.removeEventListener('keydown', onKey);
      }
    },
  );
  document.body.appendChild(overlay);
}

function attachZoomHandlers() {
  document.querySelectorAll('pre.mermaid[data-processed] svg').forEach((svg) => {
    if (svg.dataset.zoomAttached) return;
    svg.dataset.zoomAttached = 'true';
    svg.closest('pre.mermaid').addEventListener('click', () => openZoom(svg));
  });
}

function watchForMermaidDiagrams() {
  attachZoomHandlers();
  const observer = new MutationObserver(attachZoomHandlers);
  observer.observe(document.body, {
    childList: true,
    subtree: true,
    attributes: true,
    attributeFilter: ['data-processed'],
  });
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', watchForMermaidDiagrams);
} else {
  watchForMermaidDiagrams();
}

document.addEventListener('astro:after-swap', attachZoomHandlers);
document.addEventListener('astro:page-load', attachZoomHandlers);

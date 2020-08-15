var CACHE_NAME = "titso-0.0.0";
var APP_SHELL_FILES = [
	"/titso/",
	"/titso/index.html",
	"/titso/pkg/titso_web.js",
	"/titso/pkg/titso_web_bg.js",
	"/titso/pkg/titso_web_bg.wasm",
	"/titso/css/bulma.min.css"
];

self.addEventListener('install', function(e) {
	console.log('[Service Worker] Install');
	e.waitUntil(
		caches.open(cacheName).then(function(cache) {
			console.log('[Service Worker] Caching all: app shell and content');
			return cache.addAll(contentToCache);
		})
	);
});

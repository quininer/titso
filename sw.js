var CACHE_NAME = "titso-0.0.6";
var APP_SHELL_FILES = [
	"/titso/",
	"/titso/index.html",
	"/titso/titso.svg",
	"/titso/titso.png",
	"/titso/pkg/titso_web.js",
	"/titso/pkg/titso_web_bg.wasm",
	"/titso/css/bulma.min.css"
];

self.addEventListener('install', function(e) {
	console.log('[Service Worker] Install');
	e.waitUntil(
		caches.open(CACHE_NAME).then(function(cache) {
			console.log('[Service Worker] Caching all: app shell and content');
			return cache.addAll(APP_SHELL_FILES);
		})
	);
});

self.addEventListener('activate', (e) => {
	e.waitUntil(
		caches.keys().then((keyList) => {
			return Promise.all(keyList.map((key) => {
				if(key !== CACHE_NAME) {
					return caches.delete(key);
				}
			}));
		})
	);
});

self.addEventListener('fetch', (e) => {
	e.respondWith(
		caches.match(e.request).then((r) => {
			console.log('[Service Worker] Fetching resource: '+e.request.url);
			return r || fetch(e.request).then((response) => {
				return caches.open(CACHE_NAME).then((cache) => {
					console.log('[Service Worker] Caching new resource: '+e.request.url);
					cache.put(e.request, response.clone());
					return response;
				});
			});
		})
	);
});

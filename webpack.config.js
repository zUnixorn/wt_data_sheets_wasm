const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const workboxPlugin = require('workbox-webpack-plugin');
const {GenerateSW} = require("workbox-webpack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
	performance: {
		hints: false,
		maxEntrypointSize: 512000,
		maxAssetSize: 512000
	},
	experiments: {
		asyncWebAssembly: true,
		syncWebAssembly: true
	},
	mode: "production",
	entry: {
		index: "./js/index.js"
	},
	output: {
		path: dist,
		filename: "[name].js",
		publicPath: ''
	},
	devServer: {
		static: {
			directory: path.join(__dirname, "dist")
		},
		host: 'localhost',
		compress: true,
		port: 8081,
	},
	plugins: [
		new CopyPlugin([
			{from: path.resolve(__dirname, "static/manifest.json"), to: ''},
			{from: path.resolve(__dirname, "static/metafiles"), to: ''},
			{from: path.resolve(__dirname, "static/css"), to: ''},
			{from: path.resolve(__dirname, "static/html"), to: ''},
			{from: path.resolve(__dirname, "static/roboto_mono"), to: ''},
			{from: path.resolve(__dirname, "node_modules/mathjax"), to: 'mathjax'},
			{from: path.resolve(__dirname, "static/js"), to: 'js'},
			{from: path.resolve(__dirname, "static/robots.txt"), to: ''},
			{from: path.resolve(__dirname, "static/img/"), to: 'img'},
		]),

		new WasmPackPlugin({
			crateDirectory: __dirname,
		}),
		new workboxPlugin.GenerateSW({
			swDest: 'sw.js',
			clientsClaim: true,
			cleanupOutdatedCaches: true,
			runtimeCaching: [
				{
					urlPattern: /\.(?:html|css|js|wasm)$/,
					handler: 'NetworkFirst',
					options: {
						cacheName: 'short_term',
						expiration: {
							// caches no more than 1 hour
							maxAgeSeconds: 60 * 60,
							purgeOnQuotaError: true,
						},
					},
				},
				{
					urlPattern: /\.(?:svg|ico|png|ttf)$/,
					handler: 'StaleWhileRevalidate',
					options: {
						cacheName: 'long_term',
						expiration: {
							// caches no more than 1 day
							maxAgeSeconds: 60 * 60 * 24,
							purgeOnQuotaError: true,
						},
					},
				}
			],
		}),
	]
};

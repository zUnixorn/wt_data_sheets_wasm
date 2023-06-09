- [X] Remove webpack and NPM toolchain
- [X] Rewrite CI
    - [X] Install trunk
    - [X] Remove NPM
- [ ] Replace all JS glue with Yew functions
- [ ] Workbox alternative/bindings
- [ ] Mathjax replacement or integration
- [ ] Handle remaining static files:
 ```js
  {from: path.resolve(__dirname, "static/manifest.json"), to: ''},
  {from: path.resolve(__dirname, "static/metafiles"), to: ''},
  {from: path.resolve(__dirname, "static/css"), to: ''},
  {from: path.resolve(__dirname, "static/html"), to: ''},
  {from: path.resolve(__dirname, "static/font/roboto_mono"), to: ''},
  // {from: path.resolve(__dirname, "node_modules/mathjax"), to: 'mathjax'},
  {from: path.resolve(__dirname, "static/js"), to: 'js'},
  {from: path.resolve(__dirname, "static/robots.txt"), to: ''},
  {from: path.resolve(__dirname, "static/img/"), to: 'img'},
````
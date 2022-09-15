/* eslint-disable @typescript-eslint/no-require-imports */
const path = require('path')
const { defineConfig } = require('vite')

const name = 'transaction-library'

module.exports = defineConfig({
  build: {
    lib: {
      name,
      entry: path.resolve(__dirname, `src/${name}.ts`),
      fileName: (format) => `${name}.${format}.js`,
    },
  },
})

import { defineNuxtConfig } from 'nuxt/config'
import { Static } from 'vue'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false,
  spaLoadingTemplate: 'spa-loading-template.html',
  devtools: { enabled: true },
  srcDir: "./src/client",
  buildDir: "./src/client/.nuxt",
  nitro: {
    preset: "static",
    output: {
      dir: './dist/client',
      serverDir: './dist/client/server',
      publicDir: './dist/client/public'
    }
  }
})

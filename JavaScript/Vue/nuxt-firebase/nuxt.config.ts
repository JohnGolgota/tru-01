// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    devtools: { enabled: true },
    css: ["~/assets/css/main.css"],
    postcss: {
        plugins: {
            tailwindcss: {},
            autoprefixer: {},
        },
    },
    runtimeConfig: {
        public: {
            apiKey: process.env.NUXT_FIREBASE_API_KEY,
            authDomain: process.env.NUXT_FIREBASE_AUTH_DOMAIN,
            projectId: process.env.NUXT_FIREBASE_PROJECT_ID,
            storageBucket: process.env.NUXT_FIREBASE_STORAGE_BUCKET,
            messagingSenderId: process.env.NUXT_FIREBASE_MESSAGING_SENDER_ID,
            appId: process.env.NUXT_FIREBASE_APP_ID
        }
    }
});

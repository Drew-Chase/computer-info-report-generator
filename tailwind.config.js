import {heroui} from "@heroui/react";

/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
        "./node_modules/@heroui/theme/dist/**/*.{js,ts,jsx,tsx}"
    ],
    theme: {
        extend: {
            keyframes: {
                fadeIn: {
                    "0%": {opacity: "0"},
                    "100%": {opacity: "1"},
                },
            },
            animation: {
                "fade-in": "fadeIn 0.4s ease-out",
            },
        },
    },
    darkMode: "class",
    plugins: [heroui({
        themes: {
            dark: {
                colors: {
                    primary: "#60a5fa",
                    secondary: "#e5e5e5",
                    background: "#111111",
                    content1: "#1a1a1a",
                    content2: "#1f1f1f",
                    content3: "#242424",
                    content4: "#2a2a2a",
                }
            },
        }
    })]
}

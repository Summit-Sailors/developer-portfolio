/** @type {import('tailwindcss').Config} */
const shadcnConfig = require("./node_modules/ui/tailwind.config.js")

module.exports = {
  darkMode: ["class"], // Or 'media' if you prefer
  content: [
    "./src/**/*.{rs,html}", // Scan Rust files and index.html for classes
    "./index.html",
    // Add paths to any other files that might contain Tailwind classes
    ...shadcnConfig.content,
      "*.{js,ts,jsx,tsx,mdx}"
],
  theme: {
    extend: {
      ...shadcnConfig.theme.extend,
      fontFamily: {
        sans: ["Inter", "sans-serif"], // Ensure Inter is primary sans-serif
      },
      colors: {
        // Using your preferred color palette with HSL for v3
        purple: {
          400: "hsl(276, 70%, 70%)",
          500: "hsl(276, 70%, 60%)",
          600: "hsl(276, 70%, 50%)",
          700: "hsl(276, 70%, 40%)",
        },
        cyan: {
          400: "hsl(196, 70%, 70%)",
          500: "hsl(196, 70%, 60%)",
        },
        pink: {
          500: "hsl(336, 70%, 60%)",
        },
        zinc: {
          300: "hsl(240, 5%, 80%)",
          400: "hsl(240, 5%, 65%)",
          700: "hsl(240, 5%, 35%)",
          800: "hsl(240, 5%, 25%)",
          900: "hsl(240, 5%, 15%)",
        },
        yellow: {
          500: "hsl(50, 90%, 60%)",
        },
        blue: {
          500: "hsl(220, 80%, 60%)",
        },
        green: {
          500: "hsl(145, 60%, 50%)",
          600: "hsl(145, 60%, 40%)",
        },
        orange: {
          500: "hsl(30, 90%, 60%)",
          600: "hsl(25, 90%, 55%)",
        },
        red: {
          500: "hsl(0, 80%, 60%)",
        },
        // Removed shadcn/ui specific color variables
        ...shadcnConfig.theme.extend.colors,
      },
      borderRadius: {
        lg: "0.5rem", // Example, adjust as needed
        md: "0.375rem",
        sm: "0.25rem",
        ...shadcnConfig.theme.extend.borderRadius,
      },
      animation: {
        blob: "blob 7s infinite",
        "fade-in-up": "fadeInUp 0.5s ease-out forwards",
        "fade-in-scale": "fadeInScale 0.5s ease-out 0.5s forwards",
        "slide-in-left": "slideInLeft 0.5s ease-out forwards",
        "slide-in-right": "slideInRight 0.5s ease-out forwards",
        "pulse-slow": "pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite",
        ...shadcnConfig.theme.extend.animation,
      },
      keyframes: {
        blob: {
          "0%": { transform: "translate(0px, 0px) scale(1)" },
          "33%": { transform: "translate(30px, -50px) scale(1.1)" },
          "66%": { transform: "translate(-20px, 20px) scale(0.9)" },
          "100%": { transform: "translate(0px, 0px) scale(1)" },
        },
        fadeInUp: {
          from: { opacity: "0", transform: "translateY(20px)" },
          to: { opacity: "1", transform: "translateY(0)" },
        },
        fadeInScale: {
          from: { opacity: "0", transform: "scale(0.8)" },
          to: { opacity: "1", transform: "scale(1)" },
        },
        slideInLeft: {
          from: { opacity: "0", transform: "translateX(-30px)" },
          to: { opacity: "1", transform: "translateX(0)" },
        },
        slideInRight: {
          from: { opacity: "0", transform: "translateX(30px)" },
          to: { opacity: "1", transform: "translateX(0)" },
        },
        ...shadcnConfig.theme.extend.keyframes,
      },
    },
  },
  plugins: [require("tailwindcss-animate"), ...shadcnConfig.plugins], // No shadcn/ui plugins
}

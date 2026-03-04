/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        adwaita: {
          bg: '#ffffff',
          fg: '#1e1e1e',
          accent: '#3584e4',
          success: '#26a269',
          warning: '#e5a50a',
          error: '#c01c28',
          muted: '#f0f0f0',
        }
      }
    },
  },
  plugins: [],
}

/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        display: ['"IBM Plex Mono"', 'monospace'],
        body: ['"Space Grotesk"', 'sans-serif'],
      },
      colors: {
        // Technical / Industrial palette - light mode
        'upakor-bg': '#FAFAF8',        // Warm off-white
        'upakor-bg-alt': '#F0F0EE',    // Slightly darker for cards
        'upakor-fg': '#0D0D0D',        // Rich black
        'upakor-fg-muted': '#4A4A4A',  // Muted grey
        'upakor-border': '#E5E5E3',    // Subtle border
        'upakor-accent': '#0D9488',    // Technical teal
        'upakor-accent-hover': '#0F766E',
        'upakor-warning': '#EAB308',   // Technical amber
        'upakor-danger': '#DC2626',    // Technical red
      },
      boxShadow: {
        'technical': '1px 1px 0px 0px rgba(13, 13, 13, 0.1)',
        'technical-hover': '2px 2px 0px 0px rgba(13, 13, 13, 0.15)',
        'technical-lg': '3px 3px 0px 0px rgba(13, 13, 13, 0.08)',
      },
      animation: {
        'fade-in': 'fadeIn 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'slide-in': 'slideIn 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'scale-in': 'scaleIn 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideIn: {
          '0%': { opacity: '0', transform: 'translateY(-8px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        scaleIn: {
          '0%': { opacity: '0', transform: 'scale(0.95)' },
          '100%': { opacity: '1', transform: 'scale(1)' },
        },
      },
      transitionTimingFunction: {
        'snappy': 'cubic-bezier(0.16, 1, 0.3, 1)',
      },
    },
  },
  plugins: [],
}

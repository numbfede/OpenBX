/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        canvas: "rgb(var(--canvas) / <alpha-value>)",
        ink: "rgb(var(--ink) / <alpha-value>)",
        cream: "rgb(var(--cream) / <alpha-value>)",
        ready: "rgb(var(--ready) / <alpha-value>)",
        warn: "rgb(var(--warn) / <alpha-value>)",
        danger: "rgb(var(--danger) / <alpha-value>)",
      },
      fontFamily: {
        sans: ["Geist Variable", "Geist", "Segoe UI", "sans-serif"],
      },
      borderRadius: {
        card: "20px",
        btn: "14px",
      },
      boxShadow: {
        glass: "0 1px 0 0 rgb(255 255 255 / 0.06) inset, 0 20px 40px rgb(0 0 0 / 0.28)",
      },
      transitionTimingFunction: {
        openbx: "cubic-bezier(0.22, 1, 0.36, 1)",
      },
    },
  },
  plugins: [],
};

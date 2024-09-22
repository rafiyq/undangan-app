/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html"],
  theme: {
    container: {
      padding: {
        DEFAULT: '1rem',
        sm: '2rem',
        lg: '4rem',
        xl: '6rem',
        '2xl': '8rem',
      },
    },
    extend: {},
  },
  plugins: [],
}


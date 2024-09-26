/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html"],
  theme: {
    extend: {
      boxShadow: {
        'nm-flat': '0.2em 0.2em calc(0.2em * 2) #d6d3c8, calc(0.2em * -1) calc(0.2em * -1) calc(0.2em * 2) #ffffff',
        'nm-pressed': 'inset 0.2em 0.2em calc(0.2em * 2) #d6d3c8, inset calc(0.2em * -1) calc(0.2em * -1) calc(0.2em * 2) #ffffff'
      },
      colors: {
        'colour1': '#F9F5E9',
        'colour2': '#BF814B',
        'colour3': '#735F46',
        'colour4': '#9AA68F',
        'colour5': '#BFA65A',
        'colour6': '#BF8654',
      },
    },
  },
  plugins: [],
}

